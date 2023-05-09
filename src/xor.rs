pub fn xor<T: AsRef<[u8]>, U: AsRef<[u8]>>(bytes1: T, bytes2: U) -> Result<Vec<u8>, String> {
    let bytes1 = bytes1.as_ref();
    let bytes2 = bytes2.as_ref();

    if bytes1.len() != bytes2.len() {
        return Err(format!("array lengths must match"));
    }

    let result = bytes1
        .iter()
        .zip(bytes2)
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();

    Ok(result)
}

pub fn single_byte_xor<T: AsRef<[u8]>>(xor_byte: u8, bytes: T) -> Vec<u8> {
    let bytes = bytes.as_ref();
    xor(bytes, &vec![xor_byte; bytes.len()]).unwrap()
}

pub fn repeating_key_xor<T: AsRef<[u8]>, U: AsRef<[u8]>>(xor_key: T, bytes: U) -> Vec<u8> {
    let xor_key = xor_key.as_ref();
    let bytes = bytes.as_ref();

    let xor_array: Vec<u8> = (0..bytes.len())
        .collect::<Vec<usize>>()
        .iter()
        .map(|&x| xor_key[x % xor_key.len()])
        .collect();

    xor(xor_array, bytes).unwrap()
}
