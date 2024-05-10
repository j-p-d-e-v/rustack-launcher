use serde::{Deserialize, Serialize};
use serde_envfile::{Error as SerdeEnvFileError};
use serde_yaml::{Mapping, Value, Error as SerdeYamlError};
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct EnvironmentVar {
    name: String,
    value: String
}


#[derive(Deserialize, Serialize, Debug)]
struct Common {
    name: String,
    author: String,
    description: String,
}

#[derive(Deserialize,Serialize, Debug)]
struct Service {
    name: String,
    image: String,
    volumes: Vec<String>,
    env_vars: Vec<EnvironmentVar>,
    env_files: Vec<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    common: Common,
    services: Vec<Service>,
    env_files: Vec<EnvironmentFile>
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

impl Service {
    fn to_yaml(service: Service) -> Result<String,Box<dyn Error>> {

        let mut mapping: Mapping = Mapping::new();
        if let Ok(service_name) = serde_yaml::from_str::<Value>(&service.name) {
            if let Ok(properties_str) = serde_yaml::to_string(&service) {
                let properties: Value = serde_yaml::from_str(properties_str);
                mapping.insert(service_name,properties);
                match serde_yaml::to_string(&mapping){
                    Ok(yaml) => Ok(yaml),
                    Err(error) => Error("Invalid service yaml mapping: {:?}",error)
                }
            }
            else{
                Error("No service properties.")
            }
        }
        else{
            Error("Unable to get service name.")
        }
    }
}

#[cfg(test)]
mod test_service {
    use serde_yaml::{Mapping, Value};
    use crate::Service;
    use crate::EnvironmentVar;
    use std::error::Error;

    #[test]
    fn test_generate_yaml() -> Result<(),Box<dyn Error>> {
        let data: Service = Service {
            name: String::from("test_service"),
            image: String::from("test:latest"),
            volumes: Vec::from([
                String::from("/app:/var/app"),
                String::from("/build:/var/build"),
            ]),
            env_vars: Vec::from([                
                EnvironmentVar {
                    name: "MYVAR1".to_string(),
                    value: "MyVAL1".to_string()
                },
                EnvironmentVar {
                    name: "MYVAR2".to_string(),
                    value: "MyVAL2".to_string()
                }
            ]),
            env_files: Vec::from([
                String::from("/env/test1.env"),
                String::from("/env/test2.env"),
            ])
        };
        let yaml = Service::to_yaml(data)?;
        println!("{:#?}",yaml);
        Ok(())
    }
}

#[cfg(test)]
mod test_envfile {
    use crate::EnvironmentFile;
    use crate::EnvironmentVar;
    use serde_envfile::{Error};

    #[test]
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
    use crate::Service;
    use crate::EnvironmentFile;

    #[test]
    fn load_config() {
        let config = Config::load("config-test.toml".to_string());
        assert!(!config.common.name.is_empty() );
        assert!(!config.common.author.is_empty() );
        assert!(!config.common.description.is_empty() );
        for service in &config.services {
            assert!(!service.name.is_empty());
            assert!(!service.image.is_empty());
            assert!(service.volumes.len() > 0);
            assert!(service.env_vars.len() > 0);
            assert!(service.env_files.len() > 0);
        }

        //Write An Env File
        for env_file in config.env_files {
            EnvironmentFile::write(env_file).unwrap();
        }
        //config.write_yaml().unwrap();
    }

    //#[test]
    //#[ignore]
    //fn test_write_yaml() -> Result<(), serde_yaml::Error>{
    //    use std::fs::File;
    //    #[derive(Debug,Deserialize, Serialize)]
    //    struct Person {
    //        name: String, 
    //        age: u32
    //    }
    //    let ferson = Person { 
    //        name: String::from("Juan"), 
    //        age: 13,
    //    };
    //    use serde_yaml::{Mapping, Value};
//
    //    let mut smapping: Mapping = Mapping::new();
    //    let key: Value = serde_yaml::from_str(&ferson.name).unwrap();
    //    let svalue = serde_yaml::to_string(&ferson).unwrap();
    //    let value: Value = serde_yaml::from_str(&svalue).unwrap();
//
    //    println!("{:?}",value);
    //    smapping.insert(key,value);
    //    println!("{:?}",smapping);
////
    //    if let Ok(buffer) = File::create("test.yaml") {
    //        serde_yaml::to_writer(buffer,&smapping)?;
    //    }
    //    Ok(())
    //}
}