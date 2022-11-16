use shaco::{rest::RESTClient, api::api_endpoints::{get_lol_match_history_v1_products_lol_current_summoner_matches, get_lol_match_history_v1_games_by_game_id}};
use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="20")]
    amount: u32
}

#[tokio::main]
async fn main() {
    // Parse arguments and verify amount of games to fetch
    let args = Args::parse();

    let mut end_index = args.amount - 1;
    if end_index > 199 {
        println!("A maximum of 200 games can be fetched, fetching 200.");
        end_index = 199;
    }

    let client = RESTClient::new().expect("Failed to create client.");

    // Get match history

    let matches = get_lol_match_history_v1_products_lol_current_summoner_matches(&client, None, Some(end_index)).await.unwrap();

    // Create output directory
    fs::create_dir_all("out").expect("Failed to create output directory");

    // Find and fetch custom games
    for match_history in matches.games.games {
        // Check for custom games played on Summoner's rift (mapId 11)
        if (match_history.game_mode == "CLASSIC") & (match_history.game_type == "CUSTOM_GAME") & (match_history.map_id == 11) {
            // Fetch game and verify that it has 10 players
            let game_id = match_history.game_id;
            let full_match_history = get_lol_match_history_v1_games_by_game_id(&client, game_id).await.unwrap();
            if full_match_history.participant_identities.len() == 10 {
                // Serialize json object and write string to json file in output folder
                let full_match_history_string = serde_json::to_string(&full_match_history).expect("Failed to serialize json");
                fs::write(format!("out/{game_id}.json"), full_match_history_string).expect("Unable to write file");
            }
        }
    }
}
