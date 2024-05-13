use crate::generators::prelude::*;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Compose {
    pub services: HashMap<String,Service>,
    #[serde(skip_serializing_if = "is_compose_networks_empty")]
    pub networks: HashMap<String,Network>,
    #[serde(skip_serializing_if = "is_compose_volumes_empty")]
    pub volumes: HashMap<String,Volume>
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct ServiceVolume {  
    #[serde(rename(deserialize = "kind",serialize = "type"))]
    pub kind: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub read_only: bool
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct Service {
    pub hostname: String,
    pub image: String,
    pub environment: HashMap<String, String>,
    pub env_file: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_vec_empty")]
    pub networks: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_service_volumes_empty")]
    pub volumes: Vec<ServiceVolume>,
}

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
    pub fn generate(services: &mut Vec<Service>, networks: &Vec<Network>, volumes: &Vec<Volume>, repositories: &Vec<Repository>,  file_name: String, deploy_dir: &String, services_dir: &String) -> String {
        let mut compose = Self::default();
        
        for service in services {
            for repo in repositories.into_iter() {
                if repo.clone && repo.service == service.hostname {
                    let mount_source: String = Repository::git_clone(&repo.name,&repo.url,&repo.branch,services_dir);
                    service.volumes.push(
                        ServiceVolume::new(String::from("volume"),mount_source,repo.mount_target.clone(),false)
                    );
                }
            }
            compose.insert_service(service.hostname.clone(),service.clone());
        }
        for network in networks {
            let network_name: String = network.name.clone();
            compose.insert_network(network_name,network.clone());
        }
        for volume in volumes {
            let volume_name: String = volume.name.clone();
            compose.insert_volume(volume_name,volume.clone());
        }
        match Compose::write(compose,file_name, &deploy_dir) {
            Ok(file_path) => {
                file_path
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
    pub fn write(compose: Compose, file_name: String, deploy_dir: &String) -> Result<String,Box<dyn Error>> {
        let compose_file: String = serde_yaml::to_string(&compose)?;
        let file_path = format!("{}/{}",deploy_dir,file_name);
        let mut f = File::create(file_path.clone())?;
        f.write(&compose_file.as_bytes())?;
        Ok(file_path)
    }
}
