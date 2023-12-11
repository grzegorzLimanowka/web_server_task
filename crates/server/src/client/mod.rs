//! client HTTP facade

mod filter;
mod manager;

use std::marker::PhantomData;

use async_trait::async_trait;
use futures::StreamExt;
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
    async fn enqueue_single<'a>(
        &self,
        tasks: &'a mut JoinSet<Result<reqwest::Response, reqwest::Error>>,
    );

    /// fetch multiple resources
    async fn enqueue_all<'a>(
        &self,
        amount: u32,
    ) -> JoinSet<Result<reqwest::Response, reqwest::Error>>;

    async fn consume_all<'a>(&self, amount: u32) -> Result<Vec<T>, AppError>;

    async fn consume_non_unique(&self, amount: u32) -> Result<Vec<T>, AppError>;
}

enum RespState {
    Pending,
    Finished(Result<u32, AppError>),
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
    async fn enqueue_single<'a>(
        &self,
        tasks: &'a mut JoinSet<Result<reqwest::Response, reqwest::Error>>,
    ) {
        let url: String = format!("{}/post", self.base_url);

        let random = rand::random::<u32>() % 10;
        let body = format!("{{\"value\": {} }}", random); // TODO

        let fut = self.client.post(url.clone()).body(body).send();
        tasks.spawn(fut);
    }

    async fn enqueue_all<'a>(
        &self,
        amount: u32,
    ) -> JoinSet<Result<reqwest::Response, reqwest::Error>> {
        let mut tasks: JoinSet<Result<reqwest::Response, reqwest::Error>> = JoinSet::new();

        for _ in 0..amount {
            self.enqueue_single(&mut tasks).await;
        }

        tasks
    }

    async fn consume_all<'a>(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        let mut tasks = self.enqueue_all(amount).await;
        let mut responses = vec![];

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

    async fn consume_non_unique(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        let all = self.consume_all(amount).await?;

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
    async fn enqueue_single<'a>(
        &self,
        tasks: &'a mut JoinSet<Result<reqwest::Response, reqwest::Error>>,
    ) {
        todo!()
    }

    async fn enqueue_all<'a>(
        &self,
        amount: u32,
    ) -> JoinSet<Result<reqwest::Response, reqwest::Error>> {
        todo!()
    }

    async fn consume_all<'a>(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        todo!()
    }

    async fn consume_non_unique(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        todo!()
    }
}
