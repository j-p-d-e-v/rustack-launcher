use crate::generators::prelude::*;

/// The root struct of the compose file.
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Compose {
    pub services: HashMap<String,Service>,
    #[serde(skip_serializing_if = "is_compose_networks_empty")]
    pub networks: HashMap<String,Network>,
    #[serde(skip_serializing_if = "is_compose_volumes_empty")]
    pub volumes: HashMap<String,Volume>,
    #[serde(default)]
    #[serde(skip)]
    pub executable: String,
    #[serde(skip)]
    pub file: String,
    #[serde(skip)]
    pub detached: bool,
}

///Struct for volume under service.
#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct ServiceVolume {  
    #[serde(rename(deserialize = "kind",serialize = "type"))]
    pub kind: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub read_only: bool
}

///Struct for service
#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct Service {
    pub hostname: String,
    pub image: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub ports: Vec<String>,
    #[serde(default)]
    pub tty: bool,    
    #[serde(default)]
    #[serde(skip_serializing_if = "is_hashmap_empty")]
    pub environment: HashMap<String, String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub env_file: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub networks: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_service_volumes_empty")]
    pub volumes: Vec<ServiceVolume>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub depends_on: Vec<String>,    
    #[serde(default)]
    #[serde(skip_serializing_if = "is_string_empty")]
    pub restart: String,
}

/// Struct for networks
#[derive(Deserialize, Serialize, Debug , Clone)]
pub struct Network {
    pub  name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_string_empty")]
    pub driver: String,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_hashmap_empty")]
    pub labels: HashMap<String, String>
}

///Struct for volumes
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Volume {
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_string_empty")]
    pub driver: String,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_hashmap_empty")]
    pub labels: HashMap<String, String>
}

impl ServiceVolume {
    /// Create a service volume instance.
    pub fn new(kind: String, source: String, target: String, read_only: bool) -> Self{
        Self {
            kind: kind,
            source: source,
            target: target,
            read_only: read_only
        }
    }
}

impl Compose {
    ///
    ///Generate a compose file
    ///
    /// ```ignore
    /// use crate::generators::env::{ EnvironmentFile };
    /// use crate::generators::compose::{ Compose};
    /// use crate::generators::config::{ Config };
    ///
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
    pub fn new(mut config: Config) -> Self {
        let deploy_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.deploy_dir);
        let services_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.services_dir);
        Self { 
            executable: config.settings.compose_executable.clone(), 
            detached: config.settings.compose_detached, 
            ..Self::generate(
                &mut config.services,
                &config.networks,
                &config.volumes,
                &config.repositories, 
                config.settings.compose_file,
                &deploy_dir,
                &services_dir
            )
        }
    }
    pub fn generate(services: &mut Vec<Service>, networks: &Vec<Network>, volumes: &Vec<Volume>, repositories: &Vec<Repository>,  file_name: String, deploy_dir: &String, services_dir: &String) -> Self {
        let mut compose = Self::default();
        let mut services_repo_volumes: Vec<(String,ServiceVolume)> = Vec::new();
        
        for repo in repositories.into_iter() {
            if repo.clone {
                let mount_source: String = Repository::git_clone(&repo.name,&repo.url,&repo.branch,services_dir);
                let service_name: String = repo.service.clone();
                if !service_name.is_empty() {
                    services_repo_volumes.push(
                        (service_name,ServiceVolume::new(String::from("bind"),mount_source,repo.mount_target.clone(),false))
                    );
                }
            }
        }
        for service in services {
            let hostname = service.hostname.clone();
            for item in &services_repo_volumes {
                if item.0 ==  hostname {
                    service.volumes.push(item.1.clone());
                }
            }
            compose.insert_service(hostname,service.clone());
        }
        for network in networks {
            let network_name: String = network.name.clone();
            compose.insert_network(network_name,network.clone());
        }
        for volume in volumes {
            let volume_name: String = volume.name.clone();
            compose.insert_volume(volume_name,volume.clone());
        }
        match Compose::write(compose.clone(),file_name, &deploy_dir) {
            Ok(file_path) => {
                Compose { file: file_path.clone(), ..compose }
            }
            Err(error) => {
                panic!("Unable to generate compose file: {:?}.",error);
            }
        }
    }
    pub fn insert_service(&mut self, name: String,mut data: Service) {
        data.env_file = data.env_file.into_iter().map(|file| format!(".{}.env",file) ).collect::<Vec<String>>().to_vec();
        self.services.insert(name,data);
    }
    pub fn insert_network(&mut self, name: String,data: Network){
        self.networks.insert(name,data);
    }

    pub fn insert_volume(&mut self, name: String,data: Volume){
        self.volumes.insert(name,data);
    }
    ///Write the compose file.
    pub fn write(compose: Compose, file_name: String, deploy_dir: &String) -> Result<String,Box<dyn Error>> {
        let compose_file: String = serde_yaml::to_string(&compose)?;
        let file_path = format!("{}/{}",deploy_dir,file_name);
        let mut f = File::create(file_path.clone())?;
        f.write(&compose_file.as_bytes())?;
        Ok(file_path)
    }

    ///Execute the compose file.
    pub fn up(&self) -> bool {
        let mut args: Vec<String> = Vec::from([
            String::from("-f"),
            self.file.clone(),
            String::from("up")
        ]);
        if self.detached {
            args.push(String::from("-d"));
        }
        ExecuteCommand::run(self.executable.clone(),args);
        true
    }
    ///Terminates the running compose file.
    pub fn down(&self) -> bool {
        let args: Vec<String> = Vec::from([
            String::from("-f"),
            self.file.clone(),
            String::from("down")
        ]);
        ExecuteCommand::run(self.executable.clone(),args);
        true
    }
}