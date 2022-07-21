// Private helper submodules
mod frequency_analysis;

pub enum CryptanalysisMethod {
    FrequencyAnalysis,
}

impl CryptanalysisMethod {
    pub fn score<T: AsRef<[u8]>>(&self, bytes: T) -> f64 {
        match self {
            CryptanalysisMethod::FrequencyAnalysis => {
                0.0
            },
        }
    }
}
