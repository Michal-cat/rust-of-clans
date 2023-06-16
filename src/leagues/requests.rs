use std::collections::HashMap;

use crate::{client::CoCClient, errors::CoCClientError, clans::models::CapitalLeague};

use super::models::LeagueList;

impl CoCClient {
    pub async fn get_capital_leagues(self: Self, params: Option<HashMap<&str, &str>>) -> Result<LeagueList<CapitalLeague>, CoCClientError> {
        let path = format!("{}/capitalleagues/", self.url);

        let client_response = match self.send_get_request(&path, params).await {
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
    async fn test_get_capital_leagues() {
        let client = set_up_client();

        match client.get_capital_leagues(None).await {
            Ok(_) => {
                assert!(true);
            }
            Err(err) => {
                println!("{}", err);
                assert!(false);
            }
        };
    }

    #[tokio::test]
    #[ignore = "Bearer token is not configured for GitHub Actions IP address"]
    async fn test_get_capital_leagues_with_params() {
        let client = set_up_client();

        let mut params: HashMap<&str, &str> = HashMap::new();

        params.insert("limit", "5");
        
        match client.get_capital_leagues(None).await {
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