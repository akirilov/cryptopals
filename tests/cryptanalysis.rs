use cryptopals::cryptanalysis::*;
use std::fs;

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
fn find_single_byte_xor_test() {
    // Challenge 1.3
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let xor_byte_oracle = 88;
    let score_oracle = 9.26;
    let text_oracle = "Cooking MC's like a pound of bacon";
    let input = hex::decode(input).expect("hex decode failed");
    let xor_output = find_single_byte_xor(CryptanalysisMethod::FrequencyAnalysis, &input);
    assert_eq!(xor_output.xor_byte, xor_byte_oracle);
    let text_output = xor_output.bytes;
    let score_output = CryptanalysisMethod::FrequencyAnalysis.score(&text_output);
    let score_output = (100.0 * score_output).round()/100.0;
    let text_output:Vec<char> = text_output.iter().map(|&x| x as char).collect();
    let text_output:String = text_output.iter().collect();
    assert_eq!(score_output, score_oracle);
    assert_eq!(text_output, text_oracle);
}

#[test]
fn identify_xor_test() {
    // Challenge 1.4
    let score_oracle = 11.83;
    let hex_oracle = "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f";
    let plain_oracle = "Now that the party is jumping\n";
    let contents = fs::read_to_string("tests/res/4.txt").expect("Something went wrong reading the file");
    let lines = contents.split("\n");
    let mut best_score = 99999999.0;
    let mut best_hex = "";
    let mut best_plain = "".to_string();
    for hex in lines {
        let hex = hex.trim();
        let line = hex::decode(&hex).expect("hex decode failed");
        let plain = find_single_byte_xor(CryptanalysisMethod::FrequencyAnalysis, &line).bytes;
        let plain: Vec<char> = plain.iter().map(|&x| x as char).collect();
        let plain: String = plain.iter().collect();
        let score = CryptanalysisMethod::FrequencyAnalysis.score(&plain);
        if score < best_score {
            best_score = score;
            best_hex = hex;
            best_plain = String::from(plain);
        }
    }
    let best_score = (100.0 * best_score).round()/100.0;
    assert_eq!(best_score, score_oracle);
    assert_eq!(best_hex, hex_oracle);
    assert_eq!(best_plain, plain_oracle);
}

#[test]
fn get_hamming_distance_test() {
    let input1 = "this is a test";
    let input2 = "wokka wokka!!!";
    let oracle = 37;
    let output = get_hamming_distance(input1, input2).expect("Mismatched lengths");
    assert_eq!(output, oracle);
}
