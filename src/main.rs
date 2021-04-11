fn main() {
    println!("Hello, world!");
}

fn bit_for_position(byte: &u8, position: &u8) -> u8 {
    if byte & (1 << position) == 0 { 0 } else { 1 }
}

fn hex_to_base64(input: String) -> String {

    let alphabet: Vec<char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".chars().collect();

    return input.bytes().into_iter()
        .map(|b| (0..8).into_iter().rev().map(move |c| bit_for_position(&b, &c)) )
        .flatten()
        .collect::<Vec<u8>>()
        .chunks(6)
        .map(|chunk| chunk.into_iter().fold(0, |acc, i| (acc << 1) + i) as usize)
        .map(|sextet| alphabet.get(sextet).unwrap())
        .collect::<String>();
}

#[cfg(test)]
mod test {
    use super::hex_to_base64;

    #[test]
    fn no_padding() {
        let input = String::from("Man");
        let output = String::from("TWFu");

        assert_eq!(hex_to_base64(input), output);
    }
}


