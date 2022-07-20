pub fn xor<T: AsRef<[u8]>>(bytes1: T, bytes2: T) -> Result<Vec<u8>, &'static str> {
    let bytes1 = bytes1.as_ref();
    let bytes2 = bytes2.as_ref();
    let mut result = Vec::new();

    if bytes1.len() != bytes2.len() {
        return Err("array lengths must match")
    }

    for (b1, b2) in bytes1.iter().zip(bytes2) {
        result.push(b1 ^ b2);
    }
    Ok(result)
}
