use std::{path::PathBuf, println};
use std::collections::HashMap;

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
        
        // found .env file
        println!("{:?}", env_file_exists);
        let env_file = std::fs::read_to_string(dir.join(".env")).ok().unwrap();
        let mut line_vec: Vec<String> = env_file.lines().map(|s| s.to_string()).collect();
        
        let mut map: HashMap<String, String> = HashMap::new();

        // for each line in the file
        for i in 0..line_vec.len() {
            let line: &str = line_vec[i].trim();

            // empty lines or comments
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let line_pair: Vec<&str> = line.split("=").collect();
            map.insert(line_pair[0].trim().to_string(), line_pair[1].trim().to_string());

            println!("{line}");
        }
        println!("map {:?}", map);
        println!("lines: {:?}", line_vec);
        break;
    }
}
