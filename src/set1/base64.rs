pub fn to_base64<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    let mut bits: Vec<u8> = input
        .as_ref()
        .iter()
        .map(|byte| {
            (0..8)
                .into_iter()
                .rev()
                .map(move |position| if byte & (1 << position) == 0 { 0 } else { 1 })
        })
        .flatten()
        .collect();

    if bits.len() % 6 != 0 {
        bits.extend(vec![0; 6 - (bits.len() % 6)]);
    }

    let mut output: Vec<u8> = bits
        .chunks(6)
        .map(|chunk| chunk.into_iter().fold(0, |acc, i| (acc << 1) + i))
        .map(|sextet| {
            match sextet {
                0..=25 => sextet + 65,  // A - Z
                26..=51 => sextet + 71, // a - z
                52..=61 => sextet - 4,  // 0 - 9
                62 => '+' as u8,
                63 => '/' as u8,
                64 => '=' as u8,
                _ => 0,
            }
        })
        .collect();

    if output.len() % 4 != 0 {
        output.extend(vec!['=' as u8; 4 - output.len() % 4]);
    }

    output
}

pub fn from_base64<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    let mut bytes: Vec<u8> = input.as_ref().to_vec();

    while *bytes.last().unwrap() == ('=' as u8) {
        bytes.pop();
    }

    let bits = bytes
        .clone()
        .iter()
        .map(|byte| {
            match byte {
                65..=90 => byte - 65,  // A - Z
                97..=122 => byte - 71, // a - z
                48..=57 => byte + 4,   // 0 - 9
                43 => 62,              // +
                47 => 63,              // /
                61 => 64,              // =
                _ => 0,
            }
        })
        .map(|byte| {
            (0..6)
                .into_iter()
                .rev()
                .map(move |position| if byte & (1 << position) == 0 { 0 } else { 1 })
        })
        .flatten()
        .collect::<Vec<u8>>();

    let size = bits.len() - (bits.len() % 8);

    bits.into_iter()
        .take(size)
        .collect::<Vec<u8>>()
        .chunks(8)
        .map(|chunk| chunk.into_iter().fold(0, |acc, i| (acc << 1) + i))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::set1::base64::{from_base64, to_base64};
    use std::str::from_utf8;

    #[test]
    fn example_input() {
        let decoded = to_base64("Mary had a little lamb");
        let expected = from_utf8(decoded.as_slice()).unwrap();
        assert_eq!(expected, "TWFyeSBoYWQgYSBsaXR0bGUgbGFtYg==");
    }

    #[test]
    fn example_input_2() {
        let decoded = from_base64("TWFyeSBoYWQgYSBsaXR0bGUgbGFtYg==");
        let expected = from_utf8(decoded.as_slice()).unwrap();
        assert_eq!(expected, "Mary had a little lamb");
    }
}
