use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use chrono::{Utc};

fn main() -> std::io::Result<()> {
    std::fs::remove_file("foo.txt");
    std::fs::remove_file(".git/index.lock");
    let mut file = File::create("foo.txt")?;
    let now = Utc::now();
    file.write_all(format!("updated at {:?}", now).as_bytes())?;
    Command::new("git").args(&["add","."]).spawn()?;
    Command::new("git").args(&["commit","-m", format!("run at {}", now).as_str()]).spawn()?;
    println!("{:?}", Command::new("git").args(&["push","origin","master"]).output());
    Ok(())
}
