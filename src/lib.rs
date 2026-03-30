use thiserror::Error;

use crate::{client::Client, types::World};

pub mod client;

pub mod types;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("JSON parse error: {0}")]
    SerdeError(String),

    #[error("Invalid response from API")]
    InvalidResponse,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HttpError(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeError(e.to_string())
    }
}
