#![allow(non_snake_case)]
#![allow(dead_code)]
use serde_json::Value;
use shaco;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    championId: i32,
    pickTurn: u16
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryTeam {
    bans: Vec<LolMatchHistoryMatchHistoryTeamBan>,
    baronKills: u32,
    dominionVictoryScore: u32,
    dragonKills: u32,
    firstBaron: bool,
    firstBlood: bool,
    firstDargon: bool,
    firstInhibitor: bool,
    firstTower: bool,
    inhibitorKills: u32,
    riftHeraldKills: u32,
    teamId: u16,
    towerKills: u32,
    vilemawKills: u32,
    win: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryTimeline {
    creepsPerMinDeltas: Value,
    csDiffPerMinDeltas: Value,
    damageTakenDiffPerMinDeltas: Value,
    damageTakenPerMinDeltas: Value,
    goldPerMinDeltas: Value,
    lane: String,
    participantId: u16,
    role: String,
    xpDiffPerMinDeltas: Value,
    xpPerMinDeltas: Value
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryParticipantStatistics {
    assists: i64,
    causedEarlySurrender: bool,
    champLevel: i64,
    combatPlayerScore: i64,
    damageDealtToObjectives: i64,
    damageDealtToTurrets: i64,
    damageSelfMitigated: i64,
    deaths: i64,
    doubleKills: i64,
    earlySurrenderAccomplice: bool,
    firstBloodAssist: bool,
    firstBloodKill: bool,
    firstInhibitorAssist: bool,
    firstInhibitorKill: bool,
    firstTowerAssist: bool,
    firstTowerKill: bool,
    gameEndedInEarlySurrender: bool,
    gameEndedInSurrender: bool,
    goldEarned: i64,
    goldSpent: i64,
    inhibitorKills: i64,
    item0: u16,
    item1: u16,
    item2: u16,
    item3: u16,
    item4: u16,
    item5: u16,
    item6: u16,
    killingSprees: i64,
    kills: i64,
    largestCriticalStrike: i64,
    largestKillingSpree: i64,
    largestMultiKill: i64,
    longestTimeSpentLiving: i64,
    magicalDamageTaken: i64,
    magicDamageDealt: i64,
    magicDamageDealtToChampions: i64,
    neutralMinionsKilled: i64,
    neutralMinionsKilledEnemyJungle: i64,
    neutralMinionsKilledTeamJungle: i64,
    objectivePlayerScore: i64,
    participantId: u16,
    pentaKills: i64,
    perk0: i64,
    perk0Var1: i64,
    perk0Var2: i64,
    perk0Var3: i64,
    perk1: i64,
    perk1Var1: i64,
    perk1Var2: i64,
    perk1Var3: i64,
    perk2: i64,
    perk2Var1: i64,
    perk2Var2: i64,
    perk2Var3: i64,
    perk3: i64,
    perk3Var1: i64,
    perk3Var2: i64,
    perk3Var3: i64,
    perk4: i64,
    perk4Var1: i64,
    perk4Var2: i64,
    perk4Var3: i64,
    perk5: i64,
    perk5Var1: i64,
    perk5Var2: i64,
    perk5Var3: i64,
    perkPrimaryStyle: i64,
    perkSubStyle: i64,
    physicalDamageDealt: i64,
    physicalDamageDealtToChampions: i64,
    physicalDamageTaken: i64,
    playerScore0: i64,
    playerScore1: i64,
    playerScore2: i64,
    playerScore3: i64,
    playerScore4: i64,
    playerScore5: i64,
    playerScore6: i64,
    playerScore7: i64,
    playerScore8: i64,
    playerScore9: i64,
    quadraKills: i64,
    sightWardsBoughtInGame: i64,
    teamEarlySurrendered: bool,
    timeCCingOthers: i64,
    totalDamageDealt: i64,
    totalDamageDealtToChampions: i64,
    totalDamageTaken: i64,
    totalHeal: i64,
    totalMinionsKilled: i64,
    totalPlayerScore: i64,
    totalScoreRank: i64,
    totalTimeCrowdControlDealt: i64,
    totalUnitsHealed: i64,
    tripleKills: i64,
    trueDamageDealt: i64,
    trueDamageDealtToChampions: i64,
    trueDamageTaken: i64,
    turretKills: i64,
    unrealKills: i64,
    visionScore: i64,
    visionWardsBoughtInGame: i64,
    wardsKilled: i64,
    wardsPlaced: i64,
    win: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryParticipant {
    championId: i32,
    highestAchievedSeasonTier: String,
    participantId: u16,
    spell1Id: u16,
    spell2Id: u16,
    stats: LolMatchHistoryMatchHistoryParticipantStatistics,
    teamId: u16,
    timeline: LolMatchHistoryMatchHistoryTimeline
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    accountId: u64,
    currentAccountId: u64,
    currentPlatformId: String,
    matchHistoryUri: String,
    platformId: String,
    profileIcon: i32,
    summonerId: u64,
    summonerName: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    participantId: u16,
    player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryGame {
    gameCreation: u64,
    gameCreationDate: String,
    gameDuration: u32,
    gameId: u64,
    gameMode: String,
    gameType: String,
    gameVersion: String,
    mapId: u16,
    participantIdentities: Vec<LolMatchHistoryMatchHistoryParticipantIdentities>,
    participants: Vec<LolMatchHistoryMatchHistoryParticipant>,
    platformId: String,
    queueId: i32,
    seasonId: u16,
    teams: Vec<LolMatchHistoryMatchHistoryTeam>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryGameList {
    gameBeginDate: String,
    gameCount: u64,
    gameEndDate: String,
    gameIndexBegin: u64,
    gameIndexEnd: u64,
    games: Vec<LolMatchHistoryMatchHistoryGame>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryList {
    accountId: u64,
    games: 	LolMatchHistoryMatchHistoryGameList,
    platformId: String
}

#[tokio::main]
async fn main() {
    let client = shaco::rest::RESTClient::new().expect("Failed to create client.");

    let endpoint = "/lol-match-history/v1/products/lol/current-summoner/matches?endIndex=100";

    let matches: LolMatchHistoryMatchHistoryList = serde_json::from_value(client.get(endpoint.to_string()).await.expect("Failed to fetch from endpoint.").clone()).unwrap();

    for match_history in matches.games.games {
        if (match_history.gameMode == "CLASSIC") & (match_history.gameType == "CUSTOM_GAME") & (match_history.mapId == 11) {
            let gameId = match_history.gameId;
            let endpoint = format!("/lol-match-history/v1/games/{gameId}");
            let full_match_history: LolMatchHistoryMatchHistoryGame = serde_json::from_value(client.get(endpoint.to_string()).await.expect("Failed to fetch from endpoint.").clone()).unwrap();
            if full_match_history.participantIdentities.len() == 10 {
                let full_match_history_string = serde_json::to_string(&full_match_history).unwrap();
                fs::write(format!("./{gameId}.json"), full_match_history_string).expect("Unable to write file");
            }
        }
    }
}
