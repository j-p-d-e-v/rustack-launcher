use serde::{Deserialize};
use serde_envfile::{Error};
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use std::collections::HashMap;
use std::any::Any;

#[derive(Deserialize, Debug, PartialEq)]
struct EnvironmentVar {
    name: String,
    value: String
}


#[derive(Deserialize, Debug)]
struct Common {
    name: String,
    author: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Service {
    name: String,
    description: String,
    image: String,
    volumes: Vec<String>,
    env_vars: Vec<EnvironmentVar>,
    env_files: Vec<String>
}

#[derive(Deserialize, Debug)]
struct Config {
    common: Common,
    services: Vec<Service>,
    env_files: Vec<EnvironmentFile>
}

#[derive(Deserialize, Debug)]
struct EnvironmentFile {
    name: String,
    description: String,
    values: Vec<EnvironmentVar>

}

impl Config {
    pub fn load(path: String) -> Self {
        let data = read_to_string(path).expect("Unable to load config.toml file.");
        toml::from_str(data.as_str()).expect("Unable to parse toml file.")
    }

    pub fn to_compose_yaml(self) -> bool {
        todo!("Generate a docker-compose yaml file.")
    }
}

impl EnvironmentFile {
    fn write(env_file: EnvironmentFile ) -> Result<(),Error> {
        let file_name: String  = format!(".{}.env",env_file.name);
        let mut values: HashMap<String,String> = HashMap::new();
        let file_path: PathBuf = PathBuf::from(file_name);

        for item in env_file.values {
            values.insert(item.name,item.value);
        }

        serde_envfile::to_file(&file_path,&values)?;
        Ok(())
    }
}


#[cfg(test)]
mod test_envfile {
    use crate::EnvironmentFile;
    use crate::EnvironmentVar;

    #[test]
    fn test_write_env_file() {
        let data = EnvironmentFile {
            
            name: String::from("test"),
            description: String::from("This is a test env file."),
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
        //Problem found is we need to implement Any to make sure it writes string for text values and int for numeric values
        println!("{:#?}",data);
        EnvironmentFile::write(data);

    }
}


#[cfg(test)]
mod test_config{
    use crate::Config;
    use crate::Service;

    #[test]
    fn load_config(){
        let config = Config::load("config-test.toml".to_string());
        let services: Vec<Service> = config.services;
        assert!(!config.common.name.is_empty() );
        assert!(!config.common.author.is_empty() );
        assert!(!config.common.description.is_empty() );
        for service in services {
            assert!(!service.name.is_empty());
            assert!(!service.description.is_empty());
            assert!(!service.image.is_empty());
            assert!(service.volumes.len() > 0);
            assert!(service.env_vars.len() > 0);
            assert!(service.env_files.len() > 0);
            println!("service:{:#?}",service);

        }
    }

    //#[test]
    //#[ignore]
    //fn test_write_yaml(){
    //    use crate::People;
    //    use crate::Person;
    //    use std::fs::File;
    //    use crate::Friend;
    //    if let Ok(buffer) = File::create("test.yaml") {
    //        let ferson = Person { 
    //            name: String::from("Juan"), 
    //            age: 13,
    //            friends: Vec::from([
    //                Friend {                        
    //                    name: String::from("Jose"), 
    //                    age: 14,
    //                    friends: Vec::from([
    //                        Friend {                        
    //                            name: String::from("Jose"), 
    //                            age: 14,
    //                            friends: vec![],
    //                        },
    //                        Friend {                        
    //                            name: String::from("Bruno"), 
    //                            age: 15,
    //                            friends: vec![],
    //                        },
    //                    ])
    //                },
    //                Friend {                        
    //                    name: String::from("Bruno"), 
    //                    age: 15,
    //                    friends: Vec::from([
    //                        Friend {                        
    //                            name: String::from("Jose"), 
    //                            age: 14,
    //                            friends: vec![],
    //                        },
    //                        Friend {                        
    //                            name: String::from("Bruno"), 
    //                            age: 15,
    //                            friends: vec![],
    //                        },
    //                    ])
    //                },
    //            ])
    //        };
    //        let people: People = People { persons: Vec::from([ferson])};
    //        if let Ok(p) = serde_yaml::to_writer(buffer,&people) {
    //            println!("{:#?}",p);
    //        }
    //    }
    //}
}