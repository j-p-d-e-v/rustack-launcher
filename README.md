# ruStack Launcher
A Rust library for developing application launchers. It uses a single TOML configuration file to automatically transform it into .env and YAML files. This library also allows you to easily switch between Docker Compose and Podman Compose.

```
NOTE: This is a personal project created for practicing Rust development. Feel free to modify this library as you see fit.
```

Crate: https://crates.io/crates/rustack-launcher

## Configuration File

The launcher takes a configuration using TOML. See [TOML](https://toml.io/en/) for the guidelines.

## Tables

### [settings]
Provides insights into the launcher functionalities alongside frequently used configuration values.

Properties:

| Property           | Usage Example                                       |
|--------------------|-----------------------------------------------------|
| name               | Name of the application or stack ("Hello There")    |
| description        | Description of the launcher ("MyAppStack launcher.")|
| author             | Author or creator of the launcher. Example: JP Mateo   |
| base_dir           | Base directory for the launcher ("/mydirectory")   |
| deploy_dir         | Directory where deployment artifacts are stored ("deploy") |
| services_dir       | Directory containing service configurations ("services") |
| compose_executable | Executable used for Docker Compose. Values: "docker-compose", "podman-compose" |
| compose_file       | The compose file name used for deployment. Example: myapp-compose.yaml |
| compose_detached   | Whether to run Docker Compose in detached mode (true) |

Example:
```yaml
[settings]
name = "Hello There"
description = "MyAppStack launcher."
author = "JP Mateo"
base_dir = "/mydirectory"
deploy_dir = "deploy"
services_dir = "services"
compose_executable = "docker-compose"
compose_file = "docker-compose-test.yaml"
compose_detached = true
```

### [[services]]
 Comprises a list or array of services, structured in a format compatible with Docker Compose services. For further details, refer to: [Docker Compose Services Documentation](https://docs.docker.com/compose/compose-file/05-services/)

| Property     | Description                                                           | Example                                    |
|--------------|-----------------------------------------------------------------------|--------------------------------------------|
| hostname     | Hostname of the service                                               | "db"                                       |
| image        | Docker image to use for the service                                   | "postgres"                                 |
| ports        | Ports to expose (host:container)                                      | ["5432:5432"]                              |
| environment  | Environment variables to set inside the container                     | { POSTGRES_USER = "admin", ... }           |
| env_file     | Path to the environment file to load variables from                    | ["database"]                               |
| networks     | Networks the service is connected to                                   | ["mynetwork"]                              |
| volumes      | Volumes to mount (kind, source, target)                               | [{ kind = "bind", source = ..., target = ... }] |
| depends_on   | Services this service depends on                                       | ["myserviceapp"]                           |
| restart      | Restart policy for the service                                         | "always"                                   |
| tty          | Allocate a pseudo-TTY                                                  | true                                       |

Example:
```toml
[[services]]
#This is the app service.
hostname = "db"
image = "postgres"
ports = [
    "5432:5432"
]
environment = { POSTGRES_USER = "admin", POSTGRES_PASSWORD = "admin123",  PGDATA="/var/lib/postgresql/data/pgdata" }
env_file = [
    "database",
]
networks = [
    "mynetwork"
]
volumes = [
    { kind = "bind", source = "/Users/jpmateo/Codes/rust/rustack-launcher/tests/testapp/data", target = "/var/lib/postgresql/data/pgdata"}
]
depends_on = [
    "myserviceapp"
]
restart = "always"
tty = true
```

### [[networks]]
Consists of a list or array of networks, formatted in accordance with Docker Compose network specifications. For additional guidance, please consult: [Docker Compose Network Documentation](https://docs.docker.com/compose/compose-file/06-networks/)

| Property | Description                                     | Example                                 |
|----------|-------------------------------------------------|-----------------------------------------|
| name     | Name of the network                             | "mynetwork2"                            |
| driver   | Network driver: See. https://docs.docker.com/network/drivers/                                  | "bridge"                                |
| labels   | Labels associated with the network (key-value)  | { "my.network.label.1" = ..., ... }    |
| external   | Set to true if the network is external. Default: false  | external = false    |

Example:
```yaml
[[networks]]
name = "mynetwork2"
driver = "bridge"
external = false
labels = { "my.network.label.1" = "This is a network label 1.", "my.network.label.2" = "This is a network label 2." }
```


### [[volumes]]
Comprises a list or array of volumes, structured in a format compatible with Docker Compose volume specifications. For further information, refer to: [Docker Compose Volume Documentation](https://docs.docker.com/compose/compose-file/07-volumes/)
| Property    | Description                                                               | Example                                   | serde(default) |
|-------------|---------------------------------------------------------------------------|-------------------------------------------|----------------|
| name        | Name of the volume                                                        | "testvolumes"                             |                |
| driver      | Volume driver. See: https://docs.docker.com/compose/compose-file/07-volumes/#driver                                                             | "nfs"                                     |                |
| driver_opts | Options for the volume driver (name, value). See: https://docs.docker.com/compose/compose-file/07-volumes/#driver_opts                               | [{ name = "type", value = "nfs" }, ... ] |                |
| external    | Indicates whether the volume is external. Default: false                                  | false                                     |                |
| labels      | Labels associated with the volume (key-value)                             | { "my.volume.label1" = ..., ... }        |                |
Example:
```yaml
[[volumes]]
name = "testvolumes"
driver = "nfs"
driver_opts = [
    { name = "type", value = "nfs" },
    { name = "o", value = "addr=10.40.0.199,nolock,soft,rw" },
    { name = "device", value = ":/docker/example" },
]
external = false
labels = { "my.volume.label1" = "This is a volume label 1.", "my.volume.label2" = "This is a volume label 2." }
```

### [[repositories]]
 This encompasses a list or array of repositories that require pulling from the version control system.
| Property     | Description                                       | Example                                                | serde(default) |
|--------------|---------------------------------------------------|--------------------------------------------------------|----------------|
| service      | Service associated with the repository           | "app"                                                  |                |
| mount_target | Target directory for mounting the repository      | "/var/db"                                              |                |
| name         | Name of the repository                            | "execism-diffie-hellman"                              |                |
| url          | URL of the repository                            | "https://github.com/j-p-d-e-v/execism-diffie-hellman" |                |
| branch       | Branch of the repository to use                   | "master"                                               |                |
| clone        | Indicates whether to clone the repository        | false                                                  |                |

```
Note: I did not include the authentication method. It is generally better to establish a connection between your machine/server and the desired Git server before proceeding.
```

Example:
```yaml
[[repositories]]
service = "app"
mount_target = "/var/db"
name = "execism-diffie-hellman"
url = "https://github.com/j-p-d-e-v/execism-diffie-hellman"
branch = "master"
clone = false
```

## Using Docker-Compose
Pre-requisites:
1. Docker Engine must be installed in your system. See: https://docs.docker.com/
2. Docker-Compose must be installed your system: See: https://docs.docker.com/compose/install/podman-compose
```yaml
    //Load the toml configuration file.
    let config = Config::load("config-test-docker.toml".to_string());

    let deploy_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.deploy_dir);
    config.validate();
    let env_file_paths: Vec<String> = EnvironmentFile::generate(&config.env_files,&deploy_dir);
    let compose: Compose = Compose::new(config);
    compose.up();
    compose.down();
```

## Using Podman-Compose

Pre-requisites:
1. Podman must be installed in your system. See: https://podman.io/
2. Podman-Compose must be installed your system: See: https://github.com/containers/podman-compose

Example:
```yaml
    let config = Config::load("config-test-podman.toml".to_string());
    let deploy_dir: String = format!("{}/{}",config.settings.base_dir,&config.settings.deploy_dir);
    config.validate();
    let env_file_paths: Vec<String> = EnvironmentFile::generate(&config.env_files,&deploy_dir);
    let compose: Compose = Compose::new(config);
    compose.up();
    compose.down();
```

## Unit Testing
```
cargo test -- --test-threads 1 --nocapture
```

## Developer
- JP Mateo