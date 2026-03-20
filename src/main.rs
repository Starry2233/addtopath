use clap::{Parser, ValueEnum};
use std::env;
use std::process::ExitCode;
use dunce::canonicalize;

#[cfg(windows)]
use winreg::{enums::*, RegKey, RegValue};
#[cfg(windows)]
use winapi::um::winuser::{SendMessageTimeoutA, HWND_BROADCAST, WM_SETTINGCHANGE, SMTO_ABORTIFHUNG};

#[derive(Parser, Debug)]
#[command(name = "add2path")]
pub struct Cli {
    #[arg(value_name = "directory", default_value = ".")]
    directory: String,
    #[arg(
        short = 's',
        long = "scope",
        value_enum,
        default_value_t = Scope::Process
    )]
    scope: Scope,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Scope {
    Process,
    User,
    System,
}

pub fn set_to_process(directory: String) {
    let path = env::var("PATH").unwrap_or_default();
    let new_path = patch_path(path, &directory);
    unsafe {
        env::set_var("PATH", new_path);
    }
}

pub fn set_to_user(directory: String) -> bool {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let user_env = match hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE) {
            Ok(k) => k,
            Err(_) => return false,
        };
        let user_path: String = user_env.get_value("Path").unwrap_or_default();
        let new_path_str = patch_path(user_path, &directory);
        
        let val = RegValue {
            vtype: REG_EXPAND_SZ,
            bytes: string_to_win_bytes(&new_path_str).into(),
        };

        if user_env.set_raw_value("Path", &val).is_err() {
            return false;
        }
        notify_system();
        true
    }
    #[cfg(not(windows))] { false }
}

pub fn set_to_system(directory: String) -> bool {
    #[cfg(windows)]
    {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path_key = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";
        let system_env = match hklm.open_subkey_with_flags(path_key, KEY_READ | KEY_WRITE) {
            Ok(k) => k,
            Err(_) => return false,
        };
        let system_path: String = system_env.get_value("Path").unwrap_or_default();
        let new_path_str = patch_path(system_path, &directory);

        let val = RegValue {
            vtype: REG_EXPAND_SZ,
            bytes: string_to_win_bytes(&new_path_str).into(),
        };

        if system_env.set_raw_value("Path", &val).is_err() {
            return false;
        }
        notify_system();
        true
    }
    #[cfg(not(windows))] { false }
}

fn patch_path(path: String, add_new: &str) -> String {
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

#[cfg(windows)]
fn string_to_win_bytes(s: &str) -> Vec<u8> {
    use std::iter::once;
    s.encode_utf16()
        .chain(once(0))
        .flat_map(|u| u.to_le_bytes())
        .collect()
}

#[cfg(windows)]
fn notify_system() {
    unsafe {
        let mut _res: usize = 0;
        SendMessageTimeoutA(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            "Environment\0".as_ptr() as isize,
            SMTO_ABORTIFHUNG,
            5000,
            &mut _res as *mut usize as *mut _,
        );
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