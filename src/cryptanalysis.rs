use crate::xor;
use std::cmp;

// Private helper submodules
mod frequency_analysis;

pub struct SingleByteXorResult {
    pub xor_byte: u8,
    pub bytes: Vec<u8>,
}

pub struct RepeatingKeyXorResult {
    pub key: Vec<u8>,
    pub bytes: Vec<u8>,
}

pub enum CryptanalysisMethod {
    FrequencyAnalysis,
}

impl CryptanalysisMethod {
    pub fn score<T: AsRef<[u8]>>(&self, bytes: T) -> f64 {
        match self {
            CryptanalysisMethod::FrequencyAnalysis => frequency_analysis::score(bytes),
        }
    }
}

pub fn break_single_byte_xor<T: AsRef<[u8]>>(
    method: CryptanalysisMethod,
    bytes: T,
) -> SingleByteXorResult {
    let mut xor_byte = 0;
    let mut best_score = f64::INFINITY;
    let mut best_result = Vec::new();
    for i in 0..u8::max_value() {
        let result = xor::single_byte_xor(i, &bytes);
        let score = method.score(&result);
        if score < best_score {
            xor_byte = i;
            best_score = score;
            best_result = result;
        }
    }
    SingleByteXorResult {
        xor_byte: xor_byte,
        bytes: best_result,
    }
}

pub fn get_hamming_distance<T: AsRef<[u8]>, U: AsRef<[u8]>>(
    bytes1: T,
    bytes2: U,
) -> Result<u32, &'static str> {
    let bytes1 = bytes1.as_ref();
    let bytes2 = bytes2.as_ref();

    if bytes1.len() != bytes2.len() {
        return Err("array lengths must match");
    }

    // Xor arrays to get positions of differing bits, then count bits in each byte and sum
    let result = bytes1
        .iter()
        .zip(bytes2)
        .map(|(a, b)| a ^ b) // rust naive combines this with the fold
        .fold(0, |s, x| s + x.count_ones());

    Ok(result)
}

pub fn break_repeating_key_xor<T: AsRef<[u8]>>(
    max_keysize: usize,
    bytes: T,
) -> RepeatingKeyXorResult {
    let bytes = bytes.as_ref();

    // Reduce max keysize to avoid overflows later
    let max_keysize = cmp::min(max_keysize, bytes.len());

    // Step 1: Find the keysize
    let mut best_score = f64::INFINITY;
    let mut best_keysize = 0;

    // For now we'll just do a rolling comparison of all blocks and average, but in the future we can
    // improve this
    for i in 1..(max_keysize + 1) {
        let keysize = i;
        let mut score = 0.0;
        let mut count = 0;

        // Take the hamming distance between all neighboring blocks
        // Divide the hamming distance by the keysize to normalize
        for j in 0..(bytes.len() / keysize - 1) {
            let block1 = &bytes[(j * keysize)..((j + 1) * keysize)];
            let block2 = &bytes[((j + 1) * keysize)..((j + 2) * keysize)];
            let distance = get_hamming_distance(block1, block2).unwrap() as f64;
            score += distance / keysize as f64;
            count += 1;
        }

        // Average the score
        score /= count as f64;

        // The lowest candidate is probably the keysize
        if score < best_score {
            best_score = score;
            best_keysize = keysize;
        }
    }

    // Transpose into blocks (every nth character)
    let transposed_blocks = transpose_blocks(bytes, best_keysize).unwrap();
    let mut key = Vec::<u8>::new();

    // Solve each block as single byte xor
    for block in transposed_blocks {
        let block_key = break_single_byte_xor(CryptanalysisMethod::FrequencyAnalysis, block).xor_byte;
        key.push(block_key);
    }

    // Recover the key and decipher the entire message
    let plaintext = xor::repeating_key_xor(&key, &bytes);

    RepeatingKeyXorResult {
        key: key,
        bytes: plaintext,
    }
}

fn transpose_blocks<T: AsRef<[u8]>>(bytes: T, keysize: usize) -> Result<Vec<Vec<u8>>, String> {
    if keysize == 0 {
        return Err(format!("keysize must be greater than 0"));
    }

    let bytes = bytes.as_ref();
    let mut transposed_blocks = vec![Vec::new(); keysize];

    for (i, byte) in bytes.iter().enumerate() {
        transposed_blocks[i % keysize].push(*byte);
    }

    Ok(transposed_blocks)
}
