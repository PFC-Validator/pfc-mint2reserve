use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct NewNFTRequest {
    pub name: String,
    pub meta: String,
    pub svg: String,
    pub ipfs_image: String,
    pub ipfs_meta: String,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct TerraPeepMeta {
    pub name: String,
    pub token_uri: String,
    pub image: String,
    pub attributes: Vec<Trait>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}
impl TerraPeepMeta {
    pub(crate) fn read_meta(file_name: &Path) -> anyhow::Result<Self> {
        let file_path = File::open(file_name)?;
        let peep: TerraPeepMeta = serde_json::from_reader(file_path)?;
        Ok(peep)
    }
}
