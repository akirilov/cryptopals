use cryptopals::{aes, base64, pkcs7};
use openssl;
use std::fs;

#[test]
fn ecb_decode_test() {
    // Challenge 7
    let mut ciphertext = fs::read_to_string("tests/res/7.txt").expect("Something went wrong reading the file");
    ciphertext.retain(|x| x != '\n' && x != '\r');
    let oracle_plaintext = fs::read_to_string("tests/res/7_oracle.txt").expect("Something went wrong reading the file");
    let key = b"YELLOW SUBMARINE";

    let cipher_bytes = base64::decode(ciphertext).expect("Base64 decode failed");
    let cipher = openssl::symm::Cipher::aes_128_ecb();
    let result = openssl::symm::decrypt(cipher, key, None, &cipher_bytes).unwrap();
    let result = String::from_utf8(result).expect("string conversion failed");
    assert_eq!(result, oracle_plaintext);
}

#[test]
fn detect_aes_test() {
    // Challenge 8
    let ciphertext_raw = fs::read_to_string("tests/res/8.txt").expect("Something went wrong reading the file");
    let ciphertext_array: Vec<&str> = ciphertext_raw.split("\n").collect();
    let mut aes_line = None;
    for (i, ciphertext) in ciphertext_array.iter().enumerate() {
        let bytes = hex::decode(ciphertext).expect("Hex decode failed");
        let result = aes::detect_ecb(&bytes);
        if result {
            aes_line = Some(i);
        }
    }
    assert_eq!(aes_line.unwrap(), 132);
}

#[test]
fn decrypt_aes128_cbc_test() {
    // Challenge 10
    let mut ciphertext = fs::read_to_string("tests/res/10.txt").expect("Something went wrong reading the file");
    ciphertext.retain(|x| x != '\n' && x != '\r');
    let oracle = fs::read_to_string("tests/res/10_oracle.txt").expect("Something went wrong reading the file");
    let oracle = String::from_utf8(pkcs7::pad(oracle.as_bytes(), 16)).expect("string conversion failed");

    let cipher_bytes = base64::decode(ciphertext).expect("Base64 decode failed");
    let key = b"YELLOW SUBMARINE";
    let iv = [0; 16];
    let result = aes::decrypt_aes128_cbc(&cipher_bytes, key, &iv).unwrap();
    let result = String::from_utf8(result).expect("string conversion failed");
    assert_eq!(result, oracle);
}