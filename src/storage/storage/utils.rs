use std::fs;
use std::path::PathBuf;

pub fn get_app_path() -> PathBuf {
    let path = PathBuf::from("/data/user/0/com.ariane.BoozeBuddy/files");
    fs::create_dir_all(&path) // creates a directory if it doesn't exist
        .expect("storage::utilities::get_app_path: Failed to create directory");
    path
}

pub fn save_json<T: serde::Serialize>(path: PathBuf, data: &T) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string_pretty(data)?;
    fs::write(path, json).expect("storage::utilities::save_json: Failed to save data");
    Ok(())
}

pub fn load_json<T: serde::de::DeserializeOwned + Default>(
    path: PathBuf,
) -> Result<T, serde_json::Error> {
    let json = fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default();
    Ok(json)
}
