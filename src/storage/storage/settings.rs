use super::utilities::*;
use crate::storage::models::{AppSettings, User};
use parking_lot::Mutex;
use std::{fs, path::PathBuf};

static SETTINGS: Mutex<Option<AppSettings>> = Mutex::new(None);

pub fn save_user_settings(user: User) {
    with_settings(|settings| {
        settings.user = Some(user);
        save_settings(settings);
    })
}

pub fn get_user() -> Option<User> {
    with_settings(|settings| settings.user.clone())
}

fn with_settings<F, R>(f: F) -> R
where
    F: FnOnce(&mut AppSettings) -> R,
{
    let mut settings = SETTINGS.lock();
    if settings.is_none() {
        *settings = Some(load_settings());
    }
    f(settings.as_mut().unwrap())
}

fn save_settings(settings: &AppSettings) {
    // TODO: Actually need to write this such that it replaces
    // any existing user data. There only needs to be one user.
    let path = get_settings_path();
    let json = serde_json::to_string_pretty(settings).unwrap();
    fs::write(path, json).expect("storage::settings::save_settings: Failed to write settings");
}

fn load_settings() -> AppSettings {
    let path = get_settings_path();
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default()
}

fn get_settings_path() -> PathBuf {
    let path = PathBuf::from("/data/user/0/com.ariane.BoozeBuddy/files");
    fs::create_dir(&path)
        .expect("storage::settings::get_settings_path: Failed to create directory");
    path.join("settings.json")
}
