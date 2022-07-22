use cryptopals::cryptanalysis::*;
use cryptopals::xor;

#[test]
fn frequency_score_test() {
    // Straight score
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let oracle = 549.91;
    let input = hex::decode(input).expect("hex decode failed");
    let output = CryptanalysisMethod::FrequencyAnalysis.score(input);
    let output = (100.0 * output).round()/100.0;
    assert_eq!(output, oracle);
}

#[test]
fn find_xor_byte_test() {
    // Cryptopals challenge
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let rot_oracle = 88;
    let score_oracle = 9.26;
    let text_oracle = "Cooking MC's like a pound of bacon";
    let input = hex::decode(input).expect("hex decode failed");
    let rot_output = find_xor_byte(CryptanalysisMethod::FrequencyAnalysis, &input);
    assert_eq!(rot_output, rot_oracle);
    let text_output = xor::xor_byte(rot_output, &input);
    let score_output = CryptanalysisMethod::FrequencyAnalysis.score(&text_output);
    let score_output = (100.0 * score_output).round()/100.0;
    let text_output:Vec<char> = text_output.iter().map(|&x| x as char).collect();
    let text_output:String = text_output.iter().collect();
    assert_eq!(score_output, score_oracle);
    assert_eq!(text_output, text_oracle);
}
