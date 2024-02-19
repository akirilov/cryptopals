use cryptopals::base64;

#[test]
fn encode_test() {
    // Challenge 1
    // Basic test
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let oracle = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let input = hex::decode(input).expect("hex decode failed");
    let result = base64::encode(&input);
    assert_eq!(result, oracle);

    // One pad
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f";
    let oracle = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb28=";
    let input = hex::decode(input).expect("hex decode failed");
    let result = base64::encode(&input);
    assert_eq!(result, oracle);

    // Two pad
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f";
    let oracle = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hybw==";
    let input = hex::decode(input).expect("hex decode failed");
    let result = base64::encode(&input);
    assert_eq!(result, oracle);

    // String
    let input = "test";
    let oracle = "dGVzdA==";
    let result = base64::encode(&input);
    assert_eq!(result, oracle);

    // Empty
    let input = "";
    let oracle = "";
    let input = hex::decode(input).expect("hex decode failed");
    let result = base64::encode(&input);
    assert_eq!(result, oracle);
}

#[test]
fn decode_test() {
    // No Pad
    let input = "bm9wYWQh";
    let oracle = "nopad!".as_bytes().to_vec();
    let result = base64::decode(&input).expect("base64 decode failed");
    assert_eq!(result, oracle);
    // One pad
    let input = "dW4=";
    let oracle = "un".as_bytes().to_vec();
    let result = base64::decode(&input).expect("base64 decode failed");
    assert_eq!(result, oracle);
    // Two pad
    let input = "dGVzdA==";
    let oracle = "test".as_bytes().to_vec();
    let result = base64::decode(&input).expect("base64 decode failed");
    assert_eq!(result, oracle);
    // Empty
    let input = "";
    let oracle = "".as_bytes().to_vec();
    let result = base64::decode(&input).expect("base64 decode failed");
    assert_eq!(result, oracle);
    // Length Error
    let input = "dGVzdA=";
    let oracle = Err(format!("Invalid length"));
    let result = base64::decode(&input);
    assert_eq!(result, oracle);
    // Nonzero padding
    let input = "dGVzdB==";
    let oracle = Err(format!("Nonzero padding"));
    let result = base64::decode(&input);
    assert_eq!(result, oracle);
    // Unexpected character
    let input = "dGVzd)==";
    let oracle = Err(format!("Invalid character: [)]"));
    let result = base64::decode(&input);
    assert_eq!(result, oracle);
    // Pad in middle of string
    // Nonzero padding
    let input = "dGV=zdB=";
    let oracle = Err(format!("Characters after padding"));
    let result = base64::decode(&input);
    assert_eq!(result, oracle);
}
