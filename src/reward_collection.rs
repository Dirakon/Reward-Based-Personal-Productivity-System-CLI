use serde::{Deserialize, Serialize};

use crate::task::RewardPointTransferProtocol;

#[derive(Serialize, Deserialize)]
pub enum RewardType {
    DecodeFile(DecodeFilesReward),
}

#[derive(Serialize, Deserialize)]
pub struct DecodeFilesReward {
    files_to_decode: Vec<SingularFileToDecode>,
}

#[derive(Serialize, Deserialize)]
pub struct SingularFileToDecode {
    filepath: String,
    reward_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RewardCollection {
    pub name: String,
    pub description: String,
    pub is_regular: bool,
    pub reward_type: RewardType,
    pub cost: f64,
    pub spending_protocol:RewardPointTransferProtocol
}

impl RewardCollection {
    pub fn tick_reward(mut self){
        
    }
}