use serde::{Deserialize, Serialize};
use std::fs;

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
    environment_vars: Vec<EnvironmentVar>,
    environment_files: Vec<String>
}

#[derive(Deserialize, Debug)]
struct Config {
    common: Common,
    services: Vec<Service>,
}

impl Config {
    pub fn load(path: String) -> Self {
        let data = fs::read_to_string(path).expect("Unable to load config.toml file.");
        toml::from_str(data.as_str()).expect("Unable to parse toml file.")
    }

    pub fn to_compose_yaml(self) -> bool {
        todo!("Generate a docker-compose yaml file.")
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
            assert!(service.environment_vars.len() > 0);
            assert!(service.environment_files.len() > 0);
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