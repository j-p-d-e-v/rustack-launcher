use crate::generators::env::{ EnvironmentFile };
use crate::generators::compose::{ Compose};
use crate::generators::config::{ Config };

#[test]
fn load_config() {
    let config = Config::load("config-test.toml".to_string());
    let deploy_dir: String = config.common.deploy_dir.clone();
    let compose_file: String = String::from("docker-compose-test.yaml");
    config.validate();
    let env_file_paths: Vec<String> = EnvironmentFile::generate(&config.env_files,&deploy_dir);
    assert_eq!(env_file_paths.len()>0,true);
    let compose_file_path : String = Compose::generate(config,compose_file,&deploy_dir);
    assert_eq!(!compose_file_path.is_empty(),true);
}