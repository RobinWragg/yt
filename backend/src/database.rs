use sqlx::{postgres::*, Connection, Row};
use std::error::Error;

use crate::VideoDetails;

#[must_use]
pub fn select_unwatched_videos() -> Result<String, Box<dyn Error>> {
    let mut connection = open_connection();

    let query = sqlx::query(
        "SELECT json_agg(json_build_object('video_id', video_id, 'channel_id', channel_id, 'title', title, 'published', published)) FROM videos WHERE watched=FALSE;",
    )
    .fetch_one(&mut connection);

    match futures::executor::block_on(query) {
        Ok(row) => {
            let value = row.try_get_raw(0)?;
            match value.as_str() {
                Ok(v) => Ok(v.to_owned()),
                Err(e) => Err(e.to_string().into()),
            }
        }
        Err(e) => Err(e.into()),
    }
}

pub enum InsertError {
    AlreadyExists,
    Other(Box<dyn Error>),
}

#[must_use]
pub fn insert_video(video: &VideoDetails) -> Result<(), InsertError> {
    let mut connection = open_connection();

    let query = sqlx::query("INSERT INTO videos VALUES ($1, $2, $3, $4, $5);")
        .bind(&video.video_id)
        .bind(&video.channel_id)
        .bind(&video.title)
        .bind(&video.date)
        .bind(false)
        .execute(&mut connection);

    if let Err(e) = futures::executor::block_on(query) {
        if let Some(db_error) = e.as_database_error() {
            if db_error.is_unique_violation() {
                let f = InsertError::AlreadyExists;
                return Err(f);
            }
        }

        Err(InsertError::Other(Box::new(e)))
    } else {
        Ok(())
    }
}

#[must_use]
fn open_connection() -> PgConnection {
    let connection_future =
        PgConnection::connect("postgres://postgres:postgres@localhost/postgres");
    futures::executor::block_on(connection_future).unwrap()
}
