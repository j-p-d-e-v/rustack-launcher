use crate::generators::prelude::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct EnvironmentVar {
    pub name: String,
    pub value: String
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct EnvironmentFile {
    pub name: String,
    pub values: Vec<EnvironmentVar>
}

impl EnvironmentFile {
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
