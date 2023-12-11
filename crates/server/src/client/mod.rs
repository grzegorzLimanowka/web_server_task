//! client HTTP facade

mod filter;
mod manager;

use std::marker::PhantomData;

use async_trait::async_trait;
use log::error;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tokio::task::JoinSet;

use crate::error::AppError;

use self::filter::{Filter, FilterResources};

#[async_trait]
/// Trait for fetching resources from API
pub trait FetchResources<T> {
    /// fetch_single resouce to be fetched
    async fn fetch_single<'a>(
        &self,
        tasks: &'a mut JoinSet<Result<reqwest::Response, reqwest::Error>>,
    );

    /// fetch multiple resources
    async fn fetch_all<'a>(&self, amount: u32) -> Result<Vec<T>, AppError>;

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

#[derive(Debug, Clone)]
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
    async fn fetch_single<'a>(
        &self,
        tasks: &'a mut JoinSet<Result<reqwest::Response, reqwest::Error>>,
    ) {
        let url: String = format!("{}/post", self.base_url);

        let random = rand::random::<u32>() % 10;
        let body = format!("{{\"value\": {} }}", random); // TODO

        let fut = self.client.post(url.clone()).body(body).send();
        tasks.spawn(fut);
    }

    async fn fetch_all<'a>(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        let mut tasks = JoinSet::new();
        let mut responses = vec![];

        for _ in 0..amount {
            self.fetch_single(&mut tasks).await;
        }

        // TODO: make it guards?
        while let Some(task) = tasks.join_next().await {
            match task {
                Ok(Ok(res)) => match res.json::<Response>().await {
                    Ok(res) => {
                        responses.push(res.json.value);
                    }
                    Err(e) => {
                        error!("Error with deserializing response: {}", e);
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

// TODO: Decorator macros could be considered to decorator with decorations of saving each request to DB
struct DbClientDecorator<C>
where
    C: FetchResources<u32>,
{
    client: C,
    conn: DatabaseConnection,
}

#[async_trait]
impl<C> FetchResources<u32> for DbClientDecorator<C>
where
    C: FetchResources<u32> + Send + Sync,
{
    /// decorate each request with db saving
    async fn fetch_single<'a>(
        &self,
        tasks: &'a mut JoinSet<Result<reqwest::Response, reqwest::Error>>,
    ) {
        self.client.fetch_single(tasks).await
    }

    /// fetch multiple resources
    async fn fetch_all<'a>(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        self.client.fetch_all(amount).await
    }

    /// fetch multiple non-unique resources
    async fn fetch_non_unique(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        self.client.fetch_non_unique(amount).await
    }
}
