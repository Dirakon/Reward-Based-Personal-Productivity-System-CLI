pub mod app_state;
pub mod cli_utils;
pub mod task;

use app_state::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use sysinfo::{ProcessExt, SystemExt};

use crate::cli_utils::get_parsed_line_with_condition;
use crate::task::Task;
fn main() {
    if !Path::new(app_state::get_app_state_filepath()).exists() {
        app_state::initialize_default_app_state();
    }

    let mut state = app_state::AppState::load_app_state();

    state.sys.refresh_all();
    // for (pid, process) in state.sys.processes() {
    //     println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    // }
    // println!("{}",state.cur_points);
    control_loop(state);
}

fn control_loop(mut state: AppState) {
    println!("Tasks: ");
    for i in 0..state.tasks.len() {
        println!("---Task #{}---", i + 1);
        println!("{}", state.tasks[i]);
    }
    println!("------");

    match get_parsed_line_with_condition(Some("1 - remove task\n2 - add task"), |int_val| {
        return *int_val == 1 || *int_val == 2;
    }) {
        1 => {
            // Remove task
            let index_to_remove: usize = get_parsed_line_with_condition(
                Some("Enter index of task to remove: "),
                |int_val: &usize| *int_val > 0 && *int_val <= state.tasks.len(),
            ) - 1;

            state.tasks.remove(index_to_remove);
        }
        _ => {
            // Add task
            let new_task = Task::init_task_from_cli();
            state.tasks.push(new_task);
        }
    }
    control_loop(state);
}
