use crate::base64;

#[test]
fn encode_test() {
    // Basic test
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let input = hex::decode(input).expect("hex decode failed");
    let result = base64::encode(&input);
    assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    // One pad
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f";
    let input = hex::decode(input).expect("hex decode failed");
    let result = base64::encode(&input);
    assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb28=");

    // Two pad
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f";
    let input = hex::decode(input).expect("hex decode failed");
    let result = base64::encode(&input);
    assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hybw==");

    // Empty
    let input = "";
    let input = hex::decode(input).expect("hex decode failed");
    let result = base64::encode(&input);
    assert_eq!(result, "");
}
