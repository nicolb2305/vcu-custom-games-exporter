
#![allow(dead_code)]
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    pub championId: i32,
    pub pickTurn: u16
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryTeam {
    pub bans: Vec<LolMatchHistoryMatchHistoryTeamBan>,
    pub baronKills: u32,
    pub dominionVictoryScore: u32,
    pub dragonKills: u32,
    pub firstBaron: bool,
    pub firstBlood: bool,
    pub firstDargon: bool,
    pub firstInhibitor: bool,
    pub firstTower: bool,
    pub inhibitorKills: u32,
    pub riftHeraldKills: u32,
    pub teamId: u16,
    pub towerKills: u32,
    pub vilemawKills: u32,
    pub win: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryTimeline {
    pub creepsPerMinDeltas: Value,
    pub csDiffPerMinDeltas: Value,
    pub damageTakenDiffPerMinDeltas: Value,
    pub damageTakenPerMinDeltas: Value,
    pub goldPerMinDeltas: Value,
    pub lane: String,
    pub participantId: u16,
    pub role: String,
    pub xpDiffPerMinDeltas: Value,
    pub xpPerMinDeltas: Value
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryParticipantStatistics {
    pub assists: i64,
    pub causedEarlySurrender: bool,
    pub champLevel: i64,
    pub combatPlayerScore: i64,
    pub damageDealtToObjectives: i64,
    pub damageDealtToTurrets: i64,
    pub damageSelfMitigated: i64,
    pub deaths: i64,
    pub doubleKills: i64,
    pub earlySurrenderAccomplice: bool,
    pub firstBloodAssist: bool,
    pub firstBloodKill: bool,
    pub firstInhibitorAssist: bool,
    pub firstInhibitorKill: bool,
    pub firstTowerAssist: bool,
    pub firstTowerKill: bool,
    pub gameEndedInEarlySurrender: bool,
    pub gameEndedInSurrender: bool,
    pub goldEarned: i64,
    pub goldSpent: i64,
    pub inhibitorKills: i64,
    pub item0: u16,
    pub item1: u16,
    pub item2: u16,
    pub item3: u16,
    pub item4: u16,
    pub item5: u16,
    pub item6: u16,
    pub killingSprees: i64,
    pub kills: i64,
    pub largestCriticalStrike: i64,
    pub largestKillingSpree: i64,
    pub largestMultiKill: i64,
    pub longestTimeSpentLiving: i64,
    pub magicalDamageTaken: i64,
    pub magicDamageDealt: i64,
    pub magicDamageDealtToChampions: i64,
    pub neutralMinionsKilled: i64,
    pub neutralMinionsKilledEnemyJungle: i64,
    pub neutralMinionsKilledTeamJungle: i64,
    pub objectivePlayerScore: i64,
    pub participantId: u16,
    pub pentaKills: i64,
    pub perk0: i64,
    pub perk0Var1: i64,
    pub perk0Var2: i64,
    pub perk0Var3: i64,
    pub perk1: i64,
    pub perk1Var1: i64,
    pub perk1Var2: i64,
    pub perk1Var3: i64,
    pub perk2: i64,
    pub perk2Var1: i64,
    pub perk2Var2: i64,
    pub perk2Var3: i64,
    pub perk3: i64,
    pub perk3Var1: i64,
    pub perk3Var2: i64,
    pub perk3Var3: i64,
    pub perk4: i64,
    pub perk4Var1: i64,
    pub perk4Var2: i64,
    pub perk4Var3: i64,
    pub perk5: i64,
    pub perk5Var1: i64,
    pub perk5Var2: i64,
    pub perk5Var3: i64,
    pub perkPrimaryStyle: i64,
    pub perkSubStyle: i64,
    pub physicalDamageDealt: i64,
    pub physicalDamageDealtToChampions: i64,
    pub physicalDamageTaken: i64,
    pub playerScore0: i64,
    pub playerScore1: i64,
    pub playerScore2: i64,
    pub playerScore3: i64,
    pub playerScore4: i64,
    pub playerScore5: i64,
    pub playerScore6: i64,
    pub playerScore7: i64,
    pub playerScore8: i64,
    pub playerScore9: i64,
    pub quadraKills: i64,
    pub sightWardsBoughtInGame: i64,
    pub teamEarlySurrendered: bool,
    pub timeCCingOthers: i64,
    pub totalDamageDealt: i64,
    pub totalDamageDealtToChampions: i64,
    pub totalDamageTaken: i64,
    pub totalHeal: i64,
    pub totalMinionsKilled: i64,
    pub totalPlayerScore: i64,
    pub totalScoreRank: i64,
    pub totalTimeCrowdControlDealt: i64,
    pub totalUnitsHealed: i64,
    pub tripleKills: i64,
    pub trueDamageDealt: i64,
    pub trueDamageDealtToChampions: i64,
    pub trueDamageTaken: i64,
    pub turretKills: i64,
    pub unrealKills: i64,
    pub visionScore: i64,
    pub visionWardsBoughtInGame: i64,
    pub wardsKilled: i64,
    pub wardsPlaced: i64,
    pub win: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryParticipant {
    pub championId: i32,
    pub highestAchievedSeasonTier: String,
    pub participantId: u16,
    pub spell1Id: u16,
    pub spell2Id: u16,
    pub stats: LolMatchHistoryMatchHistoryParticipantStatistics,
    pub teamId: u16,
    pub timeline: LolMatchHistoryMatchHistoryTimeline
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    pub accountId: u64,
    pub currentAccountId: u64,
    pub currentPlatformId: String,
    pub matchHistoryUri: String,
    pub platformId: String,
    pub profileIcon: i32,
    pub summonerId: u64,
    pub summonerName: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    pub participantId: u16,
    pub player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryGame {
    pub gameCreation: u64,
    pub gameCreationDate: String,
    pub gameDuration: u32,
    pub gameId: u64,
    pub gameMode: String,
    pub gameType: String,
    pub gameVersion: String,
    pub mapId: u16,
    pub participantIdentities: Vec<LolMatchHistoryMatchHistoryParticipantIdentities>,
    pub participants: Vec<LolMatchHistoryMatchHistoryParticipant>,
    pub platformId: String,
    pub queueId: i32,
    pub seasonId: u16,
    pub teams: Vec<LolMatchHistoryMatchHistoryTeam>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryGameList {
    pub gameBeginDate: String,
    pub gameCount: u64,
    pub gameEndDate: String,
    pub gameIndexBegin: u64,
    pub gameIndexEnd: u64,
    pub games: Vec<LolMatchHistoryMatchHistoryGame>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LolMatchHistoryMatchHistoryList {
    pub accountId: u64,
    pub games: LolMatchHistoryMatchHistoryGameList,
    pub platformId: String
}