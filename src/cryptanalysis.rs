use crate::xor;

// Private helper submodules
mod frequency_analysis;

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

pub fn find_xor_byte<T: AsRef<[u8]>>(method: CryptanalysisMethod, bytes: T) -> u8
{
    let mut rot = 0;
    let mut best_score = method.score(&bytes);
    for i in 1..u8::max_value() {
        let score = method.score(xor::xor_byte(i, &bytes));
        if score < best_score {
            rot = i;
            best_score = score;
        }
    }
    rot
}
