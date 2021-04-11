fn bit_for_position(byte: &u8, position: &u8) -> u8 {
    if byte & (1 << position) == 0 {
        0
    } else {
        1
    }
}

pub fn hex_to_base64(input: String) -> String {
    let hex = "0123456789abcdef".chars();

    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="
        .chars()
        .collect();

    let mut input_as_bits = input
        .chars()
        .into_iter()
        .map(|c| hex.clone().into_iter().position(|r| r == c).unwrap() as u8)
        .map(|b| {
            (0..4)
                .into_iter()
                .rev()
                .map(move |c| bit_for_position(&b, &c))
        })
        .flatten()
        .collect::<Vec<u8>>();

    if input_as_bits.len() % 6 != 0 {
        input_as_bits.extend(vec![0; 6 - (input_as_bits.len() % 6)]);
    }

    let mut output = input_as_bits
        .chunks(6)
        .map(|chunk| chunk.into_iter().fold(0, |acc, i| (acc << 1) + i) as usize)
        .map(|sextet| alphabet.get(sextet).unwrap())
        .collect::<String>();

    if output.len() % 4 != 0 {
        output.extend(vec!["="; 4 - output.len() % 4]);
    }

    return output;
}

#[cfg(test)]
mod test {
    use super::hex_to_base64;

    #[test]
    fn no_padding() {
        let input = String::from("aaaaaa");
        let output = String::from("qqqq");

        assert_eq!(hex_to_base64(input), output);
    }

    #[test]
    fn padding_one() {
        let input = String::from("aa");
        let output = String::from("qg==");

        assert_eq!(hex_to_base64(input), output);
    }

    #[test]
    fn padding_two() {
        let input = String::from("aaaa");
        let output = String::from("qqo=");

        assert_eq!(hex_to_base64(input), output);
    }
}
