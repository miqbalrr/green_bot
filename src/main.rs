use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use chrono::{Utc};
use std::time::Duration;
use job_scheduler::{JobScheduler, Job};



fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("* 1 * * * *".parse().unwrap(), || {
        println!("{:?}",execute_bot());
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn execute_bot() -> std::io::Result<String> {
    std::fs::remove_file("foo.txt");
    std::fs::remove_file(".git/index.lock");
    let mut file = File::create("foo.txt")?;
    let now = Utc::now();
    file.write_all(format!("updated at {:?}", now).as_bytes())?;
    Command::new("git").args(&["add","."]).spawn()?;
    Command::new("git").args(&["commit","-m", format!("run at {}", now).as_str()]).spawn()?;
    Ok(format!("{:?}", Command::new("git").args(&["push","origin","master"]).output()))
}