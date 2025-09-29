use sqlx::{postgres::*, Connection, Row};
use std::error::Error;

use crate::VideoDetails;

pub fn select_outofdate_channels() -> Result<Vec<String>, Box<dyn Error>> {
    let mut connection = open_connection();

    let query_str = "
        SELECT DISTINCT channel_id
        FROM channels
        WHERE NOT EXISTS (
            SELECT 1
            FROM videos
            WHERE videos.channel_id = channels.channel_id
            AND videos.published > (NOW() - INTERVAL '24 hours')
        );
    ";

    let query = sqlx::query(&query_str).fetch_all(&mut connection);

    match futures::executor::block_on(query) {
        Ok(row) => {
            let strings: Vec<String> = row.iter().map(|row| row.get(0)).collect();
            Ok(strings)
        }
        Err(e) => Err(e.into()),
    }
}

pub fn dump_to_json() -> Result<String, Box<dyn Error>> {
    let mut connection = open_connection();
    let table_names = select_all_table_names()?;

    let mut json_map = serde_json::Map::new();
    for table_name in table_names {
        let command = format!("SELECT json_agg({table_name}) FROM {table_name};");
        let query = sqlx::query(&command).fetch_one(&mut connection);

        match futures::executor::block_on(query) {
            Ok(row) => {
                let value = row.try_get_raw(0)?;
                match value.as_str() {
                    Ok(v) => {
                        let jf = serde_json::from_str(v)?;
                        json_map.insert(table_name, jf);
                    }
                    Err(e) => return Err(e.to_string().into()),
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    let json_object = serde_json::Value::Object(json_map);
    Ok(json_object.to_string())
}

pub fn select_unwatched_videos_as_json() -> Result<String, Box<dyn Error>> {
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

pub fn insert_channel(channel_id: &str) -> Result<(), InsertError> {
    let mut connection = open_connection();

    let query = sqlx::query("INSERT INTO channels VALUES ($1);")
        .bind(&channel_id)
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

pub fn set_video_watched(video_id: &str) -> Result<(), Box<dyn Error>> {
    let mut connection = open_connection();

    let query = sqlx::query("UPDATE videos SET watched=TRUE WHERE video_id=$1;")
        .bind(&video_id)
        .execute(&mut connection);

    if let Err(e) = futures::executor::block_on(query) {
        Err(Box::new(e))
    } else {
        Ok(())
    }
}

pub fn select_all_channel_ids() -> Result<Vec<String>, Box<dyn Error>> {
    let mut connection = open_connection();

    let query = sqlx::query("SELECT channel_id FROM channels;").fetch_all(&mut connection);

    match futures::executor::block_on(query) {
        Ok(row) => {
            let strings: Vec<String> = row.iter().map(|row| row.get(0)).collect();
            Ok(strings)
        }
        Err(e) => Err(e.into()),
    }
}

pub fn select_all_channel_ids_as_json() -> Result<String, Box<dyn Error>> {
    let channels_vec = select_all_channel_ids()?;
    Ok(serde_json::to_string(&channels_vec)?)
}

pub fn delete_channel(channel_id: &str) -> Result<(), Box<dyn Error>> {
    let mut connection = open_connection();

    // First delete all videos associated with the channel
    let delete_videos_query = sqlx::query("DELETE FROM videos WHERE channel_id=$1;")
        .bind(&channel_id)
        .execute(&mut connection);

    if let Err(e) = futures::executor::block_on(delete_videos_query) {
        return Err(Box::new(e));
    }

    // Then delete the channel itself
    let delete_channel_query = sqlx::query("DELETE FROM channels WHERE channel_id=$1;")
        .bind(&channel_id)
        .execute(&mut connection);

    if let Err(e) = futures::executor::block_on(delete_channel_query) {
        return Err(Box::new(e));
    }

    Ok(())
}

fn select_all_table_names() -> Result<Vec<String>, Box<dyn Error>> {
    let mut connection = open_connection();

    let query = sqlx::query(
        "SELECT table_name FROM information_schema.tables WHERE table_schema='public' AND table_type='BASE TABLE';",
    )
    .fetch_all(&mut connection);

    match futures::executor::block_on(query) {
        Ok(row) => {
            let strings: Vec<String> = row.iter().map(|row| row.get(0)).collect();
            Ok(strings)
        }
        Err(e) => Err(e.into()),
    }
}

fn open_connection() -> PgConnection {
    let connection_future =
        PgConnection::connect("postgres://postgres:postgres@localhost/postgres");
    futures::executor::block_on(connection_future).unwrap()
}
