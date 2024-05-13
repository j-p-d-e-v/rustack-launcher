use crate::generators::prelude::{Repository};


#[test]
pub fn test_repository_clone(){

    let data: Vec<Repository> = Vec::from([
        Repository {
            service: String::from("service"),
            mount_target: String::from("/var/test"),
            name: String::from("test"),
            url: "https://github.com/j-p-d-e-v/embedded-rust-led-roulette".to_string(),
            branch: "dev".to_string(),
            clone: true
        }
    ]);
    for item in &data {
        assert_eq!(!Repository::git_clone(&item.name,&item.url,&item.branch,&String::from("services")).len() > 0,true);

    }
}