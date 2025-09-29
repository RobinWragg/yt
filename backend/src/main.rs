use actix_files::Files;
use actix_web::*;
use database::*;
use serde::Deserialize;
use std::path::PathBuf;
use std::time::Duration;
mod crawler;
mod database;

const FRONTEND_BUILD_PATH: &str = "../frontend/dist/";

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Clone)]
struct VideoDetails {
    video_id: String,
    channel_id: String,
    title: String,
    date: DateTime,
}

#[get("api/unwatched_videos")]
async fn unwatched_videos() -> impl Responder {
    match database::select_unwatched_videos_as_json() {
        Ok(json_array) => HttpResponse::Ok().body(json_array),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct VideoRequestInput {
    video_id: String,
}

#[derive(Deserialize)]
struct ChannelRequestInput {
    channel_id: String,
}

#[post("api/set_video_watched")]
async fn set_video_watched(info: web::Json<VideoRequestInput>) -> impl Responder {
    match database::set_video_watched(&info.video_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("api/set_video_watched failure: {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("api/insert_channel")]
async fn insert_channel(info: web::Json<ChannelRequestInput>) -> impl Responder {
    let channel_id = info.channel_id.trim();
    
    if channel_id.is_empty() {
        return HttpResponse::BadRequest().body("Channel ID cannot be empty");
    }
    
    match database::insert_channel(channel_id) {
        Ok(_) => {
            // Start crawling the channel in a separate thread
            let channel_id_owned = channel_id.to_string();
            std::thread::spawn(move || {
                crawl_channel(&channel_id_owned);
            });
            
            HttpResponse::Ok().finish()
        }
        Err(InsertError::AlreadyExists) => {
            HttpResponse::Conflict().body("Channel already exists")
        }
        Err(InsertError::Other(e)) => {
            println!("api/insert_channel failure: {}", e.to_string());
            HttpResponse::InternalServerError().body("Failed to insert channel")
        }
    }
}

// TODO: Untested!
#[get("api/all_channel_ids")]
async fn all_channel_ids() -> impl Responder {
    match database::select_all_channel_ids_as_json() {
        Ok(json_array) => HttpResponse::Ok().body(json_array),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[post("api/delete_channel")]
async fn delete_channel(info: web::Json<ChannelRequestInput>) -> impl Responder {
    match database::delete_channel(&info.channel_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("api/delete_channel failure: {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn index_handler(_req: HttpRequest) -> Result<HttpResponse> {
    let mut index_path = PathBuf::from(FRONTEND_BUILD_PATH);
    index_path.push("index.html");
    let index_html = std::fs::read_to_string(index_path).unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(index_html))
}

fn crawl_channel(channel_id: &str) {
    match crawler::get_channel_videos(&channel_id) {
        Ok(videos) => {
            for video in videos {
                // It's fine for this to fail if the video is already in the database.
                // todo: ignore error UNLESS it's due to a duplicate primary key.
                match database::insert_video(&video) {
                    Ok(()) => println!("Inserted {} {}", video.channel_id, video.video_id),
                    // Igore error if we've already stored this video.
                    Err(InsertError::AlreadyExists) => (),
                    Err(InsertError::Other(e)) => {
                        println!(
                            "Failed to insert video {} {} because: {}",
                            video.channel_id,
                            video.video_id,
                            e.to_string()
                        )
                    }
                }
            }
        }
        Err(e) => println!("Failed to crawl {} because: {}", channel_id, e.to_string()),
    }
}

fn crawler_loop() {
    // let channels = database::select_outofdate_channels().expect("Can't get channels");
    let channels = database::select_all_channel_ids().expect("Can't get channels");

    loop {
        for channel_id in &channels {
            crawl_channel(&channel_id);
        }

        println!("Sleeping");

        const SECONDS_PER_MINUTE: u64 = 60;
        const SECONDS_PER_HOUR: u64 = SECONDS_PER_MINUTE * 60;
        std::thread::sleep(Duration::from_secs(SECONDS_PER_HOUR * 23)); // Just under a day
    }
}

#[actix_web::main]
async fn main() {
    println!("Hello, world!");

    std::thread::spawn(crawler_loop);

    let _ = HttpServer::new(|| {
        // NOTE: Here, the API services need to be before the Files::new service.
        App::new()
            .service(unwatched_videos)
            .service(set_video_watched)
            .service(insert_channel)
            .service(all_channel_ids)
            .service(delete_channel)
            .service(
                Files::new("/", FRONTEND_BUILD_PATH)
                    .index_file("index.html")
                    .prefer_utf8(true),
            )
            .default_service(web::route().to(index_handler))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await;
}
