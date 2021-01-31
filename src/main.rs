use std::process::Command;

fn main() -> Result<(), ()> {
    Command::new("git").args(&["add","."]);
    Command::new("git").args(&["commit","-m", "runn"]);
    println!("{:?}", Command::new("git").arg("status").output());
    Ok(())
}
