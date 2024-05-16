use crate::generators::env::{ EnvironmentFile };
use crate::generators::compose::{ Compose};
use crate::generators::config::{ Config };
use std::{thread, time};

#[test]
#[ignore]
fn test_docker() {
    let config = Config::load("config-test-docker.toml".to_string());
    let deploy_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.deploy_dir);
    config.validate();
    let env_file_paths: Vec<String> = EnvironmentFile::generate(&config.env_files,&deploy_dir);
    assert_eq!(env_file_paths.len()>0,true);
    let compose: Compose = Compose::new(config);
    assert!(compose.up());
    thread::sleep(time::Duration::from_secs(5));
    assert!(compose.down());
}

#[test]
fn test_podman() {
    let config = Config::load("config-test-podman.toml".to_string());
    let deploy_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.deploy_dir);
    config.validate();
    let env_file_paths: Vec<String> = EnvironmentFile::generate(&config.env_files,&deploy_dir);
    assert_eq!(env_file_paths.len()>0,true);
    let compose: Compose = Compose::new(config);
    assert!(compose.up());
    thread::sleep(time::Duration::from_secs(5));
    assert!(compose.down());
}