use phf::phf_map;
use std::collections::HashMap;

static FREQUENCY_TABLE: phf::Map<char, f64> = phf_map! {
    'e' => 0.127,
    't' => 0.091,
    'a' => 0.082,
    'o' => 0.075,
    'i' => 0.070,
    'n' => 0.067,
    's' => 0.063,
    'h' => 0.061,
    'r' => 0.060,
    'd' => 0.043,
    'l' => 0.040,
    'c' => 0.028,
    'u' => 0.028,
    'm' => 0.024,
    'w' => 0.024,
    'f' => 0.022,
    'g' => 0.020,
    'y' => 0.020,
    'p' => 0.019,
    'b' => 0.015,
    'v' => 0.010,
    'k' => 0.008,
    'j' => 0.002,
    'x' => 0.002,
    'q' => 0.001,
    'z' => 0.001,
};

fn bytes_to_counts<T: AsRef<[u8]>>(bytes: T) -> HashMap<char, u64> {
    bytes
        .as_ref()
        .iter()
        .map(|b| *b as char)
        .map(|b| b.to_ascii_lowercase())
        .filter(|b| b.is_ascii_lowercase())
        .fold(HashMap::new(), |mut map, b| {
            *map.entry(b).or_insert(0) += 1;
            map
        })
}

#[test]
fn test_bytes_to_counts() {
    let input = "Hello world!";
    let oracle = HashMap::from([
        ('h', 1),
        ('e', 1),
        ('l', 3),
        ('o', 2),
        ('w', 1),
        ('r', 1),
        ('d', 1),
    ]);
    let output = bytes_to_counts(input);
    assert_eq!(output, oracle);
}
