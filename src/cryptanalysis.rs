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
    let mut best_score = u32::max_value();
    let mut best_keysize = 0;

    // For now we'll just do a rolling comparison of N=4? blocks and average, but in the future we can
    // improve this
    for i in 1..(max_keysize + 1) {
        // TODO
        // Transpose
        // Take hamming distances between N blocks and average
        // The lowest candidate is probably the keysize
        }

    // Step 2: Break into blocks (every nth character) and solve each block as single byte xor
    // TODO

    // Step 3: Recover the key and decipher the entire message
    // TODO

    RepeatingKeyXorResult {
        key: Vec::<u8>::new(),
        bytes: Vec::<u8>::new(),
    }
}
