use crate::set1::fixed_xor::fixed_xor;
use std::iter::Map;

pub fn single_byte_xor(key: char, text: &str) -> String {
    return (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).unwrap())
        .map(|i| (i ^ (key as u8)) as char)
        .collect::<String>();
}
