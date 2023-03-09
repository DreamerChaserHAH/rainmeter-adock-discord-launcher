//Importing Command
use std::process::Command;
//dynamically get user home folder
use std::env;

fn main() {

    //println!("{:?}",env::home_dir());
    let user_homedir = env::home_dir().unwrap().to_str().unwrap().to_string() + "\\AppData\\Local\\Discord\\Update.exe";

    Command::new(user_homedir)
                .args(["--processStart","Discord.exe"]).output().expect("failed to launch discord");
}
