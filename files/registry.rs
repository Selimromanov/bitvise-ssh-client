//! Windows registry read helpers for Bitvise Ssh Client.

pub const APP_KEY: &str = "Software\\Bitvise\\Bitvise Ssh Client";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Bitvise Ssh Client";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Bitvise Ssh Client") },
        RegValue { name: "Version",     value: String::from("v2.7.12") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Bitvise Ssh Client") },
        RegValue { name: "Publisher",       value: String::from("Bitvise") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Bitvise Ssh Client\\uninstall.exe\"") },
    ]
}
