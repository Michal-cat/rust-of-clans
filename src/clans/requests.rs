use std::collections::HashMap;

use urlencoding::encode;

use crate::{
    client::{CoCClient, PagedResponse},
    errors::CoCClientError,
};

use super::models::{
    Clan, ClanCapitalRaidSeason, ClanMember, ClanWar, ClanWarLeagueGroup, ClanWarLogEntry,
};

impl CoCClient {
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

        let client_response = match self.send_get_request(&path, None).await {
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

        let client_response = match self.send_get_request(&path, None).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    /// Retrieves information about a clan war league war based on the specified war tag.
    ///
    /// # Arguments
    ///
    /// * `war_tag` - The tag of the war for which to retrieve information.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the clan war league group information as `ClanWarLeagueGroup` on success,
    /// or a `CoCClientError` if there was an error in the request or response.
    pub async fn get_clan_war_league_war(
        self,
        war_tag: &str,
    ) -> Result<ClanWar, CoCClientError> {
        let encoded_war_tag = encode(&war_tag).into_owned();

        let path = format!("{}/clanwarleagues/wars/{}", self.url, encoded_war_tag);

        let client_response = match self.send_get_request(&path, None).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    /// Retrieves the clan war log for the specified clan tag.
    ///
    /// # Arguments
    ///
    /// * `clan_tag` - The tag of the clan for which to retrieve the war log.
    /// * `params` - Optional parameters for filtering the war log (e.g., limit, before, after).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the clan war log as `ClanWarLog` on success,
    /// or a `CoCClientError` if there was an error in the request or response.
    pub async fn get_clan_war_log(
        self: Self,
        clan_tag: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<ClanWarLogEntry>, CoCClientError> {
        let encoded_clan_tag = encode(&clan_tag).into_owned();

        let path = format!("{}/clans/{}/warlog", self.url, encoded_clan_tag);

        let client_response = match self.send_get_request(&path, params).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    /// Retrieves information about the current clan war for the specified clan tag.
    ///
    /// # Arguments
    ///
    /// * `clan_tag` - The tag of the clan for which to retrieve the current war information.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the current clan war information as `ClanWar` on success,
    /// or a `CoCClientError` if there was an error in the request or response.
    pub async fn get_current_clan_war(
        self: Self,
        clan_tag: &str,
    ) -> Result<ClanWar, CoCClientError> {
        let encoded_clan_tag = encode(&clan_tag).into_owned();

        let path = format!("{}/clans/{}/currentwar", self.url, encoded_clan_tag);

        let client_response = match self.send_get_request(&path, None).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    /// Retrieves the members of the specified clan.
    ///
    /// # Arguments
    ///
    /// * `clan_tag` - The tag of the clan for which to retrieve the members.
    /// * `params` - Optional parameters for filtering the clan members (e.g., limit, order).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the clan members as `ClanMembers` on success,
    /// or a `CoCClientError` if there was an error in the request or response.
    pub async fn get_clan_members(
        self: Self,
        clan_tag: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<ClanMember>, CoCClientError> {
        let encoded_clan_tag = encode(&clan_tag).into_owned();

        let path = format!("{}/clans/{}/members", self.url, encoded_clan_tag);

        let client_response = match self.send_get_request(&path, params).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    /// Retrieves the capital raid seasons for the specified clan tag.
    ///
    /// # Arguments
    ///
    /// * `clan_tag` - The tag of the clan for which to retrieve the capital raid seasons.
    /// * `params` - Optional parameters for filtering the capital raid seasons (e.g., limit, start time).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the capital raid seasons as `CapitalRaidSeasons` on success,
    /// or a `CoCClientError` if there was an error in the request or response.
    pub async fn get_clan_capital_raid_seasons(
        self: Self,
        clan_tag: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<ClanCapitalRaidSeason>, CoCClientError> {
        let encoded_clan_tag = encode(&clan_tag).into_owned();

        let path = format!("{}/clans/{}/capitalraidseasons", self.url, encoded_clan_tag);

        let client_response = match self.send_get_request(&path, params).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_clans(
        self: Self,
        params: HashMap<&str, &str>,
    ) -> Result<PagedResponse<Clan>, CoCClientError> {
        let path = format!("{}/clans", self.url);

        let client_response = match self.send_get_request(&path, Some(params)).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static CLAN_TAG: &str = "#2LUGVU89Q";

    fn set_up_client() -> CoCClient {
        let bearer_token = std::env::var("BEARER_TOKEN").expect("env var BEARER_TOKEN not set");

        CoCClient::new(bearer_token, None)
    }

    #[tokio::test]
    #[ignore = "Bearer token is not configured for GitHub Actions IP address"]
    async fn test_get_clan_information() {
        let client = set_up_client();

        match client.get_clan_information(CLAN_TAG).await {
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
    async fn test_get_current_war_league_group() {
        let client = set_up_client();

        match client.get_current_war_league_group(CLAN_TAG).await {
            Ok(_) => {
                assert!(true);
            }
            Err(err) => match err {
                // Assert true on any client error, should just assert true on not found error.
                CoCClientError::ClientError(_) => assert!(true),
                _ => {
                    println!("{}", err);
                    assert!(false)
                }
            },
        };
    }

    #[tokio::test]
    #[ignore = "Bearer token is not configured for GitHub Actions IP address"]
    async fn test_get_clan_war_league_war() {
        let client = set_up_client();

        match client.get_clan_war_league_war(CLAN_TAG).await {
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
    async fn test_get_clan_war_log() {
        let client = set_up_client();

        match client.get_clan_war_log(CLAN_TAG, None).await {
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
    async fn test_get_current_clan_war() {
        let client = set_up_client();

        match client.get_current_clan_war(CLAN_TAG).await {
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
    async fn test_get_clan_members() {
        let client = set_up_client();

        match client.get_clan_members(CLAN_TAG, None).await {
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
    async fn test_get_clan_members_with_limit_param() {
        let client = set_up_client();

        let mut params: HashMap<&str, &str> = HashMap::new();

        params.insert("limit", "5");

        match client.get_clan_members(CLAN_TAG, Some(params)).await {
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
    async fn test_get_clan_capital_raid_seasons() {
        let client = set_up_client();

        match client.get_clan_capital_raid_seasons(CLAN_TAG, None).await {
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
    async fn test_get_clans() {
        let client = set_up_client();

        let mut params: HashMap<&str, &str> = HashMap::new();

        params.insert("name", "erlendgemmer");

        match client.get_clans(params).await {
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
