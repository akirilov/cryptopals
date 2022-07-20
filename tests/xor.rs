use cryptopals::xor;

#[test]
fn xor_test() {
    // Basic test
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let oracle = "746865206b696420646f6e277420706c6179";
    let input1 = hex::decode(input1).expect("hex decode failed");
    let input2 = hex::decode(input2).expect("hex decode failed");
    let oracle = hex::decode(oracle).expect("hex decode failed");
    let output = xor::xor(input1, input2).expect("xor failed");
    assert_eq!(output, oracle);

    //String test
    let input1 = "test";
    let input2 = "test";
    let oracle = "00000000";
    let oracle = hex::decode(oracle).expect("hex decode failed");
    let output = xor::xor(input1, input2).expect("xor failed");
    assert_eq!(output, oracle);

    // Length mismatch
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c2773206579";
    let oracle = Err("array lengths must match");
    let input1 = hex::decode(input1).expect("hex decode failed");
    let input2 = hex::decode(input2).expect("hex decode failed");
    let output = xor::xor(input1, input2);
    assert_eq!(output, oracle);
}
