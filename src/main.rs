use serde_json::Value;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;
use std::fs::{self, File};
use std::io::Read;
use dotenv::dotenv;
use std::env;

use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
struct Args {
    /// Print list of available commands
    #[arg(
        short,
        long, 
        help = "Print list of available commands",
        action = ArgAction::SetTrue
    )]
    list: bool,

    /// Command-line to execute
    #[arg(required_unless_present = "list")]
    command_and_args: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.list {
        let commands_directory_path = "./Commands";
        list_commands(commands_directory_path);
    } else if !args.command_and_args.is_empty() {
        run(args.command_and_args.join(" "));
    } else {
        println!("No command specified.");
    }
}

pub fn list_commands(directory: &str) {
    let path = Path::new(directory);

    match fs::read_dir(path) {
        Ok(entries) => {
            entries.filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .filter_map(|entry| entry.path().file_name().and_then(|name| name.to_str()).map(String::from))
                .for_each(|filename_str| println!("{}", filename_str));
        }
        Err(_) => println!("Directory not found: {}", directory),
    }
}

pub fn run(input: String) -> () {
    let env_path = "./env.json";
    let json_key = "command_directory_path";

    let command_directory_path = match get_command_directory_path(env_path, json_key) {
        Ok(path) => path,
        Err(e) => panic!("Error getting directory path: {}", e),
    };

    let parts: Vec<&str> = input.split_whitespace().collect();

    // Error: Insufficient number of arguments provide.
    if parts.len() < 1 {
        eprintln!("Error: Not enough arguments.");
        return;
    }

    let command = parts[0];
    let arguments = &parts[1..];

    let command_path = match find_command_path(&command_directory_path, command) {
        Some(path) => path,
        None => {
            eprintln!(
                "Command '{}' not found in directory '{}'",
                command, command_directory_path
            );
            return;
        }
    };

    let commandline = format!("{} {}", command_path, arguments.join(" "));

    run_command(commandline).expect("Error: Command execution failed.");
}

fn run_command(command_line: String) -> std::io::Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg(command_line)
        .status()
        .and_then(|status| if status.success() { Ok(()) } else { Ok(()) })
}

fn find_command_path<P: AsRef<Path>>(dir: &P, file_name: &str) -> Option<String> {
    for entry in WalkDir::new(dir) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.file_type().is_dir() {
            continue;
        }

        if let Some(name) = entry.file_name().to_str() {
            if name == file_name {
                return Some(entry.path().to_string_lossy().into_owned());
            }
        }
    }
    None
}

fn get_command_directory_path(file_path: &str, json_key: &str) -> Result<String, String> {
    let mut file = File::open(file_path).map_err(|_e| format!("File could not be opened."))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_e| format!("Failed to load file."))?;

    let json: Value =
        serde_json::from_str(&contents).map_err(|_| "Failed to parse JSON.".to_string())?;

    match json[json_key].as_str() {
        Some(path) => Ok(path.to_string()),
        None => Err("Specified key not found in JSON file.".to_string()),
    }
}

fn get_command_directory_path2() -> Result<String, String> {
    dotenv().ok();

    let key = "COMMAND_DIRECTORY_PATH";

    get_env_var(key)
}

fn get_env_var(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("{} is not set in .env file", key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_directory_path2() {
        let path = get_command_directory_path2().unwrap();
        assert_eq!(path, "./Commands");
    }

    #[test]
    fn test_find_command_path() {
        let env_path = "./tests/env.json";
        let json_key = "test_command_directory_path";
        let command_path = get_command_directory_path(env_path, json_key).unwrap();
        let target_file1 = "test1";
        let target_file2 = "cat";
        let target_file3 = "ls";

        assert_eq!(
            find_command_path(&command_path, target_file1).unwrap(),
            "./tests/Commands/test1"
        );
        assert_eq!(
            find_command_path(&command_path, target_file2).unwrap(),
            "./tests/Commands/cat"
        );
        assert_eq!(
            find_command_path(&command_path, target_file3).unwrap(),
            "./tests/Commands/ls"
        );
    }

    #[test]
    fn test_get_command_directory_path() {
        let path = "./tests/env.json";
        let json_key = "test_command_directory_path";
        match get_command_directory_path(path, json_key) {
            Ok(result) => assert_eq!(result, "./tests/Commands"),
            Err(e) => panic!("Unexpected error: {}", e),
        }
        match get_command_directory_path(path, json_key) {
            Ok(result) => assert_ne!(result, "./tests/wrongPath"),
            Err(e) => panic!("Unexpected error: {}", e),
        }

        let wrong_path = "./tests/wrong.json";
        match get_command_directory_path(wrong_path, json_key) {
            Ok(_) => panic!("Should have fialed but didn't"),
            Err(e) => assert_eq!(e, "File could not be opened."),
        }

        let wrong_key = "wrong_JSON_key";
        match get_command_directory_path(path, wrong_key) {
            Ok(_) => panic!("Should have fialed but didn't"),
            Err(e) => assert_eq!(e, "Specified key not found in JSON file."),
        }

        let invalid_json = "./tests/invalid.json";
        match get_command_directory_path(invalid_json, json_key) {
            Ok(_) => panic!("Should have fialed due to invalid JSON format but didn't"),
            Err(e) => assert_eq!(e, "Failed to parse JSON."),
        }
    }
}
