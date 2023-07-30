//! # Clash of Clans API Client
//!
//! This crate provides a client for interacting with the Clash of Clans API.
//! It allows you to retrieve information about clans, players, wars, and more.
//!
//! ## Example Usage
//!
//! ```no_run
//! use rust_of_clans::client::CoCClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a new Clash of Clans API client
//!     let bearer_token = std::env::var("BEARER_TOKEN").expect("env var BEARER_TOKEN not set");
//!     let coc_client = CoCClient::new(bearer_token.to_owned(), None);
//!
//!     // Get information about a clan
//!     let clan_tag = "#CLAN_TAG";
//!     let clan_info = coc_client.get_clan_information(clan_tag).await;
//!
//!     match clan_info {
//!         Ok(clan) => {
//!             // Process clan information
//!             println!("Clan Name: {}", clan.name);
//!             println!("Clan Level: {}", clan.clan_level);
//!             // ...
//!         }
//!         Err(error) => {
//!             // Handle the error
//!             eprintln!("Error: {}", error);
//!         }
//!     }
//! }
//! ```
//!
//! This example demonstrates how to create a Clash of Clans API client, retrieve information
//! about a clan, and handle the result.
//!

/// The `clans` module handles requests towards the clan endpoints and provides clan models.
/// It allows you to retrieve information about clans, search for clans, and interact with clan-related data.
/// This module encapsulates functionality related to clans in the Clash of Clans API client.
pub mod clans;

/// The `client` module builds requests, handles responses, and creates a client with a bearer token.
/// It serves as the core of the Clash of Clans API client, providing the necessary functionality to interact with the API.
/// The client module allows you to authenticate with a bearer token, send requests, and process responses from the Clash of Clans API.
pub mod client;

/// The `errors` module contains all error structs and enums.
/// It provides a set of error types that can be used throughout the crate to handle and propagate errors consistently.
/// By utilizing these error types, you can handle various error scenarios that may arise during API interactions or other operations.
pub mod errors;

/// The `players` module handles requests towards the player endpoints and provides player models.
/// It enables you to retrieve player information, search for players, and perform operations related to player data in the Clash of Clans API.
/// This module encapsulates functionality related to players in the Clash of Clans API client.
pub mod players;
pub mod leagues;
pub mod goldpass;
