//! client HTTP facade

use actix_web::rt::task;
use async_trait::async_trait;
use log::{error, info};
use serde::Deserialize;
use tokio::{join, task::JoinSet};

use crate::error::AppError;

use rand::Rng;

#[async_trait]
pub trait NonUnique {
    /// Get the non unique resource from `X` url, `N` times
    /// Ex: [3, 2, 5, 1, 5, 7, 2, 1] -> [1, 2, 5]
    async fn non_unique(&self, addr: url::Url, amount: u32) -> Result<Vec<u32>, AppError>;
}

/// Implementation of getting more than once resposnses
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Deserialize)]
struct Response {
    json: ResponseValue,
}

#[derive(Deserialize)]
struct ResponseValue {
    value: u32,
}

// #[derive(Deserialize)]
// struct ResponseData {
//     value: u32,
// }

#[async_trait]
impl NonUnique for Client {
    // TODO: Split into smaller?
    async fn non_unique(&self, addr: url::Url, amount: u32) -> Result<Vec<u32>, AppError> {
        let mut tasks = JoinSet::new();
        let mut responses = vec![];

        for i in 0..amount {
            let random = rand::random::<u32>() % 10;
            let body = format!("{{\"value\": {} }}", random); // TODO

            let fut = self.client.post(addr.clone()).body(body).send();
            tasks.spawn(fut);
        }

        while let Some(task) = tasks.join_next().await {
            // responses.push(res.map_err(|e| AppError::TaskError(e)))

            let res = task.unwrap().unwrap().json::<Response>().await.unwrap();

            responses.push(res.json.value);
        }

        Ok(responses)
    }
}

// let val = task.and_then(|t| t.and_then(|v| v.json().await));

// match task {
//     Ok(res) => {
//         info!("Joined task: {:?}", res);

//         match res {
//             Ok(v) => {

//                 v.json()

//                 // let body = v.text().await;

//                 // println!("BODY: {:?}", body);

//                 // match b

//                 // v.j

//                 // info!("{}", v.status());

//                 // let body = v.text();d
//                 // info!("{:?}", v.clone().text().await);
//                 // info!("{:?}", v.text_with_charset("utf-8").await);

//                 // responses.push(v);
//             }
//             Err(e) => {
//                 error!("Error with response: {:?}", e);
//             }
//         }
//     }
//     Err(e) => {
//         error!("Error while joining task: {:?}", e);
//     }
// }
