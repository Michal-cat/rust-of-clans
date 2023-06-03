use crate::errors::CoCClientError;
use urlencoding::encode;

use crate::client::CoCClient;

use super::models::Player;

impl CoCClient {
    /// Retrieves player information for the specified player tag.
    ///
    /// # Arguments
    ///
    /// * `player_tag` - The tag of the player for which to retrieve information.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the player information as `Player` on success,
    /// or a `CoCClientError` if there was an error in the request or response.
    pub async fn get_player_info(self: Self, player_tag: &str) -> Result<Player, CoCClientError> {
        let encoded_player_tag = encode(&player_tag).into_owned();

        let path = format!("{}/players/{}", self.url, encoded_player_tag);

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

    static PLAYER_TAG: &str = "#2288UCQJ";

    fn set_up_client() -> CoCClient {
        let bearer_token = std::env::var("BEARER_TOKEN").expect("env var BEARER_TOKEN not set");

        CoCClient::new(bearer_token, None)
    }

    #[tokio::test]
    #[ignore = "Bearer token is not configured for GitHub Actions IP address"]
    async fn test_get_player_info() {
        let client = set_up_client();

        match client.get_player_info(PLAYER_TAG).await {
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
