use phf::phf_map;
use std::collections::HashMap;

static FREQUENCY_TABLE: phf::Map<char, f64> = phf_map! {
    ' ' => 0.200,// TODO: update this hack with actual counts
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

pub(super) fn score<T: AsRef<[u8]>>(bytes: T) -> f64 {
    chi_square_test(&count_to_frequency(&bytes_to_counts(bytes)))
}

fn bytes_to_counts<T: AsRef<[u8]>>(bytes: T) -> HashMap<char, u64> {
    bytes
        .as_ref()
        .iter()
        .map(|&b| b as char)
        .map(|b| b.to_ascii_lowercase())
        .fold(HashMap::new(), |mut map, b| {
            *map.entry(b).or_insert(0) += 1;
            map
        })
}

fn count_to_frequency(counts: &HashMap<char, u64>) -> HashMap<char, f64> {
    let total = counts.values().fold(0, |sum, &c| sum + c);
    counts
        .iter()
        .map(|(&k, &v)| (k, (v as f64) / (total as f64)))
        .collect()
}

fn chi_square_test(frequencies: &HashMap<char, f64>) -> f64 {
    let mut xs = 0.0;
    for (&letter, &observed) in frequencies {
        let expected = FREQUENCY_TABLE.get(&letter).unwrap_or(&0.0001);
        xs += (expected - observed).powf(2.0) / expected;
    }

    xs
}

#[test]
fn test_bytes_to_counts() {
    let input = "Helloworld";
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

#[test]
fn test_count_to_frequency() {
    let input = "Helloworld";
    let oracle = HashMap::from([
        ('h', 0.1),
        ('e', 0.1),
        ('l', 0.3),
        ('o', 0.2),
        ('w', 0.1),
        ('r', 0.1),
        ('d', 0.1),
    ]);
    let output = bytes_to_counts(input);
    let output = count_to_frequency(&output);
    assert_eq!(output, oracle);
}

#[test]
fn test_chi_square_test() {
    // Normal English
    let input = "Hello world! This is a longer string so we can see the chi square test in action.";
    let oracle = 3.2;
    let output = bytes_to_counts(input);
    let output = count_to_frequency(&output);
    let output = chi_square_test(&output);
    let output = (100.0 * output).round() / 100.0;
    assert_eq!(output, oracle);

    // ROT 13
    let input = "Uryyb jbeyq! Guvf vf n ybatre fgevat fb jr pna frr gur puv fdhner grfg va npgvba.";
    let oracle = 4.54;
    let output = bytes_to_counts(input);
    let output = count_to_frequency(&output);
    let output = chi_square_test(&output);
    let output = (100.0 * output).round() / 100.0;
    assert_eq!(output, oracle);
}

#[test]
fn test_score() {
    let input = "Hello world! This is a longer string so we can see the chi square test in action.";
    let oracle = 3.2;
    let output = score(input);
    let output = (100.0 * output).round() / 100.0;
    assert_eq!(output, oracle);

    // ROT 13
    let input = "Uryyb jbeyq! Guvf vf n ybatre fgevat fb jr pna frr gur puv fdhner grfg va npgvba.";
    let oracle = 4.54;
    let output = score(input);
    let output = (100.0 * output).round() / 100.0;
    assert_eq!(output, oracle);

    // Empty
    let input = "";
    let oracle = 0.0;
    let output = score(input);
    let output = (100.0 * output).round() / 100.0;
    assert_eq!(output, oracle);
}
