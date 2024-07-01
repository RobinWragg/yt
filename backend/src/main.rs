use curl;
use regex::Regex;
use serde_json::Value as JsonValue;
use sqlx::{postgres::PgConnection, Connection};
use std::{error::Error, time::*};

type DateTime = chrono::DateTime<chrono::Utc>;

fn parse_recency(s: &str, now: &DateTime) -> Result<DateTime, Box<dyn Error>> {
    let re = Regex::new(r"(\d+) (\w+?)s? ago")?;
    let captures = re.captures(s).ok_or("captures() call failed")?;

    let base_number = captures[1].parse::<u64>()?;

    const SECONDS_PER_MINUTE: u64 = 60;
    const SECONDS_PER_HOUR: u64 = SECONDS_PER_MINUTE * 60;
    const SECONDS_PER_DAY: u64 = SECONDS_PER_HOUR * 24;
    const SECONDS_PER_WEEK: u64 = SECONDS_PER_DAY * 7;
    const SECONDS_PER_MONTH: u64 = SECONDS_PER_DAY * 30; // Approximate!
    const SECONDS_PER_YEAR: u64 = SECONDS_PER_DAY * 365; // Approximate!

    let seconds = match &captures[2] {
        "second" => base_number,
        "minute" => base_number * SECONDS_PER_MINUTE,
        "hour" => base_number * SECONDS_PER_HOUR,
        "day" => base_number * SECONDS_PER_DAY,
        "week" => base_number * SECONDS_PER_WEEK,
        "month" => base_number * SECONDS_PER_MONTH,
        "year" => base_number * SECONDS_PER_YEAR,
        _ => unreachable!(),
    };

    Ok(*now - Duration::from_secs(seconds))
}

fn video_list_url(channel_id: &str) -> String {
    "https://www.youtube.com/@".to_owned() + channel_id + "/videos"
}

fn json_from_page(page: &str) -> Option<String> {
    let re = Regex::new(r"var ytInitialData = (.*?);</script>").ok()?;
    Some(re.captures(page)?.get(1)?.as_str().to_string())
}

#[derive(Clone)]
struct VideoDetails {
    video_id: String,
    channel_id: String,
    title: String,
    date: DateTime,
}

fn get_video(
    channel_id: &str,
    value: &JsonValue,
    now: &DateTime,
) -> Result<VideoDetails, Box<dyn Error>> {
    let recency = value["publishedTimeText"]["simpleText"]
        .as_str()
        .ok_or("Couldn't get simpleText str")?
        .to_string();
    let date = parse_recency(&recency, now)?;
    let video_id = value["videoId"]
        .as_str()
        .ok_or("Couldn't get videoId str")?
        .to_string();
    let title = value["title"]["runs"][0]["text"]
        .as_str()
        .ok_or("Couldn't get runs str")?
        .to_string();

    return Ok(VideoDetails {
        channel_id: channel_id.to_string(),
        video_id,
        title,
        date,
    });
}

fn get_channel_videos(
    channel_id: &str,
    now: &DateTime,
) -> Result<Vec<VideoDetails>, Box<dyn Error>> {
    let url = video_list_url(channel_id);
    let mut easy_curl = curl::easy::Easy::new();
    easy_curl.follow_location(true)?;
    let mut dst = Vec::new();
    {
        easy_curl.url(&url)?;
        let mut transfer = easy_curl.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?; // note, this failed once.
    }
    assert_eq!(easy_curl.response_code(), Ok(200));
    let page = std::str::from_utf8(&dst)?;

    if let Some(json) = json_from_page(page) {
        let serde_value: JsonValue = serde_json::from_str(&json)?;
        let tabs = &serde_value["contents"]["twoColumnBrowseResultsRenderer"]["tabs"];
        let videos = &tabs[1]["tabRenderer"]["content"]["richGridRenderer"]["contents"];
        let videos = videos
            .as_array()
            .ok_or("Couldn't convert videos to array")?;

        let mut videos_out = vec![];
        for video_value in videos {
            let video_value = &video_value["richItemRenderer"]["content"]["videoRenderer"];

            // Some nulls are expected.
            if !video_value.is_null() {
                match get_video(channel_id, video_value, now) {
                    Ok(video) => videos_out.push(video),
                    Err(e) => println!("Failed to get video: {} because: {}", video_value, e),
                };
            }
        }
        Ok(videos_out)
    } else {
        println!("Couldn't get json from page: {}", url);
        Ok(vec![])
    }
}

fn main() {
    println!("Hello, world!");
    let channel_id = "tested";
    let a = get_channel_videos(channel_id, &chrono::Utc::now()).unwrap();

    let mut connection = {
        let connection_future =
            PgConnection::connect("postgres://user:password@localhost/postgres");
        futures::executor::block_on(connection_future).expect("failed to obtain db connection")
    };

    for v in a {
        let query = sqlx::query("INSERT INTO videos VALUES ($1, $2, $3, $4);")
            .bind(&v.channel_id)
            .bind(&v.video_id)
            .bind(&v.title)
            .bind(&v.date)
            .execute(&mut connection);
        let query = futures::executor::block_on(query).unwrap();
    }
}
