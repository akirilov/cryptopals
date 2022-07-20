pub mod base64 {
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
        let mut leftovers = Leftovers {
            n_bits: 0,
            data: 0,
        };

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
            },
            4 => {
                let b64_char = alphabet_nth(leftovers.data << 2);
                result.push(b64_char);
                result.push(PADDING);
            },
            _ => {},
        }

        result
    }

    fn alphabet_nth(n: u32) -> char {
        ALPHABET.chars().nth(n as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::base64;

    #[test]
    fn base64_encode() {
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
}
