use std::time::{Duration, UNIX_EPOCH};

use serde_json::Value;
pub mod header;
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub ip: String,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Event {
    /// IP
    Join(String, String),
    Err(String, String),
}

pub fn get_event(text: &str) -> Event {
    let text = text.trim_end_matches("\0");
    let data = serde_json::from_str::<Value>(text);
    if data.is_err() {
        return Event::Err("Incorrect Format".to_string(), text.to_string());
    }
    let data = data.unwrap();

    if data["type"].as_str().is_none() {
        return Event::Err("Unknown type".to_string(), text.to_string());
    }
    let event_type = data["type"].as_str().unwrap();

    match event_type {
        "join" => {
            if serde_json::from_value::<header::Join>(data.clone()).is_err() {
                return Event::Err("Incorrect Format".to_string(), text.to_string());
            }
            let data = serde_json::from_value::<header::Join>(data.clone()).unwrap();
            return Event::Join(data.ip, data.name);
        }
        _ => return Event::Err("Unknown type".to_string(), text.to_string()),
    }
}

pub fn timestamp() -> usize {
    std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis() as usize
}
