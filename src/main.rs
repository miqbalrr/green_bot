use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use chrono::{Utc};

fn main() -> std::io::Result<()> {
    //test
    // match file.write_all(format!("update at {}", &Utc::now()).as_bytes()) {
    //     Ok(..) => {},
    //     Err(e) => println!("{}", e),
    // }
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;

    Command::new("git").args(&["add","."]).spawn();
    Command::new("git").args(&["commit","-m", "runn"]).spawn();
    println!("{:?}", Command::new("git").args(&["push","origin","master"]).output());
    Ok(())
}
