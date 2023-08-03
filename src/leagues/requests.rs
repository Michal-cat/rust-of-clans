use std::collections::HashMap;

use crate::{
    client::{CoCClient, PagedResponse},
    errors::CoCClientError,
};

use super::models::{League, LeagueInfo, PlayerRanking, SeasonInfo};

impl CoCClient {
    pub async fn get_capital_leagues(
        self: Self,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<LeagueInfo>, CoCClientError> {
        let path = format!("{}/capitalleagues/", self.url);

        let client_response = match self.send_get_request(&path, params).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_leagues(
        self: Self,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<League>, CoCClientError> {
        let path = format!("{}/capitalleagues/", self.url);

        let client_response = match self.send_get_request(&path, params).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_capital_league_info(
        self: Self,
        league_id: u32,
    ) -> Result<LeagueInfo, CoCClientError> {
        let path = format!("{}/capitalleagues/{}", self.url, league_id);

        let client_response = match self.send_get_request(&path, None).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_builder_base_leagues(
        self: Self,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<LeagueInfo>, CoCClientError> {
        let path = format!("{}/builderbaseleagues/", self.url);

        let client_response = match self.send_get_request(&path, params).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_builder_base_league_info(
        self: Self,
        league_id: u32,
    ) -> Result<LeagueInfo, CoCClientError> {
        let path = format!("{}/builderbaseleagues/{}", self.url, league_id);

        let client_response = match self.send_get_request(&path, None).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_league_info(self: Self, league_id: u32) -> Result<League, CoCClientError> {
        let path = format!("{}/leagues/{}", self.url, league_id);

        let client_response = match self.send_get_request(&path, None).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_war_leagues(
        self: Self,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<LeagueInfo>, CoCClientError> {
        let path = format!("{}/warleagues/", self.url);

        let client_response = match self.send_get_request(&path, params).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_war_league_info(self: Self, league_id: u32) -> Result<League, CoCClientError> {
        let path = format!("{}/warleagues/{}", self.url, league_id);

        let client_response = match self.send_get_request(&path, None).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_league_seasons(
        self: Self,
        league_id: u32,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<SeasonInfo>, CoCClientError> {
        let path = format!("{}/leagues/{}/seasons", self.url, league_id);

        let client_response = match self.send_get_request(&path, params).await {
            Ok(client_response) => client_response,
            Err(err) => return Err(err),
        };

        CoCClient::handle_response(client_response).await
    }

    pub async fn get_league_season_rankings(
        self: Self,
        league_id: u32,
        season_id: &str,
        params: Option<HashMap<&str, &str>>,
    ) -> Result<PagedResponse<PlayerRanking>, CoCClientError> {
        let path = format!("{}/leagues/{}/seasons/{}", self.url, league_id, season_id);

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

        match client.get_capital_leagues(Some(params)).await {
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
    async fn test_get_leagues() {
        let client = set_up_client();

        match client.get_leagues(None).await {
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
    async fn test_get_leagues_with_params() {
        let client = set_up_client();

        let mut params: HashMap<&str, &str> = HashMap::new();

        params.insert("limit", "5");

        match client.get_leagues(Some(params)).await {
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
    async fn test_get_capital_league_info() {
        let client = set_up_client();

        let league_id = 85000022;

        match client.get_capital_league_info(league_id).await {
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
    async fn test_get_builder_base_leagues() {
        let client = set_up_client();

        match client.get_builder_base_leagues(None).await {
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
    async fn test_get_builder_base_leagues_with_params() {
        let client = set_up_client();

        let mut params: HashMap<&str, &str> = HashMap::new();

        params.insert("limit", "5");

        match client.get_builder_base_leagues(Some(params)).await {
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
    async fn test_get_builder_base_league_info() {
        let client = set_up_client();

        let league_id = 44000004;

        match client.get_builder_base_league_info(league_id).await {
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
    async fn test_get_league_info() {
        let client = set_up_client();

        let league_id = 29000000;

        match client.get_league_info(league_id).await {
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
    async fn test_get_war_leagues() {
        let client = set_up_client();

        match client.get_war_leagues(None).await {
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
    async fn test_get_war_league_info() {
        let client = set_up_client();

        let league_id = 48000005;

        match client.get_war_league_info(league_id).await {
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
    async fn test_get_league_seasons() {
        let client = set_up_client();

        let league_id = 29000022;

        match client.get_league_seasons(league_id, None).await {
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
    async fn test_get_league_season_rankings() {
        let client = set_up_client();

        let league_id = 29000022;

        let season_id = "2023-05";

        let mut params: HashMap<&str, &str> = HashMap::new();

        params.insert("limit", "5");

        match client
            .get_league_season_rankings(league_id, season_id, Some(params))
            .await
        {
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
