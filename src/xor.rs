pub fn xor<T: AsRef<[u8]>>(bytes1: T, bytes2: T) -> Result<Vec<u8>, &'static str> {
    let bytes1 = bytes1.as_ref();
    let bytes2 = bytes2.as_ref();

    if bytes1.len() != bytes2.len() {
        return Err("array lengths must match")
    }

    let result = bytes1
        .iter()
        .zip(bytes2)
        .map(|(&b1, &b2)| b1 ^ b2)
        .collect();

    Ok(result)
}
