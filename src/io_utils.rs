use rand::Rng;
use std::{
    fs,
    io::{self, Read, Write},
};
use zip::{result::ZipResult, write::FileOptions};

use crate::crypto_utils::{decrypt, encrypt, Key};

pub fn generate_name() -> String {
    let possible_characters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let rng = rand::thread_rng();
    let filename_size = rng.gen_range(3..20);
    let name: String = (0..filename_size)
        .map(|_| possible_characters[rng.gen_range(0..possible_characters.len())])
        .collect();
    return name;
}

pub fn encode_file_by_moving(folder_pool: Vec<String>, path: &String) -> String {
    let rng = rand::thread_rng();
    let new_path = folder_pool[rng.gen_range(0..folder_pool.len())] + &generate_name();
    fs::rename(path, new_path).expect(&f!("Couldn't move file {} back to {}!", path, new_path));
    return new_path;
}
pub fn decode_file_from_moving(current_path: &String, initial_path: &String) {
    fs::rename(current_path, initial_path).expect(&f!(
        "Couldn't move file {} back to {}!",
        current_path,
        initial_path
    ));
}

pub fn encode_file_by_compression(key: &Key, path: &String) {
    let mut file_bytes = fs::read(path).expect(&f!("Could not read file {} bytewise!", &path));

    // unneeded since compressing/decompressing data is sufficient for now
    // encrypt(key, &mut file_bytes);

    let mut encoded_file_bytes = Vec::new();
    let mut encoder = zstd::stream::Encoder::new(&mut encoded_file_bytes, 1).unwrap();

    io::copy(&mut file_bytes.as_slice(), &mut encoder).unwrap();
    encoder.finish().unwrap();

    fs::write(&path, encoded_file_bytes).expect(&f!("Could not write to file {} bytewise!", &path));
}

pub fn decode_file_from_compression(key: &Key, path: &String) {
    let mut file_bytes = fs::read(&path).expect(&f!("Could not read file {} bytewise!", &path));

    let mut decoded_file_bytes = Vec::new();
    let mut binding = file_bytes.as_slice();
    let mut decoder = zstd::stream::Decoder::new(&mut binding).unwrap();

    io::copy(&mut decoder, &mut decoded_file_bytes).unwrap();
    decoder.finish();

    // unneeded since compressing/decompressing data is sufficient for now
    // decrypt(key, &mut decoded_file_bytes);

    fs::write(&path, decoded_file_bytes).expect(&f!("Could not write to file {} bytewise!", &path));
}

pub fn encode_file_unstable(key: &Key, path: &String) {
    let mut file_bytes = fs::read(path).expect(&f!("Could not read file {} bytewise!", &path));
    encrypt(key, &mut file_bytes);
    fs::remove_file(path).expect(&f!(
        "Couldn't remove file after encrypting it's data for {}",
        path
    ));
    create_archive(&file_bytes, path).expect(&f!(
        "Couldn't create the archive with encrypted data for {}",
        path
    ));
}

pub fn decode_file_unstable(key: &Key, path: &String) {
    let mut file_bytes = extract_from(path).expect(&f!("couldn't extract from archive {}", path));

    fs::remove_file(&path).expect(&f!(
        "couldn't remove archive at {} after extracting the data",
        &path
    ));

    decrypt(key, &mut file_bytes);
    fs::write(&path, file_bytes).expect(&f!("Could not write to file {} bytewise!", &path));
}

pub fn create_archive(bytes_to_write: &Vec<u8>, archive_path: &String) -> ZipResult<()> {
    let path = std::path::Path::new(archive_path);
    let file = std::fs::File::create(path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Zstd)
        .unix_permissions(0o755);
    zip.start_file("main", options)?;
    zip.write(bytes_to_write)?;

    zip.finish()?;
    Ok(())
}
pub fn extract_from(archive_path: &String) -> ZipResult<Vec<u8>> {
    // Open archive and extract the file
    let path = std::path::Path::new(&archive_path);
    let file = fs::File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut file = archive.by_index(0)?; //.expect("INVALID PASSWORD!");

    // read the whole file
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
