#[allow(clippy::missing_errors_doc)]
pub mod endpoints {
    use super::types::Champion;
    use crate::Error;
    use reqwest;

    pub async fn versions() -> Result<Vec<String>, Error> {
        Ok(
            reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json")
                .await?
                .json()
                .await?,
        )
    }

    pub async fn champion(patch: &str) -> Result<Champion, Error> {
        Ok(reqwest::get(format!(
            "https://ddragon.leagueoflegends.com/cdn/{patch}/data/en_US/champion.json"
        ))
        .await?
        .json()
        .await?)
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct Champion {
        #[serde(rename = "type")]
        pub type_: String,
        pub format: String,
        pub version: String,
        pub data: HashMap<String, ChampionData>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct ChampionData {
        pub version: String,
        pub id: String,
        pub key: String,
        pub name: String,
        pub title: String,
        pub blurb: String,
        pub info: ChampionInfo,
        pub image: ChampionImage,
        pub tags: Vec<ChampionTag>,
        pub partype: ChampionResource,
        pub stats: ChampionStats,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct ChampionInfo {
        pub attack: u8,
        pub defense: u8,
        pub magic: u8,
        pub difficulty: u8,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct ChampionImage {
        pub full: String,
        pub sprite: String,
        pub group: String,
        pub x: u32,
        pub y: u32,
        pub w: u32,
        pub h: u32,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub enum ChampionTag {
        Fighter,
        Mage,
        Assassin,
        Marksman,
        Tank,
        Support,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub enum ChampionResource {
        #[serde(alias = "")]
        None,
        #[serde(rename = "Blood Well")]
        BloodWell,
        #[default]
        Mana,
        Energy,
        Fury,
        Rage,
        Courage,
        Shield,
        Grit,
        Ferocity,
        Heat,
        #[serde(rename = "Crimson Rush")]
        CrimsonRush,
        Flow,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct ChampionStats {
        pub hp: f32,
        pub hpperlevel: f32,
        pub mp: f32,
        pub mpperlevel: f32,
        pub movespeed: f32,
        pub armor: f32,
        pub armorperlevel: f32,
        pub spellblock: f32,
        pub spellblockperlevel: f32,
        pub attackrange: f32,
        pub hpregen: f32,
        pub hpregenperlevel: f32,
        pub mpregen: f32,
        pub mpregenperlevel: f32,
        pub crit: f32,
        pub critperlevel: f32,
        pub attackdamage: f32,
        pub attackdamageperlevel: f32,
        pub attackspeedperlevel: f32,
        pub attackspeed: f32,
    }
}

#[cfg(test)]
mod tests {
    use crate::ddragon::endpoints::{champion, versions};

    #[tokio::test]
    async fn get_champions() {
        let versions = versions().await.unwrap();
        dbg!(&versions);
        let champions = champion(versions.first().unwrap()).await.unwrap();
        dbg!(champions);
    }
}
