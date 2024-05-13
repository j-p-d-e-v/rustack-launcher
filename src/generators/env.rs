use crate::generators::prelude::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
///The struct for the environment variable.
pub struct EnvironmentVar {
     /// The variable/key name.
    pub name: String,
     /// The value of the environment variable.
    pub value: String
}

#[derive(Deserialize,Serialize, Debug, Clone)]
///The struct for the environment file
pub struct EnvironmentFile {
    /// The name of the env file.
    pub name: String,
    /// The list of key/value pairs of the env file.
    pub values: Vec<EnvironmentVar>
}

impl EnvironmentFile {
    /// Write a list environment files then store it under the configured path of deploy_dir. 
    /// # Example
    /// ```ignore
    /// let data = EnvironmentFile {
    ///     name: String::from("test"),
    ///     values: Vec::from([
    ///         EnvironmentVar {
    ///             name: "MYVAR1".to_string(),
    ///             value: "MyVAL1".to_string()
    ///         },
    ///         EnvironmentVar {
    ///             name: "MYVAR2".to_string(),
    ///             value: "MyVAL2".to_string()
    ///         }
    ///     ])
    /// };
    /// assert_eq!(!data.name.is_empty(),true);
    /// assert_eq!(data.values.len() > 0,true);
    /// ```
    pub fn generate(env_files: &Vec<EnvironmentFile>, deploy_dir: &String) -> Vec<String> {
        let mut file_paths: Vec<String> = Vec::new();
        for item in env_files {
            let data = EnvironmentFile { name: item.name.clone(), values: item.values.to_vec() };
            match EnvironmentFile::write(data,&deploy_dir) {
                Ok(file_path) => {
                    file_paths.push(file_path);
                }
                Err(error) => {
                    panic!("Unable to generate env file: {:?}.",error);
                }
            };
        }
        file_paths
    }

    /// Write an environment file then store it under the configured path of deploy_dir.
    pub fn write(env_file: EnvironmentFile, deploy_dir: &String ) -> Result<String,SerdeEnvFileError> {
        let mut values: HashMap<String, String> = HashMap::new();
        let file_name: String  = format!(".{}.env",env_file.name);
        let file_path: String  = format!("{}/{}",deploy_dir,file_name);
        let path_buff: PathBuf = PathBuf::from(file_path.clone());
        for item in env_file.values {
            values.insert(item.name.clone(),item.value.clone());
        }
        serde_envfile::to_file(&path_buff,&values)?;
        Ok(file_path)
    }
}
