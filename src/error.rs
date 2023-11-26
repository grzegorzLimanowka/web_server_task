use actix_web::{error, http::header::ContentType, HttpResponse};
use thiserror::Error;
use tokio::task::JoinError;
use url::ParseError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("SomeError")]
    InternalError,

    #[error("{0}")]
    UrlParseError(#[from] ParseError),

    #[error("Task error {0}")]
    JoinError(JoinError),

    #[error("Http client error {0}")]
    ClientHttpError(reqwest::Error),

    #[error("Task error {0}")]
    TaskError(#[from] JoinError),
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> reqwest::StatusCode {
        // TODO: Fix errors:
        match self {
            AppError::InternalError => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UrlParseError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ClientHttpError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JoinError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TaskError(_) => todo!(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

// #[derive(Error, Debug)]
// pub enum ClientHttpError {
//     #[error("Reqwest error {0}")]
//     ReqwestError(reqwest::Error),
// }
