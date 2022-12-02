use std::{
    fs,
    io::{self, Read, Write},
};

use zip::{result::ZipResult, write::FileOptions};

use crate::crypto_utils::{decrypt, encrypt, Key};

pub fn encode_file(key: &Key, path: &String) {
    let mut file_bytes = fs::read(path).expect(&f!("Could not read file {} bytewise!", &path));
    encrypt(key, &mut file_bytes);
    fs::write(&path, file_bytes).expect(&f!("Could not write to file {} bytewise!", &path));
}

pub fn decode_file(key: &Key, path: &String) {
    let mut file_bytes = fs::read(&path).expect(&f!("Could not read file {} bytewise!", &path));
    decrypt(key, &mut file_bytes);
    fs::write(&path, file_bytes).expect(&f!("Could not write to file {} bytewise!", &path));
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
