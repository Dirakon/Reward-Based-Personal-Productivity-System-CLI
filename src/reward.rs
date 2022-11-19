use std::path::Path;

struct Reward{
    pub rewardType:RewardType
}

pub enum RewardType{
    DecodeFile(String)
}