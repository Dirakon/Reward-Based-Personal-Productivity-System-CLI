pub mod app_state;
pub mod cli_utils;
pub mod reward;
pub mod reward_collection;
pub mod task;

use app_state::AppState;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::path;
use std::path::Path;
use sysinfo::{ProcessExt, SystemExt};

use crate::cli_utils::get_parsed_line_with_condition;
use crate::task::Task;
fn main() {
    if !Path::new(app_state::get_app_state_filepath()).exists() {
        app_state::initialize_default_app_state();
    }

    let args: Vec<String> = env::args().collect();
    let arguments_description = "\tte - opens in task editing mode, with task ticking and such.\n\tef - encodes file for reward in the given reward collection (default name is the file name)\n\t\tArg1: \"*path to file or directory to be encoded*\" \n\t\tArg2: \"*reward collection*\" \n\t\tArg3: \"*reward name (optional)*\"\n\tre - opens in reward editing mode, where you can buy/edit rewards";
    println!("{:?}", args);
    if args.len() <= 1 {
        println!("No command line arguments!\n{}", arguments_description);
        return;
    }
    match args[1].as_str() {
        "te" => task_editing_loop(),
        "ef" => {
            if start_reward_addition(args).is_none() {
                println!("Incorrect arguments!\n{}", arguments_description);
            }
        }
        "re" => reward_editing_loop(),
        _ => println!("Incorrect arguments!\n{}", arguments_description),
    }

    // for (pid, process) in state.sys.processes() {
    //     println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    // }
    // println!("{}",state.cur_points);
}

fn reward_editing_loop() {
    let mut state = app_state::AppState::load_app_state();
    loop {
        println!("\n\n\n");
        println!("Points: {}", state.cur_points);
        edit_rewards(&mut state);
        state.update_app_state();
    }
}
fn task_editing_loop() {
    let mut state = app_state::AppState::load_app_state();
    loop {
        println!("\n\n\n");
        println!("Points: {}", state.cur_points);
        edit_tasks(&mut state);
        state.update_app_state();
    }
}

fn start_reward_addition(args: Vec<String>) -> Option<()> {
    let state = AppState::load_app_state();

    let path_as_string = args.get(2)?.clone();
    let path_to_file = Path::new(&path_as_string);
    if !path_to_file.exists() {
        return None;
    }
    if path_to_file.is_dir() {
        match add_rewards_in_folder(state, path_to_file, &args) {
            Err(err) => println!("{:?}", err),
            _ => (),
        };
    } else {
        add_rewards_in_file(state, path_to_file, &args);
    }
    return Some(());
}

fn add_rewards_in_folder(
    mut state: AppState,
    folder_path: &Path,
    args: &Vec<String>,
) -> io::Result<AppState> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            state = add_rewards_in_folder(state, &path, args)?;
        } else {
            match add_rewards_in_file(state, &entry.path(), args) {
                Some(new_state) => state = new_state,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "problems working with specific file",
                    ))
                }
            };
        }
    }
    return Ok(state);
}

fn add_rewards_in_file(state: AppState, file_path: &Path, args: &Vec<String>) -> Option<AppState> {
    let reward_name = if args.len() >= 5 {
        args[4].clone()
    } else {
        file_path.file_name()?.to_str()?.to_string()
    };

    reward_addition(file_path, args.get(3)?.to_string(), reward_name);
    return Some(state);
}
fn reward_addition(file_to_encode: &Path, reward_collection: String, reward_name: String) {
    println!(
        "Adding reward to {} by name {}: path is {}",
        reward_collection,
        reward_name,
        file_to_encode.to_string_lossy()
    );
}

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
                Some("Enter index of task to tick: "),
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

fn edit_rewards(state: &mut AppState){

}