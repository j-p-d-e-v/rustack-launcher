use crate::generators::execute_command::{ExecuteCommand};
#[test]
#[ignore]
fn test_execute_command(){
    let is_executed: bool = ExecuteCommand::run("lst".to_string(),vec![]);
    assert!(is_executed);
}