fn hex_to_byte(byte: u8) -> u8 {
    match byte {
        48..=57 => byte - 48,
        97..=102 => byte - 87,
        _ => 0,
    }
}

fn byte_to_hex(byte: u8) -> u8 {
    match byte {
        0..=9 => byte + 48,
        10..=15 => byte + 87,
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

pub fn as_hex<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    input
        .as_ref()
        .iter()
        .flat_map(|n| {
            vec![byte_to_hex(n / 16), byte_to_hex(n % 16)]
                .clone()
                .to_vec()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::set1::hex::{as_hex, hex_value};
    use std::str::from_utf8;

    #[test]
    fn example_input() {
        assert_eq!(hex_value("090A0B0C"), vec![9, 10, 11, 12]);
    }

    #[test]
    fn example_input_2() {
        let output = as_hex(vec![9, 10, 11, 12]);

        assert_eq!(from_utf8(output.as_slice()).unwrap(), "090a0b0c");
    }

    #[test]
    fn reversibility() {
        let input = "Rub a dub dub, 3 men in a tub";

        let output = hex_value(as_hex(input));

        assert_eq!(from_utf8(output.as_slice()).unwrap(), input);
    }
}
