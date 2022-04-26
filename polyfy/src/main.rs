use std::process::{Command, Stdio};
use dirs;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut arg = match std::env::args().nth(1) {
        Some(ct) => { ct }
        None => { panic!("No arguments supplied") }
    };

    arg.push('\n');

    let mut path = match dirs::home_dir() {
        Some(path) => { path }
        None => { panic!("Couldn't derive home directory") }
    };

    path.push(".config");
    path.push("polybar");
    path .push("msg");

    let mut file = match File::create(path) {
        Ok(file) => {file}
        Err(err) => {panic!("Ran into an error accessing to the file\n{}", err)}
    };

    match file.write_all(arg.as_bytes()) {
        Ok(_) => {}
        Err(err) => {panic!("Ran into an error writing to the file\n{}", err)}
    };
    println!("Outputted to file");
    
    let mut cmd = Command::new("polybar-msg");
    cmd.stdout(Stdio::null());
    cmd.arg("hook");
    cmd.arg("display");
    cmd.arg("2");

    match cmd.status() {
        Ok(_) => {}
        Err(error) => { panic!("Ary caramba, we've been played!\n{}", error) }
    };

    println!("Notified");
}
