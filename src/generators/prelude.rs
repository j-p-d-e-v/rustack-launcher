pub use serde::{Deserialize, Serialize};
pub use serde_envfile::{Error as SerdeEnvFileError};
pub use serde_yaml::{Mapping, Value, Error as SerdeYamlError};
pub use std::fs::{read_to_string, remove_dir_all, create_dir, File};
pub use std::path::{ PathBuf, Path };
pub use std::collections::HashMap;
pub use std::error::Error;
pub use std::io::Write;
pub use crate::generators::settings::{ Settings };
pub use crate::generators::env::{ EnvironmentFile, EnvironmentVar };
pub use crate::generators::compose::{ Compose, ServiceVolume, Service, Volume, Network };
pub use crate::generators::config::{ Config };
pub use crate::generators::repository::{ Repository };

///Check if a Vector of String is empty.
///Used for #[serde(skip_serializing_if="is_vec_empty")] 
pub fn is_vec_empty(n: &Vec<String>) -> bool {
    n.len() == 0
}
///Check if a HashMap of String,String is empty.
///Used for #[serde(skip_serializing_if="is_hashmap_empty")] 
pub fn is_hashmap_empty(n: &HashMap<String, String>) -> bool {
    n.is_empty()
}
///Check if a HashMap of String,Network is empty.
///Used for #[serde(skip_serializing_if="is_compose_networks_empty")] 
pub fn is_compose_networks_empty(n: &HashMap<String, Network>) -> bool {
    n.is_empty()
}
///Check if a Hashmap of String,Volume is empty.
///Used for #[serde(skip_serializing_if="is_compose_volumes_empty")] 
pub fn is_compose_volumes_empty(n: &HashMap<String, Volume>) -> bool {
    n.is_empty()
}
///Check if a vector of String,ServiceVolume is empty.
///Used for #[serde(skip_serializing_if="is_service_volumes_empty")] 
pub fn is_service_volumes_empty(n: &Vec<ServiceVolume>) -> bool {
    n.is_empty()
}
///Checks if a String is empty.
///Used for #[serde(skip_serializing_if="is_string_empty")] 
pub fn is_string_empty(n: &String) -> bool {
    n.is_empty()
}