
pub mod task;
pub mod app_state;
pub mod cli_utils;

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
fn main() {
    if !Path::new(app_state::get_app_state_filepath()).exists() {
        app_state::initialize_default_app_state();
    }

    let state = app_state::AppState::load_app_state();

}