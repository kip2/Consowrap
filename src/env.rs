use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process;

fn create_env_file() -> Result<(), String> {
    let env_file = ".env";
    let file = File::create(env_file).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn env_load() {
    if let Err(e) = check_env_file_exists() {
        println!("Error: {}", e);
        if let Err(e) = create_env_file() {
            println!("Failed to create .env file: {}", e);
            process::exit(1);
        }
        process::exit(1);
    }

    if dotenv().is_err() {
        println!("Failed to load .env file");
        process::exit(1);
    }
}

fn check_env_file_exists() -> Result<(), String> {
    let env = ".env";
    if Path::new(env).exists() {
        Ok(())
    } else {
        Err(format!("{} file does not exist. A .env file has been created. Please write your configurations.", env))
    }
}

pub fn get_env_var(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("{} is not set in .env file", key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_load() {
        env_load();
    }
}
