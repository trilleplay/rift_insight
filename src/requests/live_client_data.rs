use super::Methods;
use crate::errors::RiftApiRequestError;
use log::{debug};

use reqwest::Client;
use serde::{Deserialize, de::DeserializeOwned};
use std::{collections::HashMap, vec::Vec};


#[derive(Deserialize, Debug)]
pub struct AllGameData {
    #[serde(rename = "activePlayer")]
    active_player: ActivePlayer,
    #[serde(rename = "allPlayers")]
    all_players: Vec<Player>,
    events: EventInAllGameData,
    #[serde(rename = "gameData")]
    game_data: GameData
}

#[derive(Deserialize, Debug)]
pub struct EventInAllGameData {
    #[serde(rename = "Events")]
    events: Vec<Event>
}

#[derive(Deserialize, Debug)]
pub struct GameData {
    #[serde(rename = "gameMode")]
    game_mode: String,
    #[serde(rename = "gameTime")]
    game_time: f64,
    #[serde(rename = "mapName")]
    map_name: String,
    #[serde(rename = "mapNumber")]
    map_number: i32,
    #[serde(rename = "mapTerrain")]
    map_terrain: String
}

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "EventID")]
    event_id: i64,
    #[serde(rename = "EventName")]
    event_name: String,
    #[serde(rename = "EventTime")]
    event_time: f64
}

#[derive(Deserialize, Debug)]
pub struct ActivePlayer {
    abilities: HashMap<String, Ability>,
    #[serde(rename = "championStats")]
    champion_stats: ChampionStats,
    #[serde(rename = "currentGold")]
    current_gold: f64,
    #[serde(rename = "fullRunes")]
    full_runes: Runes,
    level: i8,
    #[serde(rename = "summonerName")]
    summoner_name: String
}

#[derive(Deserialize, Debug)]
pub struct Player {
    #[serde(rename = "championName")]
    champion_name: String,
    #[serde(rename = "isBot")]
    is_bot: bool,
    #[serde(rename = "isDead")]
    is_dead: bool,
    items: Vec<Item>,
    level: i32,
    position: String,
    #[serde(rename = "rawChampionName")]
    raw_champion_name: String,
    #[serde(rename = "respawnTimer")]
    respawn_timer: f64,
    runes: Runes,
    scores: Scores,
    #[serde(rename = "skinID")]
    skin_id: i64,
    #[serde(rename = "summonerName")]
    summoner_name: String,
    #[serde(rename = "summonerSpells")]
    summoner_spells: SummonerSpells,
    team: String
}

#[derive(Deserialize, Debug)]
pub struct SummonerSpells {
    #[serde(rename = "summonerSpellOne")]
    summoner_spell_one: SummonerSpell,
    #[serde(rename = "summonerSpellTwo")]
    summoner_spell_two: SummonerSpell
}

#[derive(Deserialize, Debug)]
pub struct SummonerSpell {
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "rawDescription")]
    raw_description: String,
    #[serde(rename = "rawDisplayName")]
    raw_display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Scores {
    assists: i64,
    #[serde(rename = "creepScore")]
    creep_score: i64,
    #[serde(rename = "deaths")]
    deaths: i64,
    #[serde(rename = "kills")]
    kills: i64,
    #[serde(rename = "wardScore")]
    ward_score: f64
}

#[derive(Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "canUse")]
    can_use: bool,
    consumable: bool,
    count: i32,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "itemID")]
    item_id: i32,
    price: i32,
    #[serde(rename = "rawDescription")]
    raw_description: String,
    #[serde(rename = "rawDisplayName")]
    raw_display_name: String,
    slot: i8
}

