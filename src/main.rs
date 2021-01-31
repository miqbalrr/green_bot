use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use chrono::{Utc};

fn main() -> std::io::Result<()> {
    std::fs::remove_file("foo.txt")?;
    let mut file = File::create("foo.txt")?;
    file.write_all(format!("updated at {:?}", Utc::now()).as_bytes())?;
    Command::new("git").args(&["add","."]).spawn();
    Command::new("git").args(&["commit","-m", "runn"]).spawn();
    println!("{:?}", Command::new("git").args(&["push","origin","master"]).output());
    Ok(())
}
