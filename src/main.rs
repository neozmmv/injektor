use std::{path::PathBuf, println};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut dir = PathBuf::from(".");
    let mut env_file_exists: bool = dir.join(".env").exists();
    println!("ARGS: {:?}", args);
    println!("CURRENT DIR: {:?}", dir);
    loop {
        if !env_file_exists {
            if !dir.pop() {
                println!("Could not find any .env file!");
                break;
            }
            env_file_exists = dir.join(".env").exists();
        }
        println!("{:?}", env_file_exists);
        break;
    }
}
