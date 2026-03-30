use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Debug,Serialize)]
pub struct World {
    #[serde(rename = "_id")]
    pub id: String,
    pub world_uuid: String,
    pub creation_date: String,
    pub creation_date_unix_seconds: u64,
    pub description: String,
    pub enforce_whitelist: bool,
    pub icon: String,
    pub last_scraped: u64,
    pub last_scraped_ms: Option<String>,
    pub locked: bool,
    pub max_players: u32,
    pub name: String,
    pub owner_name: Option<String>,
    pub owner_uuid: String,
    pub player_count: u32,
    pub version: String,
    pub visits: u32,
    pub votes: u32,
}
