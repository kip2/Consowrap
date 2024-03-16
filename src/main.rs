use clap::Parser;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

extern crate consowrap;
use consowrap::Args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args.one);
}

pub fn run_command(command_name: &str) {
    let env_path = "./env.json";
    let json_key = "command_directory_path";
    let command_name = "ls";
    let command_directory_path = match get_target_directory_path(env_path, json_key) {
        Ok(path) => path,
        Err(e) => panic!("Error getting directory path: {}", e),
    };
    let command_path = search_file_path(command_directory_path, command_name);
}

fn search_file_path<P: AsRef<Path>>(dir: P, file_name: &str) -> Option<String> {
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

fn get_target_directory_path(file_path: &str, json_key: &str) -> Result<String, String> {
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
    fn test_search_file_path() {
        let env_path = "./tests/env.json";
        let json_key = "test_command_directory_path";
        let command_path = get_target_directory_path(env_path, json_key).unwrap();
        let target_file1 = "test1";
        let target_file2 = "cat";
        let target_file3 = "ls";

        assert_eq!(
            search_file_path(&command_path, target_file1).unwrap(),
            "./tests/Commands/test1"
        );
        assert_eq!(
            search_file_path(&command_path, target_file2).unwrap(),
            "./tests/Commands/cat"
        );
        assert_eq!(
            search_file_path(&command_path, target_file3).unwrap(),
            "./tests/Commands/ls"
        );
    }

    #[test]
    fn test_read_target_directory_path() {
        let path = "./tests/env.json";
        let json_key = "test_command_directory_path";
        assert_eq!(
            get_target_directory_path(path, json_key).unwrap(),
            "./tests/Commands"
        );
        assert_ne!(
            get_target_directory_path(path, json_key).unwrap(),
            "./tests/wrongPath"
        );
    }
}
