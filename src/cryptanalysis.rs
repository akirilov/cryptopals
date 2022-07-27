use crate::xor;

// Private helper submodules
mod frequency_analysis;

pub struct SingleByteXorResult {
    pub xor_byte: u8,
    pub bytes: Vec<u8>,
}

pub enum CryptanalysisMethod {
    FrequencyAnalysis,
}

impl CryptanalysisMethod {
    pub fn score<T: AsRef<[u8]>>(&self, bytes: T) -> f64 {
        match self {
            CryptanalysisMethod::FrequencyAnalysis => {
                frequency_analysis::score(bytes)
            },
        }
    }
}

pub fn find_single_byte_xor<T: AsRef<[u8]>>(method: CryptanalysisMethod, bytes: T) -> SingleByteXorResult
{
    let mut xor_byte = 0;
    let mut best_score = method.score(&bytes);
    let mut best_result = xor::single_byte_xor(0, &bytes);
    for i in 1..u8::max_value() {
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
