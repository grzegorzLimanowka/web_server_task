use actix_web::{get, web};

use crate::{
    client::{Client, FetchResources},
    error::AppError,
    AppState,
};

#[get("/run")]
async fn run(state: web::Data<AppState>) -> Result<String, AppError> {
    todo!()
    // let resources = state.client.fetch_non_unique(30).await?;

    // Ok(format!("{:?}", resources))
}
