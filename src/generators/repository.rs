use crate::generators::prelude::*;
use git2::{ErrorCode, build::RepoBuilder};

///The struct for the repository.
#[derive(Deserialize, Serialize, Debug)]
pub struct Repository {
    ///The target service it will be mounted.
    #[serde(default)]
    pub service: String,
    ///The mount path the source code will be placed.
    #[serde(default)]
    pub mount_target: String,
    ///The name of the directory
    pub name: String,
    ///The repository url.
    pub url: String,
    ///The branch to clone.
    pub branch: String,
    ///Tells the launcher where to clone the repository or not.
    pub clone: bool
}


impl Repository {

    /// Clone a repository.
    ///
    /// # Example
    /// 
    /// ```ignore
    /// let data: Vec<Repository> = Vec::from([
    ///     Repository {
    ///         service: String::from("service"),
    ///         mount_target: String::from("/var/test"),
    ///         name: String::from("test"),
    ///         url: "https://github.com/j-p-d-e-v/embedded-rust-led-roulette".to_string(),
    ///         branch: "dev".to_string(),
    ///         clone: true
    ///     }
    /// ]);
    /// for item in &data {
    ///     assert_eq!(!Repository::git_clone(&item.name,&item.url,&item.branch,&String::from("services")).len() > 0,true);
    /// }
    /// ```
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