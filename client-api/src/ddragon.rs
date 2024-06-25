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
        type_: String,
        format: String,
        version: String,
        data: HashMap<String, ChampionData>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct ChampionData {
        version: String,
        id: String,
        key: String,
        name: String,
        title: String,
        blurb: String,
        info: ChampionInfo,
        image: ChampionImage,
        tags: Vec<ChampionTag>,
        partype: ChampionResource,
        stats: ChampionStats,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct ChampionInfo {
        attack: u8,
        defense: u8,
        magic: u8,
        difficulty: u8,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default)]
    pub struct ChampionImage {
        full: String,
        sprite: String,
        group: String,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
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
        hp: f32,
        hpperlevel: f32,
        mp: f32,
        mpperlevel: f32,
        movespeed: f32,
        armor: f32,
        armorperlevel: f32,
        spellblock: f32,
        spellblockperlevel: f32,
        attackrange: f32,
        hpregen: f32,
        hpregenperlevel: f32,
        mpregen: f32,
        mpregenperlevel: f32,
        crit: f32,
        critperlevel: f32,
        attackdamage: f32,
        attackdamageperlevel: f32,
        attackspeedperlevel: f32,
        attackspeed: f32,
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
