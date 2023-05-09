const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PADDING: char = '=';

#[derive(Debug)]
struct Leftovers {
    n_bits: u32,
    data: u32,
}

pub fn encode<T: AsRef<[u8]>>(bytes: T) -> String {
    let bytes = bytes.as_ref();
    let mut result = String::new();
    let mut leftovers = Leftovers { n_bits: 0, data: 0 };

    // Generate the result as we go
    for &b in bytes {
        // append the current byte
        leftovers.n_bits += 8;
        leftovers.data <<= 8;
        leftovers.data |= b as u32;

        // generate as many base64 characters as possible
        while leftovers.n_bits >= 6 {
            // Temporary shift right to read only the top 6 bits
            let shift = leftovers.n_bits - 6;
            let b64_char = leftovers.data >> shift;
            let b64_char = ALPHABET.chars().nth(b64_char as usize).unwrap();

            // Store the result
            result.push(b64_char);

            // Use a mask to remove the top 6 bits
            leftovers.n_bits -= 6;
            let mask = (1 << leftovers.n_bits) - 1;
            leftovers.data &= mask;
        }
    }

    // Deal with remaining leftovers and padding
    // We could do this with funny math but since we only have 3 cases it's cleaner to use
    // pattern matching
    match leftovers.n_bits {
        2 => {
            let b64_char = alphabet_nth(leftovers.data << 4);
            result.push(b64_char);
            result.push(PADDING);
            result.push(PADDING);
        }
        4 => {
            let b64_char = alphabet_nth(leftovers.data << 2);
            result.push(b64_char);
            result.push(PADDING);
        }
        _ => {}
    }

    result
}

pub fn decode<T: AsRef<[u8]>>(encoded: T) -> Result<Vec<u8>, String> {
    let encoded = encoded.as_ref();
    let mut result: Vec<u8> = Vec::new();
    let mut pad_size = 0;
    let mut leftovers = Leftovers { n_bits: 0, data: 0 };

    // Check length
    if encoded.len() % 4 != 0 {
        return Err(format!("Invalid length"));
    }

    // Loop over the encoded bytes, popping off 6 bits at a time and adding a new byte to the result
    // once we've reassembled 8 bits (same as encoding)
    for &b in encoded {
        let b = b as char;

        // append the current byte
        if b != PADDING {
            if pad_size != 0 {
                return Err(format!("Characters after padding"));
            }
            leftovers.n_bits += 6;
            leftovers.data <<= 6;
            let pos = ALPHABET.chars().position(|c| c == b);
            match pos {
                Some(x) => leftovers.data |= x as u32,
                None => return Err(format!("Invalid character: [{b}]")),
            }
        } else {
            pad_size += 1;
        }

        // build as many bytes as we can
        while leftovers.n_bits >= 8 {
            // Temporary shift right to read only the top 6 bits
            let shift = leftovers.n_bits - 8;
            let out_char = leftovers.data >> shift;

            // Store the result
            result.push(out_char as u8);

            // Use a mask to remove the top 6 bits
            leftovers.n_bits -= 8;
            let mask = (1 << leftovers.n_bits) - 1;
            leftovers.data &= mask;
        }
    }

    // Check padding
    if leftovers.data != 0 {
        return Err(format!("Nonzero padding"));
    }

    Ok(result)
}

fn alphabet_nth(n: u32) -> char {
    ALPHABET.chars().nth(n as usize).unwrap()
}
