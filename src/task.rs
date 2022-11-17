use std::fs;
use std::path::Path;

use crate::cli_utils::{
    self, get_line, get_line_with_condition, get_parsed_line, get_parsed_line_with_condition,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    is_task_regular: bool,
    reward_system: TaskRewardSystem,
    reward: i64,
    description: String,
    name: String,
}

impl Task {
    pub fn init_task_from_cli() -> Task {
        let name = get_line(Some("Enter the name of the task: "));
        let description = get_line(Some("Enter the description of the task: "));
        let is_task_regular = get_line_with_condition(Some("Is the task regular?(y/n)"), |line| {
            line.to_lowercase() == "y" || line.to_lowercase() == "n"
        })
        .to_lowercase()
            == "y";
        let reward_system = match get_parsed_line_with_condition(
            Some("Choose task reward type: \n1 - per hour reward\n2 - per completion reward"),
            |int_val| *int_val == 1 || *int_val == 2,
        ) {
            1 => TaskRewardSystem::PerHourReward,
            _ => TaskRewardSystem::PerCompletionReward,
        };
        let reward: i64 = get_parsed_line(Some("Enter the reward amount: "));
        return Task {
            is_task_regular,
            reward_system,
            reward,
            description,
            name,
        };
    }
}

#[derive(Serialize, Deserialize)]
pub enum TaskRewardSystem {
    PerHourReward,
    PerCompletionReward,
}
