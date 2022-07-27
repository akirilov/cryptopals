use cryptopals::base64;

#[test]
fn encode_test() {
    // Challenge 1.1
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
