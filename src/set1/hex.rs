fn hex_to_byte(byte: u8) -> u8 {
    match byte {
        48..=57 => byte - 48,
        97..=102 => byte - 87,
        _ => 0,
    }
}

pub fn hex_value<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    input
        .as_ref()
        .to_ascii_lowercase()
        .chunks(2)
        .map(|chunk| 16u8 * hex_to_byte(chunk[0]) + hex_to_byte(chunk[1]))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::set1::hex::hex_value;

    #[test]
    fn example_input() {
        assert_eq!(hex_value("090A0B0C"), vec![9, 10, 11, 12]);
    }
}
