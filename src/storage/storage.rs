use crate::storage::models::{DrinkType, DrinkingHistory, DrinkingSession};
use parking_lot::Mutex;
use std::fs;
use std::path::PathBuf;

static STORAGE: Mutex<Option<DrinkingHistory>> = Mutex::new(None);

pub fn start_new_session() {
    with_storage(|history| {
        history.start_new_session();
        save_data(history);
    });
}

pub fn end_current_session() {
    with_storage(|history| {
        history.end_current_session();
        save_data(history);
    });
}

pub fn add_drink_by(drink_type: DrinkType) {
    with_storage(|history| {
        if let Some(session) = history.current_session() {
            session.add_drink_by(drink_type);
            save_data(history);
        }
    });
}

pub fn remove_last(drink_type: DrinkType) {
    with_storage(|history| {
        if let Some(session) = history.current_session() {
            session.remove_last(drink_type);
            save_data(history);
        }
    })
}

pub fn get_count_by(drink_type: DrinkType) -> i32 {
    with_storage(|history| {
        history
            .current_session()
            .map_or(0, |session| session.count_by(drink_type))
    })
}

pub fn get_past_sessions() -> Vec<DrinkingSession> {
    with_storage(|history| history.past_sessions().into_iter().cloned().collect())
}

fn with_storage<F, R>(f: F) -> R
where
    F: FnOnce(&mut DrinkingHistory) -> R,
{
    let mut storage = STORAGE.lock();
    if storage.is_none() {
        *storage = Some(load_data());
    }
    f(storage.as_mut().unwrap())
}

fn save_data(data: &DrinkingHistory) {
    let path = get_storage_path();
    let json = serde_json::to_string_pretty(data).unwrap();
    fs::write(path, json).expect("Failed to save data");
}

fn load_data() -> DrinkingHistory {
    let path = get_storage_path();
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default()
}

fn get_storage_path() -> PathBuf {
    let path = PathBuf::from("/data/user/0/com.ariane.BoozeBuddy/files");
    fs::create_dir_all(&path).expect("Failed to create storage directory");
    path.join("drinking_history.json")
}
