use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let err = Command::new("kubectl").exec();
    println!("Failed to exec: {}", err);
}
