use std::collections::HashSet;

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