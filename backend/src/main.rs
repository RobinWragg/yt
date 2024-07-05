use actix_files::Files;
use actix_web::*;
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
async fn http_get_unwatched_videos() -> impl Responder {
    match database::select_unwatched_videos() {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[post("api/set_video_watched")]
async fn http_set_video_watched(req_body: String) -> impl Responder {
    // todo
    HttpResponse::Ok().body(req_body)
}

fn crawler_loop() {
    let channels = ["tested", "AdamNeely", "GameGrumps", "TheGlassCannon"];

    loop {
        for channel_id in channels {
            if let Ok(videos) = crawler::get_channel_videos(channel_id) {
                for video in videos {
                    // It's fine for this to fail if the video is already in the database.
                    if database::insert_video(&video).is_ok() {
                        println!("Inserted {} {}", video.channel_id, video.video_id);
                    }
                }
            } else {
                println!("Failed to crawl {}", channel_id); // todo: print error
            }
        }

        println!("Sleeping");

        std::thread::sleep(Duration::from_secs(30)); // todo: SECONDS_PER_DAY
    }
}

#[actix_web::main]
async fn main() {
    println!("Hello, world!");

    std::thread::spawn(crawler_loop);

    let _ = HttpServer::new(|| {
        // NOTE: Here, the API services need to be before the Files::new service.
        App::new()
            .service(http_get_unwatched_videos)
            .service(http_set_video_watched)
            .service(
                Files::new("/", "../frontend/out/")
                    .index_file("index.html")
                    .show_files_listing(),
            )
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await;
}
