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
    let args = Args::parse();

    let client = shaco::rest::RESTClient::new().expect("Failed to create client.");

    let mut amount = args.amount;
    if amount > 200 {
        println!("A maximum of 200 games can be fetched, fetching 200.");
        amount = 200;
    }
    let endpoint = format!("/lol-match-history/v1/products/lol/current-summoner/matches?endIndex={amount}");

    let matches: LolMatchHistoryMatchHistoryList = serde_json::from_value(
        client
            .get(endpoint.to_string())
            .await
            .expect("Failed to fetch from endpoint.")
            .clone()
    ).expect("Failed to deserialize match list json");

    for match_history in matches.games.games {
        if (match_history.gameMode == "CLASSIC") & (match_history.gameType == "CUSTOM_GAME") & (match_history.mapId == 11) {
            let gameId = match_history.gameId;
            let endpoint = format!("/lol-match-history/v1/games/{gameId}");
            let full_match_history: LolMatchHistoryMatchHistoryGame = serde_json::from_value(
                client
                    .get(endpoint.to_string())
                    .await
                    .expect("Failed to fetch from endpoint.")
                    .clone()
            ).expect("Failed to deserialize match history json");
            if full_match_history.participantIdentities.len() == 10 {
                let full_match_history_string = serde_json::to_string(&full_match_history).expect("Failed to serialize json");
                fs::create_dir_all("out").expect("Failed to create output directory");
                fs::write(format!("out/{gameId}.json"), full_match_history_string).expect("Unable to write file");
            }
        }
    }
}
