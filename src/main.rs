use clap::Parser;
use serde_json::Value;
use std::env;
use std::fmt::Arguments;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

extern crate consowrap;
use consowrap::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = args.join(" ");

    run(args);
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
    if parts.len() < 2 {
        eprintln!("Error: Not enough arguments.");
        return;
    }

    let command = parts[1];
    let arguments = &parts[2..];

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

pub fn run_command(command_line: String) -> std::io::Result<()> {
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
    let mut file = File::open(file_path).expect("File could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to load file.");

    let json: Value = serde_json::from_str(&contents).unwrap();

    match json[json_key].as_str() {
        Some(path) => Ok(path.to_string()),
        None => Err("File not found.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(
            get_command_directory_path(path, json_key).unwrap(),
            "./tests/Commands"
        );
        assert_ne!(
            get_command_directory_path(path, json_key).unwrap(),
            "./tests/wrongPath"
        );
    }
}
