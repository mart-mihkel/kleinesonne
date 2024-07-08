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
pub struct Dog {
    id: usize,
    name: String,
    nickname: String,
    father: String,
    mother: String,
    thumbnail: Option<String>,
    dob: u64,
    breed: String,
    gender: String,
    alive: bool,
    images: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    #[serde(deserialize_with = "de_primitive")]
    id: usize,
    name: String,
    nickname: String,
    father: String,
    mother: String,
    #[serde(deserialize_with = "de_primitive")]
    dob: u64,
    breed: String,
    gender: String,
    #[serde(deserialize_with = "de_primitive")]
    alive: bool,
}

#[derive(Deserialize)]
pub struct ReadPayload {
    #[serde(deserialize_with = "de_primitive")]
    id: usize,
}

#[derive(Serialize)]
pub enum DogsError {
    DbError,
}

impl IntoResponse for DogsError {
    fn into_response(self) -> Response {
        match self {
            DogsError::DbError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
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
) -> Result<StatusCode, DogsError> {
    conn.call(move |conn| {
        let CreatePayload { id: _, name, nickname, father, mother, dob, breed, gender, alive } = payload;

        Ok(conn.execute(
            "insert into dogs (name, nickname, father, mother, dob, breed, gender, alive) values (?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![name, nickname, father, mother, dob, breed, gender, alive],
        )?)
    })
    .await
    .map_err(|_| DogsError::DbError)?;

    Ok(StatusCode::OK)
}

pub async fn read(
    State(conn): State<Connection>,
    Json(payload): Json<ReadPayload>,
) -> Result<Json<Vec<Dog>>, DogsError> {
    conn.call(move |coll| {
        let query = r#"
            select id, name, nickname, father, mother, thumbnail, dob, breed, gender, alive, path
            from dogs where dogs.id = ?1
            left join dog_images as img on dogs.id = img.dog
        "#;

        let mut stmt = coll.prepare(query)?;
        let mut map = std::collections::BTreeMap::<usize, Dog>::new();
        let mut rows = stmt.query([payload.id])?;
        while let Some(row) = rows.next()? {
            let id = row.get(0)?;
            let img: Option<String> = row.get(10)?;

            let dog = map.entry(id).or_insert(Dog {
                id,
                name: row.get(1)?,
                nickname: row.get(2)?,
                father: row.get(3)?,
                mother: row.get(4)?,
                thumbnail: row.get(5)?,
                dob: row.get(6)?,
                breed: row.get(7)?,
                gender: row.get(8)?,
                alive: row.get(9)?,
                images: vec![],
            });

            if let Some(path) = img {
                dog.images.push(path);
            };
        }

        Ok(map.into_values().collect::<Vec<Dog>>())
    })
    .await
    .map_err(|_| DogsError::DbError)?;

    Err(DogsError::DbError)
}

pub async fn update(
    State(conn): State<Connection>,
    Json(payload): Json<CreatePayload>,
) -> Result<StatusCode, DogsError> {
    conn.call(move |conn| {
        let CreatePayload { id, name, nickname, father, mother, dob, breed, gender, alive } = payload;

        Ok(conn.execute(
            "update dogs set (name, nickname, father, mother, dob, breed, gender, alive) = (?2, ?3, ?4) where news.id = ?1",
            params![id, name, nickname, father,mother,dob,breed,gender,alive],
        )?)
    })
    .await
    .map_err(|_| DogsError::DbError)?;

    Ok(StatusCode::OK)
}

pub async fn delete(
    State(conn): State<Connection>,
    Json(payload): Json<ReadPayload>,
) -> Result<StatusCode, DogsError> {
    conn.call(move |conn| Ok(conn.execute("delete from dogs where news.id = ?1", [payload.id])?))
        .await
        .map_err(|_| DogsError::DbError)?;

    Ok(StatusCode::OK)
}
