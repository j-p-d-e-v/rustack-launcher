use crate::generators::config::Config;

#[test]
#[ignore]
fn test_docker() {
    let config = Config::load("config-test-docker.toml".to_string());
    config.validate();
    assert!(true);
}

#[test]
fn test_podman() {
    let config = Config::load("config-test-podman.toml".to_string());
    config.validate();
    assert!(true);
}