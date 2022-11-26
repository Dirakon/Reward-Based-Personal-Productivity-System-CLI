use ring::{aead::*, rand}; // these look relevant

/// AES-256 has 256-bit keys
pub type Key = [u8; 256 / 8];

/// we need an encryption key
pub fn make_key() -> Key {
    // we need an UnboundKey for doing crypto
    let rng = rand::SystemRandom::new(); // this has SecureRandom, which rand::generate wants
    rand::generate(&rng).unwrap().expose() // generate the key
}

pub fn encrypt(key: &Key, data: &mut Vec<u8>) {
    let key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key).unwrap()); // not sure why it's less safe but it has a simpler API
    let nonce = Nonce::assume_unique_for_key([0u8; 12]); // this is probably a bad idea
    key.seal_in_place_append_tag(nonce, Aad::empty(), data)
        .unwrap(); // I think this does encryption
}

pub fn decrypt(key:&Key, data: &mut Vec<u8>) {
    let key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key).unwrap());
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);
    key.open_in_place(nonce, Aad::empty(), data).unwrap(); // I think this does decryption
    data.truncate(data.len() - AES_256_GCM.tag_len()); // remove the garbage on the end
}
