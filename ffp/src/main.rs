use std::process::Command;

fn main() {
    Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe")
        .arg("-P Main --noRemote")
        .output()
        .expect("Failed");
}