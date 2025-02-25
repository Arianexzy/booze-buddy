use super::utils::*;
use crate::storage::{
    models::{Achievement, DrinkType, DrinkingHistory, DrinkingSession},
    storage::{
        error::{StorageError, StorageResult},
        settings::get_user,
    },
};
use parking_lot::Mutex;
use std::path::PathBuf;

static HISTORY_STORAGE: Mutex<Option<DrinkingHistory>> = Mutex::new(None);

// CORE STORAGE FUNCTIONS

fn with_current_session<F, R>(f: F) -> StorageResult<R>
where
    F: FnOnce(&mut DrinkingSession) -> R,
{
    with_history(|history| {
        let session = history
            .current_session()
            .ok_or(StorageError::NoCurrentSession)?;
        Ok(f(session))
    })
}

fn with_history<F, R>(f: F) -> R
where
    F: FnOnce(&mut DrinkingHistory) -> R,
{
    let mut storage = HISTORY_STORAGE.lock();
    if storage.is_none() {
        *storage = Some(load_data().expect("Failed to load history data"));
    }
    f(storage.as_mut().unwrap())
}

// PUBLIC API

pub fn has_active_session() -> bool {
    with_history(|history| history.current_session().is_some())
}

pub fn get_total_drinks() -> StorageResult<i32> {
    with_current_session(|session| session.total_drinks())
}

pub fn get_count_by(drink_type: DrinkType) -> StorageResult<i32> {
    with_current_session(|session| session.count_by(drink_type))
}

pub fn get_current_bac() -> StorageResult<f32> {
    let user = get_user();
    with_current_session(|session| session.calculate_bac(&user))
}

pub fn get_all_achievements() -> StorageResult<Vec<Achievement>> {
    with_current_session(|session| session.achievements.clone())
}

pub fn get_newly_unlocked_achievements() -> StorageResult<Vec<Achievement>> {
    let user = get_user();
    with_current_session(|session| {
        let newly_unlocked = session.check_achievements(&user);
        newly_unlocked
    })
}

pub fn add_drink_by(drink_type: DrinkType) -> StorageResult<()> {
    with_history(|history| {
        let session = history.ensure_active_session();
        session.add_drink_by(drink_type);
        save_data(history)
            .map_err(|e| StorageError::SaveFailed(format!("add_drink_by failed: {}", e)))?;
        Ok(())
    })
}

pub fn remove_last(drink_type: DrinkType) -> StorageResult<()> {
    with_history(|history| {
        let session = history.ensure_active_session();
        session.remove_last(drink_type);
        save_data(history)
            .map_err(|e| StorageError::SaveFailed(format!("remove_last failed: {}", e)))?;
        Ok(())
    })
}

pub fn end_current_session() -> StorageResult<()> {
    with_history(|history| {
        history.end_current_session();
        save_data(history)
            .map_err(|e| StorageError::SaveFailed(format!("end_current_session failed: {}", e)))?;
        Ok(())
    })
}

pub fn get_past_sessions() -> StorageResult<Vec<DrinkingSession>> {
    Ok(with_history(|history| {
        history.past_sessions().into_iter().cloned().collect()
    }))
}

// PRIVATE HELPER FUNCTIONS

fn save_data(data: &DrinkingHistory) -> StorageResult<()> {
    let path = get_history_path();
    save_json(path, data).map_err(|e| StorageError::SaveFailed(e.to_string()))
}

fn load_data() -> StorageResult<DrinkingHistory> {
    let path = get_history_path();
    load_json(path).map_err(|e| StorageError::LoadFailed(format!("load_data failed: {}", e)))
}

fn get_history_path() -> PathBuf {
    get_app_path().join("drinking_history.json")
}
