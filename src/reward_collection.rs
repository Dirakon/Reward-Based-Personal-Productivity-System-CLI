use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::crypto_utils;
use crate::io_utils::decode_file_from_moving;
use crate::io_utils::encode_file_by_moving;
use crate::io_utils::encode_file_by_moving_initial;
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
            _ => false,
        }
    }

    pub fn execute_reward(&mut self, state:&AppState) -> Option<()> {
        match self {
            RewardType::DecodeFiles(reward) => {
                let file_index = reward.choose_file("Enter index of file to decode: ")?;
                let chosen_file = &reward.files_to_decode[file_index];
                decode_file_from_moving(&chosen_file.path_after_encoding, &chosen_file.path_before_encoding);
                reward.files_to_decode.remove(file_index);
            }
        };
        return Some(());
    }
    pub fn activate_reward(&mut self, state:&AppState) -> Option<()> {
        match self {
            RewardType::DecodeFiles(reward) => {
                let file_index = reward.choose_file("Enter index of file to rent: ")?;
                reward.currently_decoded_file_index = Some(file_index);
                let chosen_file = &reward.files_to_decode[file_index];
                decode_file_from_moving(&chosen_file.path_after_encoding, &chosen_file.path_before_encoding);
            }
        };
        return Some(());
    }
    pub fn deactivate_reward(&mut self, state:&AppState) -> Option<()> {
        match self {
            RewardType::DecodeFiles(reward) => {
                let file_index = reward
                    .currently_decoded_file_index
                    .expect("No file is decoded even though reward is being deactivated!");
                let chosen_file = &reward.files_to_decode[file_index];
                let path_after_encoding = encode_file_by_moving(&chosen_file.path_after_encoding, &chosen_file.path_before_encoding);
                reward.currently_decoded_file_index = None;
            }
        };
        return Some(());
    }
}

#[derive(Serialize, Deserialize)]
pub struct DecodeFilesReward {
    pub files_to_decode: Vec<SingularFileToDecode>,
    pub currently_decoded_file_index: Option<usize>,
}
impl DecodeFilesReward {
    pub fn get_default() -> DecodeFilesReward {
        return DecodeFilesReward {
            files_to_decode: vec![],
            currently_decoded_file_index: None,
        };
    }
    pub fn add_new_file(&mut self, mut new_file: SingularFileToDecode, state:&AppState) {
        new_file.path_after_encoding =  encode_file_by_moving_initial(&state.folder_pool, &new_file.path_before_encoding);
        self.files_to_decode.push(new_file);
    }
    pub fn choose_file(&self, prompt: &str) -> Option<usize> {
        if self.files_to_decode.is_empty() {
            return None;
        }
        for i in 0..self.files_to_decode.len() {
            println!("{}.\t{}", i + 1, self.files_to_decode[i]);
        }
        let file_index: usize = get_parsed_line_with_condition(Some(prompt), |int_val: &usize| {
            *int_val > 0 && *int_val <= self.files_to_decode.len()
        }) - 1;
        return Some(file_index);
    }
}

#[derive(Serialize, Deserialize)]
pub struct SingularFileToDecode {
    pub path_before_encoding: String,
    pub reward_name: String,
    pub path_after_encoding: String
}

impl Display for SingularFileToDecode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        return write!(f, "{} --- {}", self.reward_name, self.path_before_encoding);
    }
}

#[derive(Serialize, Deserialize)]
pub struct RewardCollection {
    pub name: String,
    pub description: String,
    pub reward_type: RewardType,
    pub cost: f64,
    pub spending_protocol: RewardPointTransferProtocol,
}

impl RewardCollection {
    pub fn init_rewards_from_cli() -> RewardCollection {
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
            |int_val: &i64| *int_val == 1,
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
    pub fn tick_reward(&mut self, state:&AppState) -> TickResponse {
        let no_action_response = TickResponse { points_spent: 0.0 };
        match self.spending_protocol {
            RewardPointTransferProtocol::HourlyTransfer(starting_date) => {
                match starting_date {
                    Some(date) => {
                        if self.reward_type.deactivate_reward(state).is_none() {
                            return no_action_response;
                        };
                        self.spending_protocol = RewardPointTransferProtocol::HourlyTransfer(None);
                        return TickResponse {
                            points_spent: (Local::now().signed_duration_since(date).num_minutes()
                                as f64)
                                * (self.cost / 60.0),
                        };
                    }
                    None => {
                        if self.reward_type.activate_reward(state).is_none() {
                            return no_action_response;
                        };
                        self.spending_protocol = RewardPointTransferProtocol::HourlyTransfer(Some(Local::now()));
                        return TickResponse { points_spent: 0.0 };
                    }
                }
            }
            RewardPointTransferProtocol::SingularTransfer => {
                if self.reward_type.execute_reward(state).is_none() {
                    return no_action_response;
                };
                return TickResponse {
                    points_spent: self.cost,
                };
            }
        }
    }

    pub fn validate(&mut self) {
        match &mut self.reward_type {
            RewardType::DecodeFiles(decode_files_reward) => {
                let mut current_index_pre_retain = 0;
                decode_files_reward.files_to_decode.retain(
                    |file|{
                        let path = std::path::Path::new(&file.path_after_encoding);
                        if !path.exists(){
                            println!("File by path {} in reward collection {} does not exist anymore! Removing it...",
                            &file.path_after_encoding, self.name);
                            decode_files_reward.currently_decoded_file_index = match decode_files_reward.currently_decoded_file_index{
                                Some(index_in_question) => 
                                    if index_in_question == current_index_pre_retain{
                                        None
                                    }else if index_in_question > current_index_pre_retain{
                                        Some(index_in_question-1)
                                    }else{
                                        Some(index_in_question)
                                    }
                                ,
                                None => None,
                            };


                            current_index_pre_retain+=1;
                            return false;
                        }
                        current_index_pre_retain+=1;
                        return true;
                    }
                );
            },
        }
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
            self.name, self.description, reward_system_description, self.reward_type
        );
    }
}

impl Display for RewardType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let type_description = match self {
            RewardType::DecodeFiles(_) => "Decode files",
        };
        return write!(f, "{}", type_description);
    }
}
