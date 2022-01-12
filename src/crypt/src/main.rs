use aes::Aes256;
use std::io::stdin;
use std::{io, io::Write};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

fn show(bs: &[u8]) -> String {
    String::from_utf8_lossy(bs).into_owned()
}

fn main() {
    println!();
    let plaintext = b"Hello world!";
    // create an alias for convenience
    type Aes256Cbc = Cbc<Aes256, Pkcs7>;

    // Get plaintext
    println!("Plaintext : ");
    let mut plaintext = String::new();
    stdin()
        .read_line(&mut plaintext)
        .expect("Failed to read plaintext in!");

    let key = hex!("0000102030405060708090a0b0c0d0e0f00102030405060708090a0b0c0d0e0f"); // 32 hex length; 128 bytes? 32*4? MODDED to be 256 which is now 64 hex in length 256 bytes!
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    // buffer must have enough space for message+padding
    let mut buffer = [0u8; 32];
    // copy message to the buffer
    let pos = plaintext.len();
    buffer[..pos].copy_from_slice(plaintext.as_bytes());
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    //assert_eq!(ciphertext, hex!("1b7a4c403124ae2fb52bedc534d82fa8"));

    // re-create cipher mode instance
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let mut buf = ciphertext.to_vec();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();

    println!("Ciphertext : {:X?}",ciphertext);
    //print!("Ciphertext : ");
    //io::stdout().write(ciphertext);

    assert_eq!(decrypted_ciphertext, plaintext.as_bytes());
}
