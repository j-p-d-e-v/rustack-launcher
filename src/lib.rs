use serde::{Deserialize, Serialize};
use serde_envfile::{Error as SerdeEnvFileError};
use serde_yaml::{Mapping, Value, Error as SerdeYamlError};
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;

fn is_vec_empty(n: &Vec<String>) -> bool {
    n.len() == 0
}

#[derive(Deserialize, Serialize, Debug)]
struct Common {
    name: String,
    author: String,
    description: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Compose {
    services: HashMap<String,Service>,
    networks: HashMap<String,Network>
}

#[derive(Deserialize,Serialize, Debug)]
struct Service {
    hostname: String,
    image: String,
    volumes: Vec<String>,
    environment: HashMap<String, String>,
    env_file: Vec<String>,
    #[serde(skip_serializing_if = "is_vec_empty")]
    networks: Vec<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Network {
    name: String,
    driver: String,
    #[serde(default)]
    external: bool
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    common: Common,
    services: Vec<Service>,
    env_files: Vec<EnvironmentFile>,
    networks: Vec<Network>
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
        toml::from_str(data.as_str()).expect("Unable to parse toml file.")
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
                println!("{:?}",service_env_file);
                if env_files.into_iter().find( |&n| &n.name == service_env_file).is_none() {
                    panic!("Unable to find environment file {} of service named {} in the list of environment files.",service_env_file,service_hostname);
                }
            }
        }
    }
}

impl EnvironmentFile {
    fn write(env_file: EnvironmentFile ) -> Result<(),SerdeEnvFileError> {
        let file_name: String  = format!(".{}.env",env_file.name);
        let mut values: HashMap<String, String> = HashMap::new();
        let file_path: PathBuf = PathBuf::from(file_name);

        for item in env_file.values {
            values.insert(item.name.into(),item.value);
        }

        serde_envfile::to_file(&file_path,&values)?;
        Ok(())
    }
}

impl Compose {
    fn insert_service(&mut self, name: String,mut service: Service) {

        service.env_file = service.env_file.into_iter().map(|file| format!(".{}.env",file) ).collect::<Vec<String>>().to_vec();

        self.services.insert(name,service);
    }
    fn insert_network(&mut self, name: String,network: Network){
        self.networks.insert(name,network);
    }
    fn write(&self, path: String) -> Result<(),Box<dyn Error>> {
        let compose_file: String = serde_yaml::to_string(&self)?;
        let mut f = File::create(path)?;
        f.write(&compose_file.as_bytes())?;
        Ok(())
    }
}
/* 
use std::option::Option::None;
#[test]
fn test_poc(){
    let data: HashMap<String,Option<String>> = HashMap::from([
        (String::from("mykey1"),None),
        (String::from("mykey2"),None)
    ]);
    let mut file = File::create("test-poc.yaml").unwrap();
    let yaml = serde_yaml::to_string(&data).unwrap();
    file.write_all(yaml.replace("null","").as_bytes()).unwrap();
}
*/

#[cfg(test)]
mod test_envfile {
    use crate::EnvironmentFile;
    use crate::EnvironmentVar;
    use serde_envfile::{Error};

    #[test]
    #[ignore]
    fn test_write_env_file() -> Result<(),Error> {
        let data = EnvironmentFile {
            
            name: String::from("test"),
            values: Vec::from([
                EnvironmentVar {
                    name: "MYVAR1".to_string(),
                    value: "MyVAL1".to_string()
                },
                EnvironmentVar {
                    name: "MYVAR2".to_string(),
                    value: "MyVAL2".to_string()
                }
            ])
        };
        EnvironmentFile::write(data)?;
        Ok(())

    }
}


#[cfg(test)]
mod test_config{
    use crate::Config;
    use crate::Compose;
    use crate::EnvironmentFile;

    #[test]
    fn load_config() {
        let mut compose : Compose = Compose::default();
        let config = Config::load("config-test.toml".to_string());
        config.validate();
        assert!(!config.common.name.is_empty() );
        assert!(!config.common.author.is_empty() );
        assert!(!config.common.description.is_empty() );
        for service in config.services {
            assert!(!service.hostname.is_empty());
            assert!(!service.image.is_empty());
            assert!(service.volumes.len() > 0);
            assert!(service.environment.len() > 0);
            assert!(service.env_file.len() > 0);
            let hostname: String = service.hostname.clone();
            compose.insert_service(hostname,service);
        }
        for network in config.networks {
            let network_name: String = network.name.clone();
            compose.insert_network(network_name,network);
        }
        compose.write("docker-compose-test.yaml".to_string()).unwrap();
//
        ////Write An Env File
        //for env_file in config.env_files {
        //    EnvironmentFile::write(env_file).unwrap();
        //}
        ////config.write_yaml().unwrap();
    }
}