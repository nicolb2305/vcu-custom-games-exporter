#![allow(non_snake_case)]
mod api;
use api::api_types::{LolMatchHistoryMatchHistoryList, LolMatchHistoryMatchHistoryGame};
use shaco;
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="20")]
    amount: u8
}

#[tokio::main]
async fn main() {
    // Parse arguments and verify amount of games to fetch
    let args = Args::parse();

    let mut endIndex = args.amount - 1;
    if endIndex > 199 {
        println!("A maximum of 200 games can be fetched, fetching 200.");
        endIndex = 199;
    }

    let client = shaco::rest::RESTClient::new().expect("Failed to create client.");

    // Get match history
    let endpoint = format!("/lol-match-history/v1/products/lol/current-summoner/matches?endIndex={endIndex}");

    let matches: LolMatchHistoryMatchHistoryList = serde_json::from_value(
        client
            .get(endpoint.to_string())
            .await
            .expect("Failed to fetch from endpoint.")
    ).expect("Failed to deserialize match list json");

    // Create output directory
    fs::create_dir_all("out").expect("Failed to create output directory");

    // Find and fetch custom games
    for match_history in matches.games.games {
        // Check for custom games played on Summoner's rift (mapId 11)
        if (match_history.gameMode == "CLASSIC") & (match_history.gameType == "CUSTOM_GAME") & (match_history.mapId == 11) {
            // Fetch game and verify that it has 10 players
            let gameId = match_history.gameId;
            let endpoint = format!("/lol-match-history/v1/games/{gameId}");
            let full_match_history: LolMatchHistoryMatchHistoryGame = serde_json::from_value(
                client
                    .get(endpoint.to_string())
                    .await
                    .expect("Failed to fetch from endpoint.")
            ).expect("Failed to deserialize match history json");
            if full_match_history.participantIdentities.len() == 10 {
                // Serialize json object and write string to json file in output folder
                let full_match_history_string = serde_json::to_string(&full_match_history).expect("Failed to serialize json");
                fs::write(format!("out/{gameId}.json"), full_match_history_string).expect("Unable to write file");
            }
        }
    }
}
