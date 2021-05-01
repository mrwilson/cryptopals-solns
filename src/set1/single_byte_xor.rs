use crate::set1::bhattacharya::bhattacharya;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;

pub fn single_byte_xor<T: AsRef<[u8]>>(key: u8, text: T) -> Vec<u8> {
    return text.as_ref().into_iter().map(|i| i ^ key).collect();
}

pub fn detect_single_byte_xor(text: Vec<u8>) -> u8 {
    let letter_distribution: HashMap<u8, f32> = [
        ('e', 0.1241),
        ('t', 0.0969),
        ('a', 0.0820),
        ('i', 0.0768),
        ('n', 0.0764),
        ('o', 0.0714),
        ('s', 0.0706),
        ('r', 0.0668),
        ('l', 0.0448),
        ('d', 0.0363),
        ('h', 0.0350),
        ('c', 0.0344),
        ('u', 0.0287),
        ('m', 0.0281),
        ('f', 0.0235),
        ('p', 0.0203),
        ('y', 0.0189),
        ('g', 0.0181),
        ('w', 0.0135),
        ('v', 0.0124),
        ('b', 0.0106),
        ('k', 0.0039),
        ('x', 0.0021),
        ('j', 0.0019),
        ('q', 0.0009),
        ('z', 0.0005),
    ]
    .iter()
    .map(|pair| (pair.0 as u8, pair.1))
    .collect();

    let mut outputs: Vec<(u8, f32)> = (0u8..255)
        .into_iter()
        .map(|char| (char, single_byte_xor(char, &text)))
        .map(|(char, new_text)| {
            let mut counts: HashMap<u8, f32> = HashMap::new();

            for char in &new_text {
                *counts.entry(*char).or_insert(0f32) += 1f32;
            }

            (char, bhattacharya(counts, letter_distribution.clone()))
        })
        .collect();

    outputs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Equal));

    return outputs.last().unwrap().clone().0;
}

mod test {
    use crate::set1::single_byte_xor::{detect_single_byte_xor, single_byte_xor};

    #[test]
    fn example_input() {
        let key = 'A' as u8;

        let input = single_byte_xor(key, "Once upon a time");

        assert_eq!(detect_single_byte_xor(input), key);
    }
}
