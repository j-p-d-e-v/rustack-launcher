
use std::process::{Command, Stdio};
use std::io::{ BufRead, BufReader };
#[warn(unused_imports)]
fn main() {
    let mut child = Command::new("ping")
        .arg("1.1.1.1")
        .stdout(Stdio::piped())
        .spawn().unwrap();
    let stdout = child.stdout.take().unwrap();
    let lines = BufReader::new(stdout).lines();  
    for line in lines {
        println!("{}",line.unwrap());
    }
    println!("Finished");
}