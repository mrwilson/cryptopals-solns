use crate::set1::bhattacharya::bhattacharya;
use crate::set1::fixed_xor::fixed_xor;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::iter::Map;

pub fn single_byte_xor(key: char, text: &str) -> String {
    return (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).unwrap())
        .map(|i| (i ^ (key as u8)) as char)
        .collect::<String>();
}

pub fn detect_single_byte_xor(text: &str) -> (String, u8, f32) {
    let etaoin_shrdlu: HashMap<u8, f32> = [
        ('e', 0.124167f32),
        ('t', 0.0969225f32),
        ('a', 0.0820011f32),
        ('i', 0.0768052f32),
        ('n', 0.0764055f32),
        ('o', 0.0714095f32),
        ('s', 0.0706768f32),
        ('r', 0.0668132f32),
        ('l', 0.0448308f32),
        ('d', 0.0363709f32),
        ('h', 0.0350386f32),
        ('c', 0.0344391f32),
        ('u', 0.028777f32),
        ('m', 0.0281775f32),
        ('f', 0.0235145f32),
        ('p', 0.0203171f32),
        ('y', 0.0189182f32),
        ('g', 0.0181188f32),
        ('w', 0.0135225f32),
        ('v', 0.0124567f32),
        ('b', 0.0106581f32),
        ('k', 0.00393019f32),
        ('x', 0.00219824f32),
        ('j', 0.0019984f32),
        ('q', 0.0009325f32),
        ('z', 0.000599),
    ]
    .iter()
    .map(|pair| (pair.0 as u8, pair.1))
    .collect();

    let mut outputs: Vec<(String, u8, f32)> = (0u8..127)
        .into_iter()
        .map(|a| {
            let sbx = single_byte_xor(a as char, text);

            let mut counts: HashMap<u8, f32> = HashMap::new();

            for char in sbx.chars() {
                *counts.entry(char as u8).or_insert(0f32) += (1f32 / (text.len() as f32));
            }

            (sbx, a, bhattacharya(counts, etaoin_shrdlu.clone()))
        })
        .collect();

    outputs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Equal));

    return outputs.last().unwrap().clone();
}
