use reqwest::{header, Client};

struct CoCClient {
    base_url: String,
    bearer_token: String,
    client: Option<Client>,
    url: String,
    version: String,
}

impl CoCClient {
    fn new(self, bearer_token: String, client: Option<Client>) -> Self {
        let base_url = String::from("https://api.clashofclans.com");

        let version = String::from("v1");

        let url = format!("{}/{}", base_url, version);

        let client: Client = match client {
            Some(client) => client,
            None => {
                let mut headers = header::HeaderMap::new();

                let mut auth_value = header::HeaderValue::from_static("secret");

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