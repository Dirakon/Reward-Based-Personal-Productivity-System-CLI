
use std::fs;

use crate::crypto_utils::{Key, encrypt,decrypt};

pub fn encode_file(key:&Key,path:String){
    let mut file_bytes = fs::read(&path).expect(&f!("Could not read file {} bytewise!", &path));
    encrypt(key, &mut file_bytes);
    fs::write(&path, file_bytes).expect(&f!("Could not write to file {} bytewise!", &path));
}

pub fn decode_file(key:&Key,path:String){
    let mut file_bytes = fs::read(&path).expect(&f!("Could not read file {} bytewise!", &path));
    decrypt(key, &mut file_bytes);
    fs::write(&path, file_bytes).expect(&f!("Could not write to file {} bytewise!", &path));
}