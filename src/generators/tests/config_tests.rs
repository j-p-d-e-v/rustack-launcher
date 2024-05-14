use crate::generators::env::{ EnvironmentFile };
use crate::generators::compose::{ Compose};
use crate::generators::config::{ Config };

#[test]
fn load_config() {
    let mut config = Config::load("config-test.toml".to_string());
    let deploy_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.deploy_dir);
    let services_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.services_dir);
    let compose_file: String = String::from("docker-compose-test.yaml");
    config.validate();
    let env_file_paths: Vec<String> = EnvironmentFile::generate(&config.env_files,&deploy_dir);
    assert_eq!(env_file_paths.len()>0,true);
    let compose_file_path : String = Compose::generate(&mut config.services,&config.networks,&config.volumes,&config.repositories, compose_file,&deploy_dir,&services_dir);
    assert_eq!(!compose_file_path.is_empty(),true);
    Compose::execute("docker-compose".to_string(),vec![
        String::from("-f"),
        compose_file_path,
        String::from("up")
    ]);
}