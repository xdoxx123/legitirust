use serde::de::DeserializeOwned;

use crate::{Error, types::{Player, Stats, World}};


pub struct Client {
    req_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self { req_client: reqwest::Client::new() }
    }
    // yummy generics
    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, crate::Error> {
            let resp = self.req_client.get(url).send().await?;
            Ok(resp.json::<T>().await?)
    }
    async fn getworldbyurl(&self, url:&str) -> Result<World,crate::Error> {
        self.get(url).await
    }
    async fn getworldsbyurl(&self, url:&str) -> Result<Vec<World>,crate::Error> {
        self.get(url).await
    }
    
    // Result<World,crate::Error> 
    pub async  fn get_world_by_uuid(&self,uuid:&str) -> Result<World,crate::Error>  {
        self.getworldbyurl(format!("https://api.legiti.dev/world/{uuid}").as_str()).await
    }
    pub async fn get_all_worlds(&self) -> Result<Vec<World>,crate::Error> {
        self.getworldsbyurl("https://api.legiti.dev/all").await
    }
    
    pub async fn get_worlds_by_index(&self,index:u32) -> Result<Vec<World>, Error> {
        self.getworldsbyurl(format!( "https://api.legiti.dev/index/{index}").as_str()).await
    }
    
    pub async fn get_top_worlds_by_number(&self,maxamount:u32) -> Result<Vec<World>,Error> {
        self.getworldsbyurl(format!( "https://api.legiti.dev/top/{maxamount}").as_str()).await
    }
    // Helper cuz why not?
    pub async fn get_all_top_worlds(&self) -> Result<Vec<World>,Error> {
        self.getworldsbyurl("https://api.legiti.dev/top/0").await
    }
    pub async fn search_worlds(&self,query:&str) -> Result<Vec<World>,Error> {
        self.getworldsbyurl(format!("https://api.legiti.dev/search/{query}").as_str()).await
    }
    pub async fn get_player_worlds_by_uuid(&self,uuid:&str) -> Result<Vec<World>,Error> {
        self.getworldsbyurl(format!("https://api.legiti.dev/owner/{uuid}").as_str()).await
    }
    pub async fn get_players(&self) -> Result<Vec<Player>,Error> {
        self.get("https://api.legiti.dev/player").await
    }
    pub async fn get_stats(&self) -> Result<Vec<Stats>,Error> {
        self.get("https://api.legiti.dev/player").await
    }
    

}