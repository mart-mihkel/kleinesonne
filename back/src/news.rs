use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use rusqlite::params;
use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt::Display, str::FromStr};
use tokio_rusqlite::Connection;

#[derive(Serialize)]
pub struct Article {
    id: usize,
    title: String,
    text: String,
    date: u64,
    images: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    #[serde(deserialize_with = "de_primitive")]
    id: usize,
    title: String,
    text: String,
    #[serde(deserialize_with = "de_primitive")]
    date: u64,
}

#[derive(Deserialize)]
pub struct ReadPayload {
    #[serde(deserialize_with = "de_primitive")]
    count: usize,
    #[serde(deserialize_with = "de_primitive")]
    from: usize,
}

#[derive(Deserialize)]
pub struct DeletePayload {
    #[serde(deserialize_with = "de_primitive")]
    id: usize,
}

#[derive(Serialize)]
pub enum NewsError {
    DbError,
}

impl IntoResponse for NewsError {
    fn into_response(self) -> Response {
        match self {
            NewsError::DbError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        }
        .into_response()
    }
}

fn de_primitive<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

pub async fn create(
    State(conn): State<Connection>,
    Json(payload): Json<CreatePayload>,
) -> Result<StatusCode, NewsError> {
    conn.call(move |conn| {
        let CreatePayload {
            id: _,
            title,
            text,
            date,
        } = payload;

        Ok(conn.execute(
            "insert into news (title, text, date) values (?2, ?3, ?4)",
            params![title, text, date],
        )?)
    })
    .await
    .map_err(|_| NewsError::DbError)?;

    Ok(StatusCode::OK)
}

pub async fn read(
    State(conn): State<Connection>,
    Json(payload): Json<ReadPayload>,
) -> Result<Json<Vec<Article>>, NewsError> {
    let news = conn
        .call(move |conn| {
            let ReadPayload { count, from } = payload;
            let query = r#"
                select id, title, text, date, path 
                from (select * from news where date < ?1 order by date desc limit ?2) as news 
                left join news_images as img on news.id = img.article
            "#;

            let mut stmt = conn.prepare(query)?;
            let mut map = std::collections::BTreeMap::<usize, Article>::new();
            let mut rows = stmt.query([from, count])?;
            while let Some(row) = rows.next()? {
                let id = row.get(0)?;
                let img: Option<String> = row.get(4)?;

                let article = map.entry(id).or_insert(Article {
                    id,
                    title: row.get(1)?,
                    text: row.get(2)?,
                    date: row.get(3)?,
                    images: vec![],
                });

                if let Some(path) = img {
                    article.images.push(path);
                };
            }

            Ok(map.into_values().collect::<Vec<Article>>())
        })
        .await
        .map_err(|_| NewsError::DbError)?;

    Ok(Json(news))
}

pub async fn update(
    State(conn): State<Connection>,
    Json(payload): Json<CreatePayload>,
) -> Result<StatusCode, NewsError> {
    conn.call(move |conn| {
        let CreatePayload {
            id,
            title,
            text,
            date,
        } = payload;

        Ok(conn.execute(
            "update news set (title, text, date) = (?2, ?3, ?4) where news.id = ?1",
            params![id, title, text, date],
        )?)
    })
    .await
    .map_err(|_| NewsError::DbError)?;

    Ok(StatusCode::OK)
}

pub async fn delete(
    State(conn): State<Connection>,
    Json(payload): Json<DeletePayload>,
) -> Result<StatusCode, NewsError> {
    conn.call(move |conn| Ok(conn.execute("delete from news where news.id = ?1", [payload.id])?))
        .await
        .map_err(|_| NewsError::DbError)?;

    Ok(StatusCode::OK)
}
