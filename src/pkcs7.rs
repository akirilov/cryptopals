pub fn pad_bytes<T: AsRef<[u8]>>(bytes: T, blocksize: usize) -> Vec<u8> {
    let bytes = bytes.as_ref();
    let mut result: Vec<u8> = bytes.to_vec();
    let padding_len = blocksize - (bytes.len() % blocksize);
    result.append(&mut vec![4; padding_len]);
    result
}