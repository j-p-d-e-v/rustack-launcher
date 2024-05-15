use std::process::{ Command, Stdio };
use std::io::{ BufRead, BufReader };

pub struct ExecuteCommand {
    pub exec: String,
    pub args: Vec<String>
}

impl ExecuteCommand {
    pub fn run(exec: String, args: Vec<String>) -> bool {
        match Command::new(&exec).args(args).stdout(Stdio::piped()).spawn() {
            Ok(mut child) => {
                if let Some(stdout) = child.stdout.take() {
                    let lines = BufReader::new(stdout).lines();  
                    for line in lines {
                        match line {
                            Ok(output) => println!("{}",output),
                            Err(error) => panic!("{}",error)
                        }
                    }
                }
                else {
                    println!("No output.");
                }
                true
            }
            Err(error) => {
                panic!("Unable to execute {}. Error: {:?}",&exec,error);
            }
        }
    }
}