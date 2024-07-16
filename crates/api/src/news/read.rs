use std::sync::Arc;

use axum::{Extension, Json};
use db::news::{AllTitles, Article};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{errors::ApiError, res::ApiResponse};

#[derive(Deserialize)]
pub struct ById {
    id: i32,
}

#[derive(Deserialize)]
pub struct NewsRead {
    from: i64,
    n: i64,
}

pub async fn all_titles(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
) -> Result<ApiResponse<Vec<AllTitles>>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let titles = db::news::all_titles().bind(&tx).all().await?;

    Ok(ApiResponse::Data(titles))
}

pub async fn article_by_id(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(ById { id }): Json<ById>,
) -> Result<ApiResponse<Article>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let article = db::news::article_by_id().bind(&tx, &id).one().await?;

    Ok(ApiResponse::Data(article))
}

pub async fn n_news_older_than(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(NewsRead { from, n }): Json<NewsRead>,
) -> Result<ApiResponse<Vec<Article>>, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let news = db::news::n_news_older_than()
        .bind(&tx, &from, &n)
        .all()
        .await?;

    Ok(ApiResponse::Data(news))
}
