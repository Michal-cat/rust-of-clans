use crate::errors::{ClientError, CoCClientError, ServerError};
use crate::models::Clan;
use crate::CoCClient;
use reqwest::StatusCode;
use urlencoding::encode;

impl CoCClient {
    pub async fn get_clan_information(
        self: Self,
        clan_tag: String,
    ) -> Result<Clan, CoCClientError> {
        let encoded_clan_tag = encode(&clan_tag).into_owned();

        let path = format!("{}/clans/{}", self.url, encoded_clan_tag);

        let client = self.client.ok_or(CoCClientError::MissingClientError)?;

        let response = client
            .get(path)
            .send()
            .await
            .map_err(CoCClientError::Request)?;

        let status_code = response.status();

        let response_text = response.text().await.map_err(CoCClientError::Request)?;

        match status_code {
            StatusCode::OK => {
                let clan: Clan =
                    serde_json::from_str(&response_text).map_err(CoCClientError::Deserlisation)?;

                Ok(clan)
            }
            StatusCode::BAD_REQUEST
            | StatusCode::FORBIDDEN
            | StatusCode::NOT_FOUND
            | StatusCode::TOO_MANY_REQUESTS => {
                let error_body: ClientError =
                    serde_json::from_str(&response_text).map_err(CoCClientError::Deserlisation)?;

                return Err(CoCClientError::ClientError(error_body));
            }
            StatusCode::INTERNAL_SERVER_ERROR | StatusCode::SERVICE_UNAVAILABLE => {
                let error_body: ServerError =
                    serde_json::from_str(&response_text).map_err(CoCClientError::Deserlisation)?;

                return Err(CoCClientError::ServerError(error_body));
            }
            _ => Err(CoCClientError::UnkownError),
        }
    }
}
