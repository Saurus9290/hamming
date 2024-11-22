/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    Some(s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count())
}

fn main() {
    // Test cases for the hamming_distance function
    let test_cases = vec![
        ("karolin", "kathrin"),
        ("karolin", "kerstin"),
        ("1011101", "1001001"),
        ("2173896", "2233796"),
        ("short", "longer"), // Mismatched lengths
        ("", ""),           // Empty strings
    ];

    for (s1, s2) in test_cases {
        match hamming_distance(s1, s2) {
            Some(distance) => println!("Hamming distance between \"{}\" and \"{}\" is {}", s1, s2, distance),
            None => println!("Strings \"{}\" and \"{}\" have mismatched lengths.", s1, s2),
        }
    }
}
