use crate::generators::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Common {
    pub name: String,
    pub author: String,
    pub description: String,
    pub deploy_dir: String
}
