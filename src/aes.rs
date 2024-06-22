use std::collections::HashSet;
use crate::xor;
use openssl;

// Lovely example of why Copilot is bad and you shouldn't use it except
// for the most trivial of tasks. This is Copilot's suggestion, which
// is straight up wrong.
/*
pub fn detect_aes<T: AsRef<[u8]>>(bytes: T) -> bool {
    let bytes = bytes.as_ref();
    bytes.len() % 16 == 0 && bytes.windows(16).all(|w| w == &bytes[0..16])
}
*/

pub fn detect_ecb<T: AsRef<[u8]>>(bytes: T) -> bool {
    let blocksize = 16;
    let bytes = bytes.as_ref();
    if bytes.len() % blocksize != 0 {
        return false;
    }
    let chunks = bytes.chunks(blocksize);
    let chunks_set = chunks.clone().collect::<HashSet<_>>();
    // Repeated block suggests AES ECB
    return chunks.len() != chunks_set.len();
}

pub fn decrypt_aes128_cbc<T: AsRef<[u8]>, U: AsRef<[u8]>>(bytes: T, key: U, iv: U) -> Result<Vec<u8>, String> {
    let bytes = bytes.as_ref();
    let key = key.as_ref();
    let iv = iv.as_ref();
    let blocksize = 16;

    // Err Checking
    if bytes.len() % blocksize != 0 {
        return Err(format!("bytes length must be a multiple of 16"));
    }

    // AES setup
    let cipher = openssl::symm::Cipher::aes_128_ecb();
    let mut decrypter = match openssl::symm::Crypter::new(cipher, openssl::symm::Mode::Decrypt, key, None) {
        Ok(decrypter) => decrypter,
        Err(_) => return Err("AES Decrypter creation failed".to_string())
    };
    decrypter.pad(false);

    // Decrypt blocks
    let mut result: Vec<u8> = Vec::new();
    let mut prev_block = iv;
    for block in bytes.chunks(blocksize) {
        let mut intermediate = vec![0; 32];
        match decrypter.update(&block, &mut intermediate) {
            Ok(_) => (),
            Err(_) => return Err("AES Decrypt failed".to_string())
        }
        let plaintext_block = xor::xor(&intermediate[..blocksize], &prev_block)?;
        result.extend(plaintext_block);
        prev_block = block;
    }
    Ok(result)
}