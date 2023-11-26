//! client HTTP facade

use std::collections::{BTreeMap, HashMap};

use actix_web::rt::task;
use async_trait::async_trait;
use log::{error, info};
use serde::Deserialize;
use tokio::{join, task::JoinSet};

use crate::error::AppError;

use rand::Rng;

// #[async_trait]
// pub trait ResourceFilter {
//     /// Get non unique resource, ex: [3, 2, 5, 1, 5, 7, 2, 1] -> [1, 2, 5]
//     async fn non_unique(&self, amount: u32) -> Result<Vec<u32>, AppError> {
//         let resources = self.random_resources(amount).await?;

//         let mut resp_map = BTreeMap::<u32, u32>::new();

//         resources.iter().map(|v| match resp_map.get_mut(v) {
//             Some(v) => {
//                 *v += 1;
//             }
//             None => {
//                 resp_map.insert(*v, 1);
//             }
//         });

//         Ok(resp_map.iter().filter(|v| v.1 > &1).map(|v| *v.0).collect())
//     }
// }

#[async_trait]
pub trait NonUnique {
    /// Send, then receive N times random number
    async fn random_resources(&self, amount: u32) -> Result<Vec<u32>, AppError>;

    /// Get non unique resource, ex: [3, 2, 5, 1, 5, 7, 2, 1] -> [1, 2, 5]
    async fn non_unique(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        let resources = self.random_resources(amount).await?;

        let mut resp_map = BTreeMap::<u32, u32>::new();

        resources.iter().map(|v| match resp_map.get_mut(v) {
            Some(v) => {
                *v += 1;
            }
            None => {
                resp_map.insert(*v, 1);
            }
        });

        Ok(resp_map.iter().filter(|v| v.1 > &1).map(|v| *v.0).collect())
    }
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

#[derive(Deserialize)]
struct Response {
    json: ResponseValue,
}

#[derive(Deserialize)]
struct ResponseValue {
    value: u32,
}

#[async_trait]
impl NonUnique for Client {
    // TODO: Split into smaller?
    async fn random_resources(&self, amount: u32) -> Result<Vec<u32>, AppError> {
        let url: String = format!("{}/post", self.base_url);

        let mut tasks = JoinSet::new();
        let mut responses = vec![];

        for i in 0..amount {
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

    // async fn non_unique(&self, amount: u32) -> Result<Vec<u32>, AppError> {
    //     let resources = self.random_resources(amount).await?;

    //     let mut resp_map = BTreeMap::<u32, u32>::new();

    //     resources.iter().map(|v| match resp_map.get_mut(v) {
    //         Some(v) => {
    //             *v += 1;
    //         }
    //         None => {
    //             resp_map.insert(*v, 1);
    //         }
    //     });

    //     Ok(resp_map.iter().filter(|v| v.1 > &1).map(|v| *v.0).collect())
    // }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use rstest::rstest;

    use crate::error::AppError;

    use super::NonUnique;

    struct MockClient {}

    #[async_trait]
    impl NonUnique for MockClient {
        async fn random_resources(&self, amount: u32) -> Result<Vec<u32>, AppError> {
            todo!()
        }
    }

    #[rstest]
    #[case(vec![3, 2, 5, 1, 5, 7, 2, 1], vec![1, 2, 5])]
    #[case(vec![5, 7, 7], vec![7])]
    fn non_unique(#[case] input: Vec<u32>, #[case] output: Vec<u32>) {
        //
    }

    #[test]
    fn testing() {
        //
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

// let res = task.unwrap().unwrap().json::<Response>().await.unwrap();

// task.map_err(|e| error!("Error while receiving task {:?}", e))
//     .and_then(|v| v.map(|v| v.json::<Response>()));

// if let Err(e) = task {
//     error!("Error occured while joining task: {}", e);
//     continue;
// }

// Result<Result<Response, Error>, JoinError>
