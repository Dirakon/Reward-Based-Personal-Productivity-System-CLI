pub mod app_state;
pub mod cli_utils;
pub mod reward;
pub mod task;

use app_state::AppState;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use sysinfo::{ProcessExt, SystemExt};

use crate::cli_utils::get_parsed_line_with_condition;
use crate::task::Task;
fn main() {
    if !Path::new(app_state::get_app_state_filepath()).exists() {
        app_state::initialize_default_app_state();
    }

    let args: Vec<String> = env::args().collect();
    let arguments_description = "\top - opens in operational mode, with task editing and such.\n\tef \"*reward collection*\" \"*reward name (optional)*\" - encodes file for reward in the given reward collection (default name is the file name)";
    println!("{:?}", args);
    if args.len() <= 1 {
        println!("No command line arguments!\n{}", arguments_description);
        return;
    }
    match args[1].as_str() {
        "op" => control_loop(),
        "ar" => {
            if start_reward_addition(args).is_none() {
                println!("Incorrect arguments!\n{}", arguments_description);
            }
        }
        _ => println!("Incorrect arguments!\n{}", arguments_description),
    }

    // for (pid, process) in state.sys.processes() {
    //     println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    // }
    // println!("{}",state.cur_points);
}

fn control_loop() {
    let mut state = app_state::AppState::load_app_state();
    state.sys.refresh_all();
    loop {
        edit_tasks(&mut state);
        state.update_app_state();
    }
}

fn start_reward_addition(args: Vec<String>) -> Option<()> {
    let path_as_string = args.get(2)?.clone();
    let path_to_file = Path::new(&path_as_string);
    let reward_name = if args.len() >= 5 {
        args[4].clone()
    } else {
        path_to_file.file_name()?.to_str()?.to_string()
    };

    reward_addition(path_as_string, args.get(3)?.to_string(), reward_name);
    return Some(());
}

fn reward_addition(file_to_encode: String, reward_collection: String, reward_name: String) {}

fn edit_tasks(state: &mut AppState) {
    if !state.tasks.is_empty() {
        println!("Tasks: ");
        for i in 0..state.tasks.len() {
            println!("---Task #{}---", i + 1);
            println!("{}", state.tasks[i]);
        }
        println!("------");
    }
    let prompt = if state.tasks.is_empty() {
        "1 - add task"
    } else {
        "1 - add task\n2 - remove task\n3 - tick task"
    };

    match get_parsed_line_with_condition(Some(prompt), |int_val| {
        return (*int_val == 2 && !state.tasks.is_empty())
            || (*int_val == 3 && !state.tasks.is_empty())
            || *int_val == 1;
    }) {
        3 => {
            // Tick task
            let index_to_tick: usize = get_parsed_line_with_condition(
                Some("Enter index of task to complete: "),
                |int_val: &usize| *int_val > 0 && *int_val <= state.tasks.len(),
            ) - 1;
            let tick_response = state.tasks[index_to_tick].tick_task();
            if tick_response.task_is_to_be_removed {
                state.tasks.remove(index_to_tick);
            }
            state.cur_points += tick_response.reward_acquired;
        }
        2 => {
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
}
