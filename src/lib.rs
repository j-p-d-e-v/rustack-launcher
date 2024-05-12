use serde::{Deserialize, Serialize};
use serde_envfile::{Error as SerdeEnvFileError};
use serde_yaml::{Mapping, Value, Error as SerdeYamlError};
use std::fs::{read_to_string, remove_dir_all, create_dir, File};
use std::path::{ PathBuf, Path };
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::any::Any;

fn is_vec_empty(n: &Vec<String>) -> bool {
    n.len() == 0
}
fn is_hashmap_empty(n: &HashMap<String, String>) -> bool {
    n.is_empty()
}
fn is_compose_networks_empty(n: &HashMap<String, Network>) -> bool {
    n.is_empty()
}
fn is_compose_volumes_empty(n: &HashMap<String, Volume>) -> bool {
    n.is_empty()
}
fn is_service_volumes_empty(n: &Vec<ServiceVolume>) -> bool {
    n.is_empty()
}
fn is_string_empty(n: &String) -> bool {
    n.is_empty()
}

#[derive(Deserialize, Serialize, Debug)]
struct Common {
    name: String,
    author: String,
    description: String,
    deploy_dir: String
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Compose {
    services: HashMap<String,Service>,
    #[serde(skip_serializing_if = "is_compose_networks_empty")]
    networks: HashMap<String,Network>,
    #[serde(skip_serializing_if = "is_compose_volumes_empty")]
    volumes: HashMap<String,Volume>
}

#[derive(Deserialize,Serialize, Debug)]
struct ServiceVolume {  
    #[serde(rename(deserialize = "kind",serialize = "type"))]
    kind: String,
    source: String,
    target: String,
    #[serde(default)]
    read_only: bool
}

#[derive(Deserialize,Serialize, Debug)]
struct Service {
    hostname: String,
    image: String,
    environment: HashMap<String, String>,
    env_file: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_vec_empty")]
    networks: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_service_volumes_empty")]
    volumes: Vec<ServiceVolume>
}

#[derive(Deserialize, Serialize, Debug )]
struct Network {
    name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_string_empty")]
    driver: String,
    #[serde(default)]
    external: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_hashmap_empty")]
    labels: HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Volume {
    name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_string_empty")]
    driver: String,
    #[serde(default)]
    external: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_hashmap_empty")]
    labels: HashMap<String, String>
}


#[derive(Deserialize, Serialize, Debug)]
struct Config {
    common: Common,
    services: Vec<Service>,
    env_files: Vec<EnvironmentFile>,
    networks: Vec<Network>,
    volumes: Vec<Volume>
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct EnvironmentVar {
    name: String,
    value: String
}

#[derive(Deserialize,Serialize,  Debug)]
struct EnvironmentFile {
    name: String,
    values: Vec<EnvironmentVar>
}

impl Config {
    pub fn load(path: String) -> Self {
        let data = read_to_string(path).expect("Unable to load config.toml file.");
        match toml::from_str::<Config>(data.as_str()) {
            Ok(config) => {
                let deploy_dir: String = config.common.deploy_dir.clone();
                
                match remove_dir_all(&deploy_dir) {
                    Ok(_) => {
                        create_dir(&deploy_dir).expect("Unabel to create directory");
                    },
                    Err(error) => {
                        if error.kind() == std::io::ErrorKind::NotFound {
                            create_dir(&deploy_dir).expect("Unabel to create directory");
                        }
                        else{
                            panic!("Unable to remove deploy dir: {:?}",error);
                        }
                    }
                };
                config
            }
            Err(error) => panic!("Unable to parse toml file. {}",error)
        }
    }

    pub fn validate(&self) {
        let networks = &self.networks;
        let env_files = &self.env_files;

        for service in &self.services {
            let service_hostname: &String = &service.hostname;
            let service_networks: &Vec<String> = &service.networks;
            let service_env_files: &Vec<String> = &service.env_file;
            
            //Validate the networks declared in a service to the network configurations.
            for service_network in service_networks {
                if networks.into_iter().find( |&n| &n.name == service_network).is_none() {
                    panic!("Unable to find network {} of service named {} in the list of networks.",service_network,service_hostname);
                }
            }

            //Validate the environment files.
            for service_env_file in service_env_files {
                if env_files.into_iter().find( |&n| &n.name == service_env_file).is_none() {
                    panic!("Unable to find environment file {} of service named {} in the list of environment files.",service_env_file,service_hostname);
                }
            }
        }
    }
}

impl EnvironmentFile {
    fn write(env_file: &EnvironmentFile, deploy_dir: &String ) -> Result<(),SerdeEnvFileError> {
        let file_name: String  = format!(".{}.env",env_file.name);
        let mut values: HashMap<String, String> = HashMap::new();
        let _file_path = format!("{}/{}",deploy_dir,file_name);
        let file_path: PathBuf = PathBuf::from(_file_path);

        for item in &env_file.values {
            values.insert(item.name.clone(),item.value.clone());
        }

        serde_envfile::to_file(&file_path,&values)?;
        Ok(())
    }
}

impl Compose {
    fn generate(config: Config) -> Compose {
        let mut compose = Self::default();
        for service in config.services {
            let hostname: String = service.hostname.clone();
            compose.insert_service(hostname,service);
        }
        for network in config.networks {
            let network_name: String = network.name.clone();
            compose.insert_network(network_name,network);
        }
        for volume in config.volumes {
            let volume_name: String = volume.name.clone();
            compose.insert_volume(volume_name,volume);
        }
        compose
    }
    fn insert_service(&mut self, name: String,mut data: Service) {
        data.env_file = data.env_file.into_iter().map(|file| format!(".{}.env",file) ).collect::<Vec<String>>().to_vec();
        self.services.insert(name,data);
    }
    fn insert_network(&mut self, name: String,data: Network){
        self.networks.insert(name,data);
    }
    fn insert_volume(&mut self, name: String,data: Volume){
        self.volumes.insert(name,data);
    }
    fn write(&self, file_name: String, deploy_dir: &String) -> Result<(),Box<dyn Error>> {
        let compose_file: String = serde_yaml::to_string(&self)?;
        let file_path = format!("{}/{}",deploy_dir,file_name);
        let mut f = File::create(file_path)?;
        f.write(&compose_file.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod test_envfile {
    use crate::EnvironmentFile;
    use crate::EnvironmentVar;
    use serde_envfile::{Error};

    //#[test]
    //#[ignore]
    //fn test_write_env_file() -> Result<(),Error> {
    //    let data = EnvironmentFile {
    //        
    //        name: String::from("test"),
    //        values: Vec::from([
    //            EnvironmentVar {
    //                name: "MYVAR1".to_string(),
    //                value: "MyVAL1".to_string()
    //            },
    //            EnvironmentVar {
    //                name: "MYVAR2".to_string(),
    //                value: "MyVAL2".to_string()
    //            }
    //        ])
    //    };
    //    EnvironmentFile::write(data)?;
    //    Ok(())
//
    //}
}


#[cfg(test)]
mod test_config{
    use crate::Config;
    use crate::Compose;
    use crate::EnvironmentFile;

    #[test]
    fn load_config() {
        let config = Config::load("config-test.toml".to_string());
        let deploy_dir = config.common.deploy_dir.clone();
        config.validate();
        
        println!("Deploy Dir: {}",&deploy_dir);
        //Write An Env File
        for env_file in &config.env_files {
            EnvironmentFile::write(&env_file,&deploy_dir).unwrap();
        }
        let compose : Compose = Compose::generate(config);
        compose.write("docker-compose-test.yaml".to_string(),&deploy_dir).unwrap();
    }
}