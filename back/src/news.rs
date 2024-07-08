use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt::Display, str::FromStr};
use tokio_rusqlite::Connection;

#[derive(Deserialize)]
pub struct ReadPayload {
    #[serde(deserialize_with = "de_primitive")]
    count: usize,
    #[serde(deserialize_with = "de_primitive")]
    from: usize,
}

pub struct CreatePayload {}

fn de_primitive<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Serialize)]
pub struct Article {
    id: usize,
    title: String,
    text: String,
    date: u64,
    images: Vec<String>,
}

#[derive(Serialize)]
pub enum NewsError {
    Error,
}

impl IntoResponse for NewsError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorMessage {
            error: String,
        }

        let (status, message) = match self {
            NewsError::Error => (StatusCode::INTERNAL_SERVER_ERROR, "error"),
        };

        let body = Json(ErrorMessage {
            error: message.into(),
        });

        (status, body).into_response()
    }
}

pub async fn read_news(
    State(conn): State<Connection>,
    Json(payload): Json<ReadPayload>,
) -> Result<Json<Vec<Article>>, NewsError> {
    let news = conn
        .call(move |conn| {
            let ReadPayload { count, from } = payload;
            let query = r#"
                select news.id, title, text, date, path 
                from (select * from news where date < ?1 order by date desc limit ?2) as news 
                left join news_images as img on news.id = img.article
            "#;
            let mut stmt = conn.prepare(query)?;

            struct Row(usize, String, String, u64, Option<String>);
            let mut map = std::collections::BTreeMap::<usize, Article>::new();
            stmt.query_map([from, count], |row| {
                Ok(Row(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })?
            .for_each(|row| {
                let row = row.unwrap();
                let map = &mut map;
                let id = row.0;

                let article = map.entry(id).or_insert(Article {
                    id,
                    title: row.1,
                    text: row.2,
                    date: row.3,
                    images: vec![],
                });

                if let Some(path) = row.4 {
                    article.images.push(path);
                }
            });

            Ok(map.into_values().collect::<Vec<Article>>())
        })
        .await
        .map_err(|_| NewsError::Error)?;

    Ok(Json(news))
}
