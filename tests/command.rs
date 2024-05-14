
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};
fn main() -> Result<(), Error>  {

    let stdout = Command::new("ping")
        .arg("1.1.1.1")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout.ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;
        let reader = BufReader::new(stdout);

        reader
        .lines()
        .for_each(|line| println!("{}", line.ok().unwrap()));

        Ok(())
}