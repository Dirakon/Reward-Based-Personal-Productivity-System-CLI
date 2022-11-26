use chrono;
use chrono::DateTime;
use chrono::Local;
use std::fs;
use std::path::Path;

use crate::cli_utils::{
    self, get_line, get_line_with_condition, get_parsed_line, get_parsed_line_with_condition,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub is_regular: bool,
    pub reward_system: RewardPointTransferProtocol,
    pub reward: f64,
    pub description: String,
    pub name: String,
}

impl Task {
    pub fn init_task_from_cli() -> Task {
        chrono::Local::now();
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
            1 => RewardPointTransferProtocol::HourlyTransfer(None),
            _ => RewardPointTransferProtocol::SingularTransfer,
        };
        let reward: f64 = get_parsed_line(Some("Enter the reward amount: "));
        return Task {
            is_regular: is_task_regular,
            reward_system,
            reward,
            description,
            name,
        };
    }
    pub fn tick_task(&mut self) -> TickResponse {
        match self.reward_system {
            RewardPointTransferProtocol::HourlyTransfer(starting_date) => {
                self.reward_system =
                    RewardPointTransferProtocol::HourlyTransfer(if starting_date.is_some() {
                        None
                    } else {
                        Some(Local::now())
                    });
                match starting_date {
                    Some(date) => {
                        return TickResponse {
                            task_is_to_be_removed: !self.is_regular,
                            reward_acquired: (Local::now().signed_duration_since(date).num_minutes()
                                as f64)
                                * (self.reward / 60.0),
                        }
                    }
                    None => {
                        return TickResponse {
                            task_is_to_be_removed: false,
                            reward_acquired: 0.0,
                        }
                    }
                }
            }
            RewardPointTransferProtocol::SingularTransfer => {
                return TickResponse {
                    task_is_to_be_removed: !self.is_regular,
                    reward_acquired: self.reward,
                };
            }
        }
    }
}

pub struct TickResponse {
    pub task_is_to_be_removed: bool,
    pub reward_acquired: f64,
}
impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let reward_system_description = match self.reward_system {
            RewardPointTransferProtocol::SingularTransfer => {
                format!("Per-completion reward ({})", self.reward)
            }
            RewardPointTransferProtocol::HourlyTransfer(starting_date) => format!(
                "Per-hour reward ({}), {}",
                self.reward,
                if starting_date.is_some() {
                    "started"
                } else {
                    "not started"
                }
            ),
        };
        return write!(
            f,
            "name: {}\ndescription: {}\nis_regular: {}\nreward system description: {}",
            self.name, self.description, self.is_regular, reward_system_description
        );
    }
}
#[derive(Serialize, Deserialize)]
pub enum RewardPointTransferProtocol {
    HourlyTransfer(Option<DateTime<Local>>),
    SingularTransfer,
}
