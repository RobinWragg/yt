use crate::VideoDetails;
use curl;
use regex::Regex;
use serde_json::Value as JsonValue;
use std::error::Error;
use std::time::Duration;

type DateTime = chrono::DateTime<chrono::Utc>;

pub fn get_channel_videos(channel_id: &str) -> Result<Vec<VideoDetails>, Box<dyn Error>> {
    let url = video_list_url(channel_id);
    let mut easy_curl = curl::easy::Easy::new();
    easy_curl.follow_location(true)?;
    let mut dst = Vec::new();
    {
        easy_curl.url(&url)?;
        let mut headers = curl::easy::List::new();
        headers.append("Accept-Language: en")?;
        easy_curl.http_headers(headers)?;
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
        let now = chrono::Utc::now();
        for video_value in videos {
            let video_value = &video_value["richItemRenderer"]["content"]["videoRenderer"];

            // Some nulls are expected.
            if !video_value.is_null() {
                match get_video(channel_id, video_value, &now) {
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

fn json_from_page(page: &str) -> Option<String> {
    let re = Regex::new(r"var ytInitialData = (.*?);</script>").ok()?;
    Some(re.captures(page)?.get(1)?.as_str().to_string())
}

fn video_list_url(channel_id: &str) -> String {
    "https://www.youtube.com/@".to_owned() + channel_id + "/videos"
}

fn parse_recency(s: &str, now: &DateTime) -> Result<DateTime, Box<dyn Error>> {
    const SECONDS_PER_MINUTE: u64 = 60;
    const SECONDS_PER_HOUR: u64 = SECONDS_PER_MINUTE * 60;
    const SECONDS_PER_DAY: u64 = SECONDS_PER_HOUR * 24;
    const SECONDS_PER_WEEK: u64 = SECONDS_PER_DAY * 7;
    const SECONDS_PER_MONTH: u64 = SECONDS_PER_DAY * 30; // Approximate!
    const SECONDS_PER_YEAR: u64 = SECONDS_PER_DAY * 365; // Approximate!

    let re = Regex::new(r"(\d+) (\w+?)s? ago")?;
    let captures = re
        .captures(s)
        .ok_or(format!("captures() call failed on str: {s}"))?;

    let base_number = captures[1].parse::<u64>()?;

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

mod tests {
    use super::*;

    #[test]
    fn test_get_video() {
        let s = std::fs::read_to_string("test_files/get_video_common.json").unwrap();
        let json: serde_json::Value = serde_json::from_str(&s).unwrap();
        let now = chrono::Utc::now();
        get_video("mychannel", &json, &now).unwrap();
    }
}
