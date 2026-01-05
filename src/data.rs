use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, env::home_dir};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub api_key: String,
    pub fetch_interval: u128,
    pub last_fetch: u128,
    pub filters: Vec<(String, String)>,
    pub tasks: HashMap<String, Vec<Task>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    pub time: Option<u128>,
}

fn get_path() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(".remembrall");

    path
}

pub fn init_data(api_key: String) {
    let path = get_path();
    fs::File::create(&path).unwrap();
    fs::write(
        &path,
        serde_json::to_string_pretty(&Data {
            api_key: api_key,
            fetch_interval: 5 * 60 * 1000,
            last_fetch: 0,
            filters: vec![],
            tasks: HashMap::new(),
        })
        .unwrap(),
    )
    .unwrap();
}

pub fn load_data() -> Option<Data> {
    let path = get_path();

    if !path.exists() {
        return None;
    }

    let data = serde_json::from_slice::<Data>(fs::read(path).unwrap().as_slice()).unwrap();

    Some(data)
}

pub fn save_data(data: &Data) {
    let path = get_path();

    if !path.exists() {
    }

    fs::write(&path, serde_json::to_string_pretty::<Data>(&data).unwrap()).unwrap();
}