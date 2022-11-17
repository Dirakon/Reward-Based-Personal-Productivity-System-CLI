use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::task;
pub fn get_app_state_filepath() -> &'static str {
    return "./state";
}

pub fn initialize_default_app_state() {
    &AppState::get_default().update_app_state();
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    cur_points: i64,
    tasks: Vec<task::Task>,
}

impl AppState {
    fn get_default() -> AppState {
        return AppState {
            cur_points: (0),
            tasks: Vec::new(),
        };
    }

    pub fn load_app_state() -> AppState {
        let contents = fs::read_to_string(get_app_state_filepath()).expect("read from file failed");
        return serde_json::from_str(&contents).unwrap();
    }
    
    pub fn update_app_state(&self) {
        fs::write(
            get_app_state_filepath(),
            serde_json::to_string(&self).unwrap(),
        )
        .expect("write to file failed");
    }
    
}
