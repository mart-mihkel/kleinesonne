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

#[derive(Serialize, Deserialize)]
pub struct Article {
    #[serde(deserialize_with = "de_primitive")]
    id: usize,
    title: String,
    text: String,
    #[serde(deserialize_with = "de_primitive")]
    date: u64,
    images: Vec<String>,
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
        #[derive(Serialize)]
        struct ErrorMessage {
            error: String,
        }

        let (status, message) = match self {
            NewsError::DbError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };

        let body = Json(ErrorMessage {
            error: message.into(),
        });

        (status, body).into_response()
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
    Json(payload): Json<Article>,
) -> Result<StatusCode, NewsError> {
    conn.call(move |conn| {
        let Article {
            id,
            title,
            text,
            date,
            images,
        } = payload;

        let tx = conn.transaction()?;

        tx.execute(
            "insert into news (id, title, text, date) values (?1, ?2, ?3, ?4)",
            params![id, title, text, date],
        )?;

        for img in images {
            tx.execute(
                "insert into news_images (article, path) values (?1, ?2)",
                params![id, img],
            )?;
        }

        tx.commit()?;

        Ok(())
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
        .map_err(|_| NewsError::DbError)?;

    Ok(Json(news))
}

pub async fn update(
    State(conn): State<Connection>,
    Json(payload): Json<Article>,
) -> Result<StatusCode, NewsError> {
    conn.call(move |conn| {
        let Article {
            id,
            title,
            text,
            date,
            images: _,
        } = payload;

        conn.execute(
            "update news set (title, text, date) = (?2, ?3, ?4) where news.id = ?1",
            params![id, title, text, date],
        )?;

        Ok(())
    })
    .await
    .map_err(|_| NewsError::DbError)?;

    Ok(StatusCode::OK)
}

pub async fn delete(
    State(conn): State<Connection>,
    Json(payload): Json<DeletePayload>,
) -> Result<StatusCode, NewsError> {
    conn.call(move |conn| {
        conn.execute("delete from news where news.id = ?1", [payload.id])?;
        Ok(())
    })
    .await
    .map_err(|_| NewsError::DbError)?;

    Ok(StatusCode::OK)
}
