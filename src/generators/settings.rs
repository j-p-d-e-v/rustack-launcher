use crate::generators::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub name: String,
    pub author: String,
    pub description: String,
    pub base_dir: String,
    pub deploy_dir: String,
    pub services_dir: String,
}
