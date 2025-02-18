use super::utils::*;
use crate::storage::{
    models::{DrinkType, DrinkingHistory, DrinkingSession},
    storage::{error::StorageResult, settings::get_user},
};
use parking_lot::Mutex;
use std::path::PathBuf;

static HISTORY_STORAGE: Mutex<Option<DrinkingHistory>> = Mutex::new(None);

// CORE STORAGE FUNCTIONS

fn with_active_session<F, R>(f: F) -> R
where
    F: FnOnce(&mut DrinkingSession) -> R,
{
    with_history(|history| f(history.ensure_active_session()))
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

// PUBLIC API

pub fn get_total_drinks() -> StorageResult<i32> {
    with_active_session(|session| Ok(session.total_drinks()))
}

pub fn get_count_by(drink_type: DrinkType) -> StorageResult<i32> {
    with_active_session(|session| Ok(session.count_by(drink_type)))
}

pub fn get_current_bac() -> StorageResult<f32> {
    let user = get_user();
    with_active_session(|session| Ok(session.calculate_bac(&user)))
}

pub fn add_drink_by(drink_type: DrinkType) -> StorageResult<()> {
    with_history(|history| {
        let session = history.ensure_active_session();
        session.add_drink_by(drink_type);
        save_data(history);
        Ok(())
    })
}

pub fn remove_last(drink_type: DrinkType) -> StorageResult<()> {
    with_history(|history| {
        let session = history.ensure_active_session();
        session.remove_last(drink_type);
        save_data(history);
        Ok(())
    })
}

pub fn end_current_session() -> StorageResult<()> {
    with_history(|history| {
        history.end_current_session();
        save_data(history);
    });
    Ok(())
}

pub fn get_past_sessions() -> StorageResult<Vec<DrinkingSession>> {
    Ok(with_history(|history| {
        history.past_sessions().into_iter().cloned().collect()
    }))
}

// PRIVATE HELPER FUNCTIONS

fn save_data(data: &DrinkingHistory) {
    let path = get_history_path();
    save_json(path, data).expect("Failed to save Drinking History")
}

fn load_data() -> DrinkingHistory {
    let path = get_history_path();
    load_json(path).expect("Failed to load Drinking History")
}

fn get_history_path() -> PathBuf {
    get_app_path().join("drinking_history.json")
}
