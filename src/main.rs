
pub mod task;
pub mod app_state;
pub mod cli_utils;


use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use sysinfo::{SystemExt, ProcessExt};
fn main() {
    if !Path::new(app_state::get_app_state_filepath()).exists() {
        app_state::initialize_default_app_state();
    }

    let mut state = app_state::AppState::load_app_state();

    state.sys.refresh_all();
    for (pid, process) in state.sys.processes() {
        println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }
    println!("{}",state.cur_points);
}