use crate::generators::env::{EnvironmentFile, EnvironmentVar};

#[test]
fn test_initialization(){
    let data = EnvironmentFile {
        name: String::from("test"),
        values: Vec::from([
            EnvironmentVar {
                name: "MYVAR1".to_string(),
                value: "MyVAL1".to_string()
            },
            EnvironmentVar {
                name: "MYVAR2".to_string(),
                value: "MyVAL2".to_string()
            }
        ])
    };
    assert_eq!(!data.name.is_empty(),true);
    assert_eq!(data.values.len() > 0,true);
}