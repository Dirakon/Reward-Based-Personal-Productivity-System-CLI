pub mod app_state;
pub mod cli_utils;
pub mod crypto_utils;
pub mod io_utils;
pub mod reward_collection;
pub mod task;

#[macro_use]
extern crate fstrings;

use app_state::AppState;
use reward_collection::RewardCollection;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::iter::Filter;
use std::path;
use std::path::Path;
use sysinfo::{ProcessExt, SystemExt};

use crate::cli_utils::get_parsed_line_with_condition;
use crate::crypto_utils::make_key;
use crate::io_utils::decode_file;
use crate::io_utils::encode_file;
use crate::reward_collection::DecodeFilesReward;
use crate::reward_collection::RewardType;
use crate::reward_collection::SingularFileToDecode;
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
        "ef" => start_reward_addition(args),
        "re" => {
            validate_rewards();
            reward_editing_loop();
        }
        _ => println!("Incorrect arguments!\n{}", arguments_description),
    }
}

fn validate_rewards() {
    let mut state = app_state::AppState::load_from_disk();
    state
        .rewards
        .iter_mut()
        .for_each(|reward| reward.validate());
    state.save_on_disk();
}

fn reward_editing_loop() {
    let mut state = app_state::AppState::load_from_disk();
    loop {
        println!("\n\n\n");
        println!("Points: {}", state.cur_points);
        edit_rewards(&mut state);
        state.save_on_disk();
    }
}
fn task_editing_loop() {
    let mut state = app_state::AppState::load_from_disk();
    loop {
        println!("\n\n\n");
        println!("Points: {}", state.cur_points);
        edit_tasks(&mut state);
        state.save_on_disk();
    }
}

fn start_reward_addition(args: Vec<String>) {
    let mut state = AppState::load_from_disk();

    let path_as_string = args.get(2).expect("Missing file path argument!").clone();
    let path_to_file = Path::new(&path_as_string);
    if !path_to_file.exists() {
        panic!("Path {} does not exists!", path_as_string);
    }
    if path_to_file.is_dir() {
        match add_rewards_in_folder(state, path_to_file, &args) {
            Err(err) => panic!("{:?}", err),
            Ok(new_state) => state = new_state,
        };
    } else {
        state = add_rewards_in_file(state, path_to_file, &args);
    }
    state.save_on_disk();
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
            state = add_rewards_in_file(state, &entry.path(), args);
        }
    }
    return Ok(state);
}

fn add_rewards_in_file(mut state: AppState, file_path: &Path, args: &Vec<String>) -> AppState {
    let reward_name = if args.len() >= 5 {
        args[4].clone()
    } else {
        file_path
            .file_name()
            .expect(&f!("Could not get file name of {:?}", file_path))
            .to_str()
            .expect(&f!(
                "Could not convert file name of {:?} to string",
                file_path
            ))
            .to_string()
    };

    state = reward_addition(
        state,
        file_path
            .to_str()
            .expect(&f!("Could not convert path {:?} to string", file_path))
            .to_string(),
        args.get(3)
            .expect("Missing collection name argument!")
            .to_string(),
        reward_name,
    );
    return state;
}
fn reward_addition(
    mut state: AppState,
    file_to_encode: String,
    reward_collection: String,
    reward_name: String,
) -> AppState {
    println!(
        "Adding reward to {} by name {}: path is {}",
        reward_collection, reward_name, file_to_encode
    );

    let collection_to_append = state
        .rewards
        .iter_mut()
        .find_map(|collection| {
            if collection.name == reward_collection {
                match &mut collection.reward_type {
                    RewardType::DecodeFiles(decode_files_reward) => Some(decode_files_reward),
                    _ => None
                }
            } else {
                None
            }
        })
        .expect(&f!(
            "Could not find decodeFiles collection by name {}",
            reward_collection
        ));

    let new_file = SingularFileToDecode {
        filepath: file_to_encode,
        reward_name,
    };
    collection_to_append.add_new_file(new_file);

    return state;
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
            tick_task(state);
        }
        2 => {
            remove_task(state);
        }
        _ => {
            add_task(state);
        }
    }
}

fn add_task(state: &mut AppState) {
    let new_task = Task::init_task_from_cli();
    state.tasks.push(new_task);
}

fn remove_task(state: &mut AppState) {
    let index_to_remove: usize = get_parsed_line_with_condition(
        Some("Enter index of task to remove: "),
        |int_val: &usize| *int_val > 0 && *int_val <= state.tasks.len(),
    ) - 1;
    state.tasks.remove(index_to_remove);
}

fn tick_task(state: &mut AppState) {
    let index_to_tick: usize =
        get_parsed_line_with_condition(Some("Enter index of task to tick: "), |int_val: &usize| {
            *int_val > 0 && *int_val <= state.tasks.len()
        }) - 1;
    let tick_response = state.tasks[index_to_tick].tick_task();
    if tick_response.task_is_to_be_removed {
        state.tasks.remove(index_to_tick);
    }
    state.cur_points += tick_response.reward_acquired;
}

fn edit_rewards(state: &mut AppState) {
    if !state.rewards.is_empty() {
        println!("Rewards: ");
        for i in 0..state.rewards.len() {
            println!("---Reward #{}---", i + 1);
            println!("{}", state.rewards[i]);
        }
        println!("------");
    }
    let prompt = if state.rewards.is_empty() {
        "1 - add reward"
    } else if state.cur_points <= 0.0 {
        "1 - add reward\n2 - remove reward"
    } else {
        "1 - add reward\n2 - remove reward\n3 - tick reward"
    };

    match get_parsed_line_with_condition(Some(prompt), |int_val| {
        return (*int_val == 2 && !state.rewards.is_empty())
            || (*int_val == 3 && !state.rewards.is_empty() && state.cur_points > 0.0)
            || *int_val == 1;
    }) {
        3 => {
            tick_reward(state);
        }
        2 => {
            remove_reward(state);
        }
        _ => {
            add_reward(state);
        }
    }
}

fn add_reward(state: &mut AppState) {
    let new_reward = RewardCollection::init_rewards_from_cli();
    state.rewards.push(new_reward);
}

fn remove_reward(state: &mut AppState) {
    let index_to_remove: usize = get_parsed_line_with_condition(
        Some("Enter index of reward to remove: "),
        |int_val: &usize| *int_val > 0 && *int_val <= state.rewards.len(),
    ) - 1;
    state.rewards.remove(index_to_remove);
}

fn tick_reward(state: &mut AppState) {
    let index_to_tick: usize = get_parsed_line_with_condition(
        Some("Enter index of reward to tick: "),
        |int_val: &usize| *int_val > 0 && *int_val <= state.rewards.len(),
    ) - 1;
    let tick_response = state.rewards[index_to_tick].tick_reward();
    state.cur_points -= tick_response.points_spent;
}
