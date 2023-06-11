use std::collections::HashMap;

use reqwest::{header, Client, StatusCode};

use crate::errors::{ClientError, CoCClientError, ServerError};

/// Represents a Clash of Clans API client.
pub struct CoCClient {
    pub base_url: String,
    pub bearer_token: String,
    pub client: Option<Client>,
    pub url: String,
    pub version: String,
}

pub struct ClientResponse {
    pub text: String,
    pub status_code: StatusCode,
}

impl CoCClient {
    /// Creates a new instance of `CoCClient`.
    ///
    /// # Arguments
    ///
    /// * `bearer_token` - A string representing the bearer token used for authentication.
    /// * `client` - An optional `Client` instance for making HTTP requests. If `None` is provided,
    ///              a default `Client` will be created with custom headers.
    ///
    /// # Returns
    ///
    /// A new instance of `CoCClient` initialized with the provided bearer token and client.
    ///
    pub fn new(bearer_token: String, client: Option<Client>) -> Self {
        let base_url = String::from("https://api.clashofclans.com");

        let version = String::from("v1");

        let url = format!("{}/{}", base_url, version);

        let client: Client = match client {
            Some(client) => client,
            None => {
                let mut headers = header::HeaderMap::new();

                let mut auth_value =
                    header::HeaderValue::from_str(&format!("Bearer {}", bearer_token)).unwrap();

                auth_value.set_sensitive(true);

                headers.insert(header::AUTHORIZATION, auth_value);

                Client::builder().default_headers(headers).build().unwrap()
            }
        };

        Self {
            base_url,
            bearer_token,
            client: Some(client),
            url,
            version,
        }
    }

    /// Handles the response from the client and deserializes it into the specified type `T`.
    ///
    /// # Arguments
    ///
    /// * `client_response` - The response received from the HTTP client.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the deserialized response as `T` on success,
    /// or a `CoCClientError` if there was an error in deserialization or the response status code is unexpected.
    pub async fn handle_response<T>(client_response: ClientResponse) -> Result<T, CoCClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        match client_response.status_code {
            status_code if status_code == StatusCode::OK => {
                let result: T = serde_json::from_str(&client_response.text)
                    .map_err(CoCClientError::Deserlisation)?;
                Ok(result)
            }
            StatusCode::BAD_REQUEST
            | StatusCode::FORBIDDEN
            | StatusCode::NOT_FOUND
            | StatusCode::TOO_MANY_REQUESTS => {
                let error_body: ClientError = serde_json::from_str(&client_response.text)
                    .map_err(CoCClientError::Deserlisation)?;
                Err(CoCClientError::ClientError(error_body))
            }
            StatusCode::INTERNAL_SERVER_ERROR | StatusCode::SERVICE_UNAVAILABLE => {
                let error_body: ServerError = serde_json::from_str(&client_response.text)
                    .map_err(CoCClientError::Deserlisation)?;
                Err(CoCClientError::ServerError(error_body))
            }
            _ => Err(CoCClientError::UnkownError),
        }
    }

    /// Sends a GET request to the specified path and returns the client response.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to which the GET request should be sent.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the client response on success, or a `CoCClientError`
    /// if there was an error sending the request or receiving the response.
    pub async fn send_get_request(
        self,
        path: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<ClientResponse, CoCClientError> {
        let client = self.client.ok_or(CoCClientError::MissingClientError)?;

        let mut request_builder = client.get(path);

        if let Some(params) = params {
            for (key, value) in params {
                request_builder = request_builder.query(&[(key, value)]);
            }
        }

        let response = request_builder
            .send()
            .await
            .map_err(CoCClientError::Request)?;

        let status_code = response.status();
        let text = response.text().await.map_err(CoCClientError::Request)?;

        let client_response = ClientResponse { text, status_code };

        Ok(client_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_coc_client() {
        let bearer_token = String::from("MY_BEARER_TOKEN");

        let base_url = String::from("https://api.clashofclans.com");

        let version = String::from("v1");

        let url = String::from("https://api.clashofclans.com/v1");

        let coc_client = CoCClient::new(bearer_token, None);

        assert_eq!(coc_client.bearer_token, String::from("MY_BEARER_TOKEN"));

        assert_eq!(coc_client.base_url, base_url);

        assert_eq!(coc_client.version, version);

        assert_eq!(coc_client.url, url);

        assert!(coc_client.client.is_some())
    }
}
