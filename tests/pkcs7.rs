use cryptopals::pkcs7;

#[test]
fn pad_test() {
    // Challenge 9
    let input = "YELLOW SUBMARINE";
    let blocksize = 20;
    let oracle = "YELLOW SUBMARINE\x04\x04\x04\x04";
    let output = pkcs7::pad(input, blocksize);
    let output = std::str::from_utf8(&output).unwrap();
    assert_eq!(output, oracle);
}