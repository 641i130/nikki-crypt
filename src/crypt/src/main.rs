use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use std::io::stdin;


use sodiumoxide::crypto::pwhash::argon2id13;
use std::time::Instant;


pub fn hash(passwd: &str) -> argon2id13::HashedPassword {
    sodiumoxide::init().unwrap();
    let hash = argon2id13::pwhash(
        passwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    let texthash = std::str::from_utf8(&hash.0).unwrap().to_string();
    //(texthash, hash)
    hash
}


fn main(){
    println!("Password : ");
    let mut password = String::new();
    stdin()
        .read_line(&mut password)
        .expect("Failed to read password!");

    // Password salting!!! Must make it random by applying hashes to it:
    let hash = hash(&password);


    let key = Key::from_slice(&hash.to_bytes());
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message; We will randomize this later TODO

    // Get plaintext
    println!("Plaintext : ");
    let mut plaintext = String::new();
    stdin()
        .read_line(&mut plaintext)
        .expect("Failed to read plaintext in!");

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes().as_ref())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

    println!("Ciphertext : {:X?}",ciphertext);

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

    println!("Decrypted : {:?}",plaintext);
    //print!("Ciphertext : ");
    //io::stdout().write(ciphertext);

}
