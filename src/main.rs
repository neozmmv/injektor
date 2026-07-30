use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
mod helpers;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut dir = PathBuf::from(".");
    let mut env_file_exists: bool = dir.join(".env").exists();

    if args.is_empty() {
        let logo = r#"
  _____        _      _    _             
  \_   \_ __  (_) ___| | _| |_ ___  _ __ 
   / /\/ '_ \ | |/ _ \ |/ / __/ _ \| '__|
/\/ /_ | | | || |  __/   <| || (_) | |   
\____/ |_| |_|/ |\___|_|\_\\__\___/|_|   
            |__/                         
"#;

        println!("{logo}");
        println!("Environment variable injector.\n");
        println!("Usage: injektor <command> [args...]");
        return;
    }

    loop {
        if !env_file_exists {
            if !dir.pop() {
                eprintln!("Could not find any .env file!");
                std::process::exit(1);
            }
            env_file_exists = dir.join(".env").exists();
            continue; // go back to the top and re-check, instead of falling through
        }

        // found .env file, try to read it
        let env_file = match std::fs::read_to_string(dir.join(".env")) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading .env file: {}", e);
                std::process::exit(1);
            }
        };

        let line_vec: Vec<String> = env_file.lines().map(|s| s.to_string()).collect();
        let mut map: HashMap<String, String> = HashMap::new();

        // for each line in the file
        for (line_number, raw_line) in line_vec.iter().enumerate() {
            let line = raw_line.trim();

            // empty lines or comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // split only on the first '=', so values containing '=' stay intact
            match line.split_once('=') {
                Some((key, value)) => {
                    let key = helpers::kill_quotes(key.trim().to_string());
                    let value = helpers::kill_quotes(value.trim().to_string());
                    map.insert(key, value);
                }
                None => {
                    // malformed line (no '='), warn but keep going instead of crashing
                    eprintln!(
                        "Warning: ignoring malformed line {} in .env: '{}'",
                        line_number + 1,
                        line
                    );
                }
            }
        }
        // map ready

        let cmd = Command::new(&args[0])
            .args(&args[1..])
            .envs(&map)
            .status();

        match cmd {
            Ok(status) => {
                std::process::exit(status.code().unwrap_or(1));
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("Error: command '{}' not found", args[0]);
                std::process::exit(127); // shell convention for "command not found"
            }
            Err(e) => {
                eprintln!("Error running command: {}", e);
                std::process::exit(1);
            }
        }
    }
}