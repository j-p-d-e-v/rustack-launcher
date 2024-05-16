use crate::generators::execute_command::{ExecuteCommand};
#[test]
fn test_execute_command(){
    let is_executed: bool = ExecuteCommand::run("ls".to_string(),vec![]);
    assert!(is_executed);
}