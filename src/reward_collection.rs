use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::task::RewardPointTransferProtocol;
use chrono;
use chrono::DateTime;
use chrono::Local;


use crate::cli_utils::{
    self, get_line, get_line_with_condition, get_parsed_line, get_parsed_line_with_condition,
};

#[derive(Serialize, Deserialize)]
pub enum RewardType {
    DecodeFiles(DecodeFilesReward),
}
impl RewardType {
    pub fn is_decode_files(&self) -> bool {
        match self {
            RewardType::DecodeFiles(_) => true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DecodeFilesReward {
    pub files_to_decode: Vec<SingularFileToDecode>,
}
impl DecodeFilesReward {
    pub fn get_default()->DecodeFilesReward{
        return DecodeFilesReward { files_to_decode: vec![] }
    }
    pub fn add_new_file(mut self, mut new_file : SingularFileToDecode) -> DecodeFilesReward{
        // TODO: encode file and stuff

        return self;
    }
}

#[derive(Serialize, Deserialize)]
pub struct SingularFileToDecode {
    pub filepath: String,
    pub reward_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RewardCollection {
    pub name: String,
    pub description: String,
    pub reward_type: RewardType,
    pub cost: f64,
    pub spending_protocol:RewardPointTransferProtocol
}

impl RewardCollection {
    pub fn init_rewards_from_cli() -> RewardCollection {
        chrono::Local::now();
        let name = get_line(Some("Enter the name of the reward: "));
        let description = get_line(Some("Enter the description of the reward: "));
        let spending_protocol = match get_parsed_line_with_condition(
            Some("Choose reward cost type: \n1 - per hour cost\n2 - one-time cost"),
            |int_val| *int_val == 1 || *int_val == 2,
        ) {
            1 => RewardPointTransferProtocol::HourlyTransfer(None),
            _ => RewardPointTransferProtocol::SingularTransfer,
        };
        let cost: f64 = get_parsed_line(Some("Enter the cost amount: "));

        let reward_type = match get_parsed_line_with_condition(
            Some("Choose reward type: \n1 - decode files"),
            |int_val:&i64| *int_val == 1,
        ) {
            _ => RewardType::DecodeFiles(DecodeFilesReward::get_default()),
        };


        return RewardCollection {
            spending_protocol,
            cost,
            description,
            name,
            reward_type,
        };
    }
    pub fn tick_reward(&mut self)->TickResponse{
        match self.spending_protocol {
            RewardPointTransferProtocol::HourlyTransfer(starting_date) => {
                self.spending_protocol = RewardPointTransferProtocol::HourlyTransfer(if starting_date.is_some() {
                    None
                } else {
                    Some(Local::now())
                });
                match starting_date {
                    Some(date) => {
                        self.reward_type = self.deactivate_reward();
                        return TickResponse {
                            points_spent: (Local::now().signed_duration_since(date).num_minutes()
                                as f64)
                                * (self.cost / 60.0),
                        }
                    }
                    None => {
                        self.reward_type = self.activate_reward();
                        return TickResponse {
                            points_spent: 0.0,
                        }
                    }
                }
            }
            RewardPointTransferProtocol::SingularTransfer => {
                self.reward_type = self.execute_reward();
                return TickResponse {
                    points_spent: self.cost,
                };
            }
        }
        
    }
    pub fn execute_reward(& self)->RewardType{
        match self.reward_type {
            RewardType::DecodeFiles(file_list) => {
                // TODO: choose file to decode and remove from the list
            },
        };
    }
    pub fn activate_reward(& self)->RewardType{
        match self.reward_type {
            RewardType::DecodeFiles(file_list) => {
                // TODO: choose file to decode and save the fact that the file is being rented
            },
        };
    }
    pub fn deactivate_reward(& self)->RewardType{
        match self.reward_type {
            RewardType::DecodeFiles(file_list) => {
                // TODO: encode decoded previously file
            },
        };
    }
}
pub struct TickResponse {
    pub points_spent: f64,
}

impl Display for RewardCollection {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let reward_system_description = match self.spending_protocol {
            RewardPointTransferProtocol::SingularTransfer => {
                format!("One-time payment ({})", self.cost)
            }
            RewardPointTransferProtocol::HourlyTransfer(starting_date) => format!(
                "Per-hour payment ({}), {}",
                self.cost,
                if starting_date.is_some() {
                    "started"
                } else {
                    "not started"
                }
            ),
        };
        return write!(
            f,
            "name: {}\ndescription: {}\nreward system description: {}\ntype:{}",
            self.name, self.description, reward_system_description,self.reward_type
        );
    }
}

impl Display for RewardType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let type_description = match self {
            RewardType::DecodeFiles(_) => "Decode files",
        };
        return write!(
            f,
            "{}",
            type_description
        );
    }
}