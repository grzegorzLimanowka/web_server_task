use actix_web::{get, web};

use crate::{
    client::{Client, FetchResources},
    error::AppError,
};

#[get("/run")]
async fn run(client: web::Data<Client>) -> Result<String, AppError> {
    let resources = client.fetch_non_unique(30).await?;

    Ok(format!("{:?}", resources))
}
