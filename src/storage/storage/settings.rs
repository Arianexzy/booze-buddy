use super::utils::*;
use crate::storage::models::{AppSettings, User};
use parking_lot::Mutex;
use std::path::PathBuf;

static SETTINGS_STORAGE: Mutex<Option<AppSettings>> = Mutex::new(None);

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
    let mut settings = SETTINGS_STORAGE.lock();
    if settings.is_none() {
        *settings = Some(load_settings());
    }
    f(settings.as_mut().unwrap())
}

fn save_settings(settings: &AppSettings) {
    let path = get_settings_path();
    save_json(path, settings).expect("Failed to save Settings")
}

fn load_settings() -> AppSettings {
    let path = get_settings_path();
    load_json(path).expect("Failed to load Settings")
}

fn get_settings_path() -> PathBuf {
    get_app_path().join("settings.json")
}
