use cryptopals::base64;
use openssl;
use std::fs;

#[test]
fn ecb_decode_test() {
    // Challenge 1.7
    let mut ciphertext = fs::read_to_string("tests/res/1.7.txt").expect("Something went wrong reading the file");
    ciphertext.retain(|x| x != '\n' && x != '\r');
    let oracle_plaintext = fs::read_to_string("tests/res/1.7_oracle.txt").expect("Something went wrong reading the file");
    let key = b"YELLOW SUBMARINE";

    let cipher_bytes = base64::decode(ciphertext).expect("Base64 decode failed");
    let cipher = openssl::symm::Cipher::aes_128_ecb();
    let result = openssl::symm::decrypt(cipher, key, None, &cipher_bytes).unwrap();
    let result = String::from_utf8(result).expect("string conversion failed");
    assert_eq!(result, oracle_plaintext);
}
