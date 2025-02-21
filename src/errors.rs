use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
/// Represents various error types that can occur in the Clash of Clans client.
pub enum CoCClientError {
    /// Represents an error that occurred during a request.
    Request(reqwest::Error),
    /// Represents an error that occurred during deserialization.
    Deserlisation(serde_json::Error),
    /// Represents an error that occurred on the client side.
    ClientError(ClientError),
    /// Represents an error that occurred on the server side.
    ServerError(ServerError),
    /// Represents an error where the client is missing in the CoCClient.
    MissingClientError,
    /// Represents an unknown error.
    UnkownError,
}

impl fmt::Display for CoCClientError {
    /// Formats the `CoCClientError` as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoCClientError::Request(err) => write!(f, "Request error: {}", err),
            CoCClientError::Deserlisation(err) => write!(f, "Deserialization error: {}", err),
            CoCClientError::ClientError(err) => write!(f, "{}", err),
            CoCClientError::ServerError(err) => write!(f, "{}", err),
            CoCClientError::MissingClientError => write!(f, "Client is missing in CoCClient"),
            CoCClientError::UnkownError => write!(f, "unkown error"),
        }
    }
}

/// Represents a client-side error in the Clash of Clans client.
#[derive(Debug, Deserialize, Serialize)]
pub struct ClientError {
    /// A string that describes the reason for the client error.
    pub reason: String,
    /// A string that provides additional information about the client error.
    pub message: Option<String>,
    /// An optional string that specifies the type of the client error.
    pub r#type: Option<String>,
    /// An optional HashMap that contains additional details about the client error.
    pub detail: Option<HashMap<String, String>>,
}

/// Represents a server-side error in the Clash of Clans client.
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerError {
    /// A string that describes the reason for the server error.
    pub reason: String,
    /// A string that provides additional information about the server error.
    pub message: Option<String>,
    /// An optional string that specifies the type of the server error.
    pub r#type: Option<String>,
    /// An optional HashMap that contains additional details about the server error.
    pub detail: Option<HashMap<String, String>>,
}

/// Represents an unknown error.
#[derive(Debug)]
pub struct UnkownError {}

impl Error for ClientError {}

impl Error for ServerError {}

impl Error for UnkownError {}

impl fmt::Display for ClientError {
    /// Formats the `ClientError` as a string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Client error: Reason={}, Message={:?}, Type={:?}, Detail={:?}",
            self.reason, self.message, self.r#type, self.detail
        )
    }
}

impl fmt::Display for ServerError {
    /// Formats the `ServerError` as a string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Server error: Reason={}, Message={:?}, Type={:?}, Detail={:?}",
            self.reason, self.message, self.r#type, self.detail
        )
    }
}

impl fmt::Display for UnkownError {
    /// Formats the `UnkownError` as a string.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unkown error")
    }
}
