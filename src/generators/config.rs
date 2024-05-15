use crate::generators::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
///The struct for the configuration(toml) file.
pub struct Config {
    pub settings: Settings,
    pub services: Vec<Service>,
    pub env_files: Vec<EnvironmentFile>,
    #[serde(default)]
    pub networks: Vec<Network>,
    #[serde(default)]
    pub volumes: Vec<Volume>,
    #[serde(default)]
    pub repositories: Vec<Repository>,
}

impl Config {
    /// loads the TOML file
    /// # Example
    /// ```ignore
    /// let mut config = Config::load("config-test.toml".to_string());
    /// let deploy_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.deploy_dir);
    /// let services_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.services_dir);
    /// let compose_file: String = String::from("docker-compose-test.yaml");
    /// config.validate();
    /// let env_file_paths: Vec<String> = EnvironmentFile::generate(&config.env_files,&deploy_dir);
    /// assert_eq!(env_file_paths.len()>0,true);
    /// let compose_file_path : String = Compose::generate(&mut config.services,&config.networks,&config.volumes,&config.repositories, compose_file,&deploy_dir,&services_dir);
    /// assert_eq!(!compose_file_path.is_empty(),true);
    /// ```
    pub fn load(config_path: String) -> Self {
        let data = read_to_string(config_path).expect("Unable to load config.toml file.");
        match toml::from_str::<Config>(data.as_str()) {
            Ok(config) => {
                let base_dir: &String = &config.settings.base_dir;
                let deploy_dir: String = format!("{}/{}",base_dir,&config.settings.deploy_dir);

                match remove_dir_all(&deploy_dir) {
                    Ok(_) => {
                        create_dir(&deploy_dir).expect("Unabel to create directory");
                    },
                    Err(error) => {
                        if error.kind() == std::io::ErrorKind::NotFound {
                            create_dir(&deploy_dir).expect("Unabel to create deploy directory");
                        }
                        else{
                            panic!("Unable to remove deploy dir: {:?}",error);
                        }
                    }
                };
                let services_dir: String = format!("{}/{}",base_dir,&config.settings.services_dir);

                if Path::new(&services_dir).try_exists().is_err() {
                    create_dir(&services_dir).expect("Unabel to create services directory");
                }
                config
            }
            Err(error) => panic!("Unable to parse toml file. {}",error)
        }
    }

    ///Validates the configuration file such as:
    /// - if a service uses a network it must be declared in the list of networks.
    /// - if a service uses a environment file, it must be declared in the list of environment files
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