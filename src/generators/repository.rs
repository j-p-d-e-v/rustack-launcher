use crate::generators::prelude::*;
use git2::{ErrorCode, build::RepoBuilder};

#[derive(Deserialize, Serialize, Debug)]
pub struct Repository {
    pub service: String,
    pub mount_target: String,
    pub name: String,
    pub url: String,
    pub branch: String,
    pub clone: bool
}


impl Repository {
    
    pub fn git_clone(name: &String, url: &String, branch: &String, services_dir: &String) -> String {
        let service_path: String = format!("{}/{}",services_dir,name);
        let mut repo_builder: RepoBuilder = RepoBuilder::new();
        match repo_builder.branch(branch).clone(url,Path::new(&service_path)) {
            Ok(_) => {
                println!("Successfully cloned {}({}) to {}",name,branch,&services_dir);
            }
            Err(error) => {
                if error.code() == ErrorCode::Exists {
                    println!("Skipped cloning {}({}) because it already exists.",&name,&service_path);
                }
                else{
                    panic!("Unable to clone {}: {:?}",name,error);
                }
            }
        }
        service_path.clone()
    }
}