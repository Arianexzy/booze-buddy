use crate::storage::{
    models::{DrinkType, DrinkingHistory, DrinkingSession},
    storage::settings::get_user,
};
use parking_lot::Mutex;
use std::path::PathBuf;

use super::utils::*;

static HISTORY_STORAGE: Mutex<Option<DrinkingHistory>> = Mutex::new(None);

pub fn start_new_session() {
    with_history(|history| {
        history.start_new_session();
        save_data(history);
    });
}

pub fn end_current_session() {
    with_history(|history| {
        history.end_current_session();
        save_data(history);
    });
}

pub fn add_drink_by(drink_type: DrinkType) {
    with_history(|history| {
        if let Some(session) = history.current_session() {
            session.add_drink_by(drink_type);
            save_data(history);
        }
    });
}

pub fn remove_last(drink_type: DrinkType) {
    with_history(|history| {
        if let Some(session) = history.current_session() {
            session.remove_last(drink_type);
            save_data(history);
        }
    })
}

fn save_data(data: &DrinkingHistory) {
    let path = get_history_path();
    save_json(path, data).expect("Failed to save Drinking History")
}

pub fn get_count_by(drink_type: DrinkType) -> i32 {
    with_history(|history| {
        history
            .current_session()
            .map_or(0, |session| session.count_by(drink_type))
    })
}

pub fn get_total_drinks() -> i32 {
    with_history(|history| history.current_session().unwrap().total_drinks())
}

pub fn has_active_session() -> bool {
    with_history(|history| history.current_session().is_some())
}

pub fn get_past_sessions() -> Vec<DrinkingSession> {
    with_history(|history| history.past_sessions().into_iter().cloned().collect())
}

pub fn get_current_bac() -> f32 {
    with_history(|history| {
        let user = get_user();
        history
            .current_session()
            .map_or(0.0, |session| session.calculate_bac(&user.unwrap()))
    })
}

fn with_history<F, R>(f: F) -> R
where
    F: FnOnce(&mut DrinkingHistory) -> R,
{
    let mut storage = HISTORY_STORAGE.lock();
    if storage.is_none() {
        *storage = Some(load_data());
    }
    f(storage.as_mut().unwrap())
}

fn load_data() -> DrinkingHistory {
    let path = get_history_path();
    load_json(path).expect("Failed to load Drinking History")
}

fn get_history_path() -> PathBuf {
    get_app_path().join("drinking_history.json")
}
