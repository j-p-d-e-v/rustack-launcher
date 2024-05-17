use crate::generators::env::EnvironmentFile;
use crate::generators::config::Config;

#[test]
fn test_env_file(){
    let config = Config::load("config-test-podman.toml".to_string());
    let deploy_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.deploy_dir);
    config.validate();
    let env_file_paths: Vec<String> = EnvironmentFile::generate(&config.env_files,&deploy_dir);
    assert_eq!(env_file_paths.len()>0,true);
}