pub mod models;
pub mod requests;
pub mod errors;

use reqwest::{header, Client};

/// Represents a Clash of Clans API client.
pub struct CoCClient {
    pub base_url: String,
    pub bearer_token: String,
    pub client: Option<Client>,
    pub url: String,
    pub version: String,
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
