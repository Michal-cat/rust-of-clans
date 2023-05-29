use crate::errors::{ClientError, CoCClientError, ServerError};
use crate::models::{Clan, ClanWarLeagueGroup};
use crate::CoCClient;
use reqwest::StatusCode;
use urlencoding::encode;

pub struct ClientResponse {
    pub text: String,
    pub status_code: StatusCode,
}

impl CoCClient {
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
    async fn handle_response<T>(client_response: ClientResponse) -> Result<T, CoCClientError>
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
    async fn send_get_request(self, path: &str) -> Result<ClientResponse, CoCClientError> {
        let client = self.client.ok_or(CoCClientError::MissingClientError)?;
        let response = client
            .get(path)
            .send()
            .await
            .map_err(CoCClientError::Request)?;
        let status_code = response.status();
        let text = response.text().await.map_err(CoCClientError::Request)?;

        let client_response = ClientResponse { text, status_code };

        Ok(client_response)
    }

    /// Retrieves clan information for the specified clan tag.
    ///
    /// # Arguments
    ///
    /// * `clan_tag` - The tag of the clan for which to retrieve information.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the clan information as `Clan` on success,
    /// or a `CoCClientError` if there was an error in the request or response.
    pub async fn get_clan_information(self: Self, clan_tag: &str) -> Result<Clan, CoCClientError> {
        let encoded_clan_tag = encode(&clan_tag).into_owned();

        let path = format!("{}/clans/{}", self.url, encoded_clan_tag);

        let client_response = match self.send_get_request(&path).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    /// Retrieves the current war league group for the specified clan tag.
    ///
    /// # Arguments
    ///
    /// * `clan_tag` - The tag of the clan for which to retrieve the current war league group.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the current war league group as `ClanWarLeagueGroup` on success,
    /// or a `CoCClientError` if there was an error in the request or response.
    pub async fn get_current_war_league_group(
        self,
        clan_tag: &str,
    ) -> Result<ClanWarLeagueGroup, CoCClientError> {
        let encoded_clan_tag = encode(&clan_tag).into_owned();

        let path = format!(
            "{}/clans/{}/currentwar/leaguegroup",
            self.url, encoded_clan_tag
        );

        let client_response = match self.send_get_request(&path).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }
}
