//Importing Command
use std::process::Command;

fn main() {
    Command::new("C:\\Users\\WebRTC User\\AppData\\Local\\Discord\\Update.exe")
                .args(["--processStart","Discord.exe"]).output().expect("failed to launch discord");
}
