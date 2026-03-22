pub mod utils;
use clap::{Parser, ValueEnum};
use std::process::ExitCode;
use dunce::canonicalize;
use utils::*;

#[derive(Parser, Debug)]
#[command(name = "add2path")]
pub struct Cli {
    #[arg(value_name = "directory", default_value = ".")]
    directory: String,
    #[arg(
        short = 's',
        long = "scope",
        value_enum,
        default_value_t = Scope::User
    )]
    scope: Scope,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Scope {
    Process,
    User,
    System,
}

pub fn set_to_process(directory: String) -> bool {
    let path = match get_process_env("PATH") {
        Ok(e) => e,
        Err(_) => return false
    };
    let new_path = patch_path(path, &directory);
    set_process_env_safe("PATH", &new_path)
}

pub fn set_to_user(directory: String) -> bool {
    let user_path: String = match get_user_env("PATH") {
        Ok(e) => e,
        Err(_) => return false
    };
    let new_path_str = patch_path(user_path, &directory);
    let res = set_user_env_safe("PATH", &new_path_str);

    #[cfg(windows)]
    { notify_system(); }

    res
}

pub fn set_to_system(directory: String) -> bool {
    let user_path: String = match get_system_env("PATH") {
        Ok(e) => e,
        Err(_) => return false
    };
    let new_path_str = patch_path(user_path, &directory);
    let res = set_system_env_safe("PATH", &new_path_str);

    #[cfg(windows)]
    { notify_system(); }

    res
}

fn patch_path(path: String, add_new: &str) -> String {
    #[cfg(windows)]
    {
        let normalized = add_new.replace('/', "\\").trim_end_matches('\\').to_string();
        let current_paths: Vec<String> = path.split(';').map(|s| s.to_string()).collect();

        if current_paths.iter().any(|p| p.eq_ignore_ascii_case(&normalized)) {
            return path;
        }
    
        let mut new_path = path;
        if !new_path.is_empty() && !new_path.ends_with(';') {
            new_path.push(';');
        }
        new_path.push_str(&normalized);
        new_path
    } 
    #[cfg(not(windows))]
    {
        let normalized = add_new.replace('\\', "/").trim_end_matches('/').to_string();
        let current_paths: Vec<String> = path.split(':').map(|s| s.to_string()).collect();

        if current_paths.iter().any(|p| p.eq_ignore_ascii_case(&normalized)) {
            return path;
        }
    
        let mut new_path = path;
        if !new_path.is_empty() && !new_path.ends_with(':') {
            new_path.push(':');
        }
        new_path.push_str(&normalized);
        new_path
    } 
}



fn main() -> ExitCode {
    let args = Cli::parse();
    let dir = canonicalize(&args.directory);
    if dir.is_err() { 
        eprintln!("Failed to reslove directory {}.", &args.directory);
        return ExitCode::FAILURE;
    }
    let dir = dir.unwrap().to_string_lossy().into_owned();

    match args.scope {
        Scope::Process => {
            set_to_process(dir);
            println!("Process PATH updated (current process only).");
        }
        Scope::User => {
            if set_to_user(dir.clone()) {
                println!("Successfully added {} to User PATH.", dir);
            } else {
                eprintln!("Failed to update User PATH.");
                return ExitCode::FAILURE;
            }
        }
        Scope::System => {
            if set_to_system(dir.clone()) {
                println!("Successfully added {} to System PATH.", dir);
            } else {
                eprintln!("Failed to update System PATH. Check Administrator privileges.");
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}