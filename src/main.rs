use std::process::Command;

fn main() -> Result<(), ()> {
    //test
    Command::new("git").args(&["add","."]).spawn();
    Command::new("git").args(&["commit","-m", "runn"]).spawn();
    println!("{:?}", Command::new("git").args(&["push","origin","master"]).output());
    Ok(())
}
