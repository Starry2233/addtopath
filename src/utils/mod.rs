use std::env;
use std::panic::catch_unwind;
use anyhow::Result;
#[cfg(windows)]
use winapi::um::winuser::{SendMessageTimeoutA, HWND_BROADCAST, WM_SETTINGCHANGE, SMTO_ABORTIFHUNG};

#[cfg(windows)]
use winreg::{enums::*, RegKey, RegValue};

pub unsafe fn set_process_env(key: &str, value: &str) {
    unsafe {
        env::set_var(key, value);
    }
}

pub fn set_process_env_safe(key: &str, value: &str) -> bool {
    match catch_unwind(|| unsafe {
        set_process_env(key, value);
    }) {
        Ok(_) => return true,
        Err(_) => return false
    }
} 

pub fn get_process_env(key: &str) -> Result<String, env::VarError> { env::var(key) }

pub unsafe fn set_user_env(key: &str, value: &str) {
    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let user_env = match hkcu.open_subkey_with_flags("Environment", KEY_WRITE) {
            Ok(k) => k,
            Err(_) => panic!("Failed to open environment"),
        };
        
        let val = RegValue {
            vtype: REG_EXPAND_SZ,
            bytes: string_to_win_bytes(value).into(),
        };

        if user_env.set_raw_value(key, &val).is_err() {
            panic!("Failed to set environment")
        }
    }
}

pub fn set_user_env_safe(key: &str, value: &str) -> bool {
    match catch_unwind(|| unsafe {
        set_user_env(key, value);
    }) {
        Ok(_) => return true,
        Err(_) => return false
    }
}

pub fn get_user_env(key: &str) -> Result<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let user_env = hkcu
        .open_subkey_with_flags("Environment", KEY_READ)?;
    let val = user_env.get_value(key)?;
    Ok(val)
}

pub unsafe fn set_system_env(key: &str, value: &str) {
    #[cfg(windows)]
    {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let user_env = match hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", KEY_WRITE) {
            Ok(k) => k,
            Err(_) => panic!("Failed to open environment"),
        };
        
        let val = RegValue {
            vtype: REG_EXPAND_SZ,
            bytes: string_to_win_bytes(value).into(),
        };

        if user_env.set_raw_value(key, &val).is_err() {
            panic!("Failed to set environment")
        }
    }
}

pub fn set_system_env_safe(key: &str, value: &str) -> bool {
    match catch_unwind(|| unsafe {
        set_system_env(key, value);
    }) {
        Ok(_) => return true,
        Err(_) => return false
    }
}

pub fn get_system_env(key: &str) -> Result<String> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let user_env = hklm
        .open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", KEY_READ)?;
    let val = user_env.get_value(key)?;
    Ok(val)
}

#[cfg(windows)]
pub fn notify_system() {
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

#[cfg(windows)]
fn string_to_win_bytes(s: &str) -> Vec<u8> {
    use std::iter::once;
    s.encode_utf16()
        .chain(once(0))
        .flat_map(|u| u.to_le_bytes())
        .collect()
}