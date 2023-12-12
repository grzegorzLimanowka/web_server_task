use actix_web::{get, web};

use crate::{client::FetchResources, error::AppError, AppState};

#[get("/run")]
async fn run(state: web::Data<AppState>) -> Result<String, AppError> {
    let resources = state.fetcher.consume_non_unique(30).await?;

    Ok(format!("{:?}", resources))
}
