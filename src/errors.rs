use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub enum CoCClientError {
    Request(reqwest::Error),
    Deserlisation(serde_json::Error),
    ClientError(ClientError),
    ServerError(ServerError),
    MissingClientError,
    UnkownError,
}

impl fmt::Display for CoCClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoCClientError::Request(err) => write!(f, "Request error: {}", err),
            CoCClientError::Deserlisation(err) => write!(f, "Deserialization error: {}", err),
            CoCClientError::ClientError(err) => write!(f, "Client error: {}", err),
            CoCClientError::ServerError(err) => write!(f, "Server error: {}", err),
            CoCClientError::MissingClientError => write!(f, "Client is missing in CoCClient"),
            CoCClientError::UnkownError => write!(f, "unkown error"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientError {
    reason: String,
    message: String,
    r#type: String,
    detail: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerError {
    reason: String,
    message: String,
    r#type: String,
    detail: HashMap<String, String>,
}

#[derive(Debug)]
pub struct UnkownError {}

impl Error for ClientError {}

impl Error for ServerError {}

impl Error for UnkownError {}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client error: {}", self)
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server error: {}", self)
    }
}

impl fmt::Display for UnkownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unkown error")
    }
}
