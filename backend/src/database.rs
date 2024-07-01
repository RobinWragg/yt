use sqlx::{postgres::PgConnection, Connection};
use std::error::Error;

use crate::VideoDetails;

#[must_use]
pub fn open_connection() -> PgConnection {
    let connection_future =
        PgConnection::connect("postgres://postgres:password@localhost/postgres");
    futures::executor::block_on(connection_future).unwrap()
}

#[must_use]
pub fn insert_video(video: &VideoDetails) -> Result<(), Box<dyn Error>> {
    let mut connection = open_connection();

    let query = sqlx::query("INSERT INTO videos VALUES ($1, $2, $3, $4, $5);")
        .bind(&video.video_id)
        .bind(&video.channel_id)
        .bind(&video.title)
        .bind(&video.date)
        .bind(false)
        .execute(&mut connection);

    if let Err(e) = futures::executor::block_on(query) {
        Err(Box::new(e))
    } else {
        Ok(())
    }
}
