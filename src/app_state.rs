use std::fs;
use std::path::Path;

use serde::{de::IgnoredAny, Deserialize, Deserializer, Serialize};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use crate::{crypto_utils, reward_collection::RewardCollection, task};
pub fn get_app_state_filepath() -> &'static str {
    return "./state";
}

pub fn initialize_default_app_state() {
    AppState::get_default().save_on_disk();
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppState {
    pub cur_points: f64,
    pub tasks: Vec<task::Task>,
    pub rewards: Vec<RewardCollection>,
    pub folder_pool: Vec<String>,
    #[serde(default, deserialize_with = "skip", skip_serializing)]
    pub sys: System,
}

fn skip<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default,
{
    // Ignore the data in the input.
    IgnoredAny::deserialize(deserializer)?;
    Ok(T::default())
}

impl AppState {
    fn get_default() -> AppState {
        return AppState {
            cur_points: (0.0),
            tasks: Vec::new(),
            folder_pool: Vec::new(),
            rewards: Vec::new(),
            sys: System::new_all(),
        };
    }

    pub fn load_from_disk() -> AppState {
        let contents = fs::read_to_string(get_app_state_filepath()).expect("read from file failed");
        let mut state: AppState = serde_json::from_str(&contents).unwrap();
        state.sys.refresh_all();
        return state;
    }

    pub fn save_on_disk(&self) {
        fs::write(
            get_app_state_filepath(),
            serde_json::to_string(&self).unwrap(),
        )
        .expect("write to file failed");
    }
}
