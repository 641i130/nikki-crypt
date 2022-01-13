use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};
use std::io::stdin;

fn main() {
    println!("Password : ");
    let mut pass = String::new();
    stdin()
        .read_line(&mut pass)
        .expect("Failed to read pass!");

    // Password salting!!! Must make it random by applying hashes to it:

    let key = Key::from_slice(b"test");
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
