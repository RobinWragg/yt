use actix_files::Files;
use actix_web::*;
use database::*;
use std::time::Duration;
mod crawler;
mod database;

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

#[post("api/set_video_watched")]
async fn set_video_watched(req_body: String) -> impl Responder {
    // todo
    HttpResponse::Ok().body(req_body)
}

// TODO: Untested!
#[get("api/all_channel_ids")]
async fn all_channel_ids() -> impl Responder {
    match database::select_all_channel_ids_as_json() {
        Ok(json_array) => HttpResponse::Ok().body(json_array),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

fn crawler_loop() {
    let channels = database::select_all_channel_ids().expect("Can't get channels");

    loop {
        for channel_id in &channels {
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
            .service(all_channel_ids)
            .service(
                Files::new("/yt", "../frontend/out/")
                    .index_file("index.html")
                    .show_files_listing(),
            )
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await;
}
