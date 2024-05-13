use crate::generators::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
///The struct for the launcher settings.
pub struct Settings {
    ///The name of app launcher.
    pub name: String,
    ///The author of the launcher
    pub author: String,
    ///The description of the launcher.
    pub description: String,
    ///The base directory usually its the root directory of the launcher.
    pub base_dir: String,
    ///The directory where all the configurations will be saved.
    pub deploy_dir: String,
    ///The directory where all the git repositories will be cloned.
    pub services_dir: String,
}
