//! client HTTP facade

mod filter;

use async_trait::async_trait;
use log::error;
use serde::Deserialize;
use tokio::task::JoinSet;

use crate::error::AppError;

use self::filter::{Filter, FilterResources};

#[async_trait]
pub trait FetchResources<T> {
    /// fetch multiple resources  
    async fn fetch_all(&self, amount: u32) -> Result<Vec<T>, AppError>;

    /// fetch multiple non-unique resources
    async fn fetch_non_unique(&self, amount: u32) -> Result<Vec<T>, AppError>;
}

#[derive(Deserialize)]
struct Response {
    json: ResponseValue,
}

#[derive(Deserialize)]
struct ResponseValue {
    value: u32,
}

pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: url.to_owned(),
        }
    }
}

#[async_trait]
impl FetchResources<u32> for Client {
    // TODO: Split into smaller?
    async fn fetch_all(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        let url: String = format!("{}/post", self.base_url);

        let mut tasks = JoinSet::new();
        let mut responses = vec![];

        for _ in 0..amount {
            let random = rand::random::<u32>() % 10;
            let body = format!("{{\"value\": {} }}", random); // TODO

            let fut = self.client.post(url.clone()).body(body).send();
            tasks.spawn(fut);
        }

        while let Some(task) = tasks.join_next().await {
            match task {
                Ok(Ok(res)) => match res.json::<Response>().await {
                    Ok(res) => {
                        responses.push(res.json.value);
                    }
                    Err(e) => {
                        error!("Error with deserialiszing response: {}", e);
                    }
                },
                Ok(Err(e)) => {
                    error!("Error with http response: {}", e);
                }
                Err(e) => {
                    error!("Error while joining task: {}", e);
                }
            }
        }

        Ok(responses)
    }

    async fn fetch_non_unique(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        let all = self.fetch_all(amount).await?;

        Ok(Filter.non_unique(all))
    }
}