#[derive(Deserialize, Debug)]
pub struct Ability {
    #[serde(rename = "abilityLevel")]
    ability_level: Option<i32>,
    #[serde(rename = "displayName")]
    display_name: String,
    id: String,
    #[serde(rename = "rawDescription")]
    raw_description: String,
    #[serde(rename = "rawDisplayName")]
    raw_display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct ChampionStats {
    #[serde(rename = "abilityPower")]
    ability_power: Option<f64>,
    armor: f64,
    #[serde(rename = "armorPenetrationFlat")]
    armor_penetration_flat: f64,
    #[serde(rename = "armorPenetrationPercent")]
    armor_penetration_percent: f64,
    #[serde(rename = "attackDamage")]
    attack_damage: f64,
    #[serde(rename = "attackRange")]
    attack_range: f64,
    #[serde(rename = "attackSpeed")]
    attack_speed: f64,
    #[serde(rename = "bonusArmorPenetrationPercent")]
    bonus_armor_penetration_percent: f64,
    #[serde(rename = "bonusMagicPenetrationPercent")]
    bonus_magic_penetration_percent: f64,
    #[serde(rename = "cooldownReduction")]
    cooldown_reduction: Option<f64>,
    #[serde(rename = "critChance")]
    crit_chance: Option<f64>,
    #[serde(rename = "critDamage")]
    crit_damage: Option<f64>,
    #[serde(rename = "currentHealth")]
    current_health: f64,
    #[serde(rename = "healthRegenRate")]
    health_regen_rate: f64,
    #[serde(rename = "lifeSteal")]
    life_steal: Option<f64>,
    #[serde(rename = "magicLethality")]
    magic_lethality: Option<f64>,
    #[serde(rename = "magicPenetrationFlat")]
    magic_penetration_flat: Option<f64>,
    #[serde(rename = "magicPenetrationPercent")]
    magic_penetration_percent: Option<f64>,
    #[serde(rename = "magicResist")]
    magic_resist: Option<f64>,
    #[serde(rename = "maxHealth")]
    max_health: f64,
    #[serde(rename = "moveSpeed")]
    move_speed: f64,
    #[serde(rename = "physicalLethality")]
    physical_lethality: Option<f64>,
    #[serde(rename = "resourceMax")]
    resource_max: f64,
    #[serde(rename = "resourceRegenRate")]
    resource_regen_rate: f64,
    #[serde(rename = "resourceType")]
    resource_type: String,
    #[serde(rename = "resourceValue")]
    resource_value: f64,
    #[serde(rename = "spellVamp")]
    spell_vamp: Option<f64>,
    tenacity: Option<f64>
}

#[derive(Deserialize, Debug)]
pub struct Runes {
    #[serde(rename = "generalRunes")]
    general_runes: Option<Vec<Rune>>,
    keystone: Rune,
    #[serde(rename = "primaryRuneTree")]
    primary_rune_tree: Rune,
    #[serde(rename = "secondaryRuneTree")]
    secondary_rune_tree: Rune,
    #[serde(rename = "statRunes")]
    stat_runes: Option<Vec<StatRune>>
}

#[derive(Deserialize, Debug)]
pub struct Rune {
    #[serde(rename = "displayName")]
    display_name: String,
    id: i32,
    #[serde(rename = "rawDescription")]
    raw_description: String,
    #[serde(rename = "rawDisplayName")]
    raw_display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct StatRune {
    id: i32,
    #[serde(rename = "rawDescription")]
    raw_description: String,
}

pub struct LiveClientData {
    pub http: Client
}

impl LiveClientData {
    pub fn new() -> LiveClientData {
        let http = Client::builder()
            .add_root_certificate(super::security::get_certificate())
            .build()
            .unwrap();
        return LiveClientData { http: http};
    }

    async fn request<T>(&mut self, method: Methods, path: &str) -> Result<T, RiftApiRequestError> where T: DeserializeOwned {
        let url = format!("https://127.0.0.1:2999/{}", path);
        debug!("Constructed url for {:?} request: {}", method, url);
        let response = self.http.request(method.as_reqwest(), url).send().await;
        return match response {
            Ok(r) => {
                let parsed = r.json::<T>().await;
                match parsed {
                    Ok(p) => Ok(p),
                    Err(e) => Err(RiftApiRequestError::new(e)),
                }
            }
            Err(e) => {
                Err(RiftApiRequestError::new(e))
            }
        }
    }

    pub async fn get_all_gamedata(&mut self) -> Result<AllGameData, RiftApiRequestError> {
        return self.request::<AllGameData>(Methods::Get, "liveclientdata/allgamedata").await;
    }

    pub async fn get_active_player(&mut self) -> Result<ActivePlayer, RiftApiRequestError> {
        return self.request::<ActivePlayer>(Methods::Get, "liveclientdata/activeplayer").await;
    }

    pub async fn get_event_data(&mut self) -> Result<EventInAllGameData, RiftApiRequestError> {
        return self.request::<EventInAllGameData>(Methods::Get, "liveclientdata/eventdata").await;
    }

    pub async fn get_game_stats(&mut self) -> Result<GameData, RiftApiRequestError> {
        return self.request::<GameData>(Methods::Get, "liveclientdata/gamestats").await;
    }
}