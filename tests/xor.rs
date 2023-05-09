use cryptopals::xor;

#[test]
fn xor_test() {
    // Challenge 1.2
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
    let oracle = Err(format!("array lengths must match"));
    let input1 = hex::decode(input1).expect("hex decode failed");
    let input2 = hex::decode(input2).expect("hex decode failed");
    let output = xor::xor(input1, input2);
    assert_eq!(output, oracle);
}

#[test]
fn single_byte_xor_test() {
    // Null test
    let input = "Hello, world!";
    let oracle: Vec<u8> = input.bytes().collect();
    let output = xor::single_byte_xor(0, input);
    assert_eq!(output, oracle);

    // Basic test
    let input = "Hello, world!";
    let oracle = [126, 83, 90, 90, 89, 26, 22, 65, 89, 68, 90, 82, 23];
    let output = xor::single_byte_xor(0x36, input);
    assert_eq!(output, oracle);
}

#[test]
fn repeating_key_xor_test() {
    // Simple test
    let input = "Hello, world!";
    let rot_key = [0x36, 0];
    let oracle = [126, 101, 90, 108, 89, 44, 22, 119, 89, 114, 90, 100, 23];
    let output = xor::repeating_key_xor(rot_key, input);
    assert_eq!(output, oracle);

    // Challenge 1.5
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let rot_key = "ICE";
    let oracle = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let oracle = hex::decode(oracle).expect("hex decode failed");
    let output = xor::repeating_key_xor(rot_key, input);
    assert_eq!(output, oracle);
}
