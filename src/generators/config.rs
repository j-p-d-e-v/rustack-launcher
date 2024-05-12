use crate::generators::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub common: Common,
    pub services: Vec<Service>,
    pub env_files: Vec<EnvironmentFile>,
    pub networks: Vec<Network>,
    pub volumes: Vec<Volume>
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