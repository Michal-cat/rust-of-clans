use crate::{client::CoCClient, errors::CoCClientError};

use super::models::GoldPassSeason;

impl CoCClient {
    pub async fn get_current_gold_pass(self: Self) -> Result<GoldPassSeason, CoCClientError> {
        let path = format!("{}/goldpass/seasons/current", self.url);

        let client_response = match self.send_get_request(&path, None).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up_client() -> CoCClient {
        let bearer_token = std::env::var("BEARER_TOKEN").expect("env var BEARER_TOKEN not set");

        CoCClient::new(bearer_token, None)
    }

    #[tokio::test]
    #[ignore = "Bearer token is not configured for GitHub Actions IP address"]
    async fn test_get_current_gold_pass() {
        let client = set_up_client();

        match client.get_current_gold_pass().await {
            Ok(_) => {
                assert!(true);
            }
            Err(err) => {
                println!("{}", err);
                assert!(false);
            }
        };
    }
}
