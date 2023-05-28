# Rust of Clans 🛡️

Welcome to Rust of Clans, the ultimate Clash of Clans API client for Rust! 🎮 Whether you're a seasoned clan leader, a strategy enthusiast, or simply love diving into the world of Clash of Clans, this package is here to make your life easier.

## Features

- **Simplified API Access:** Rust of Clans provides a clean and intuitive interface to interact with the official Clash of Clans API effortlessly.
- **Clan Management:** Retrieve detailed information about clans, including member lists, clan wars, clan league standings, and more.
- **Player Insights:** Access player profiles, their achievements, troop levels, and attack histories, helping you strategize your next move.
- **Leaderboard Rankings:** Get global or local rankings for players and clans, and track their progress in real-time.
- **Achievements Unlocked:** Fetch information about achievements, both for individual players and clans, showcasing their accomplishments.
- **In-depth Data Analysis:** Utilize comprehensive data sets to gain insights, visualize trends, and create stunning reports for your clan.

## Usage

```rust
use rust_of_clans::CoCClient;

#[tokio::main]
async fn main() {
    // Create a new Clash of Clans API client
    let bearer_token = std::env::var("BEARER_TOKEN").expect("env var BEARER_TOKEN not set");
    let coc_client = CoCClient::new(bearer_token.to_owned(), None);

    // Get information about a clan
    let clan_tag = "#CLAN_TAG";
    let clan_info = coc_client.get_clan_information(clan_tag).await;

    match clan_info {
        Ok(clan) => {
            // Process clan information
            println!("Clan Name: {}", clan.name);
            println!("Clan Level: {}", clan.clan_level);
            // ...
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {}", error);
        }
    }
}
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

Before submitting a pull request, please ensure that your code adheres to the project's coding conventions and passes all tests.

## License

This project is licensed under the [MIT License](LICENSE).
