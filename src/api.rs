use anyhow::Result;
use serde::de::DeserializeOwned;

// TODO: Make this configurable.
const BASE_URL: &str = "http://localhost:3001";

fn get<T: DeserializeOwned>(path: &str) -> Result<T> {
    Ok(serde_json::from_str(
        &reqwest::blocking::get(format!("{}/{}", BASE_URL, path))?.text()?,
    )?)
}

pub fn list_equipment_classnames() -> Result<Vec<String>> {
    get("equipment/classnames")
}
