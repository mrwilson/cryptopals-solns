pub fn fixed_xor(one: String, two: String) -> String {
    let alphabet: Vec<char> = "0123456789abcdef".chars().collect();

    if one == two {
        return String::from_utf8(vec![b'0'; one.len()]).unwrap();
    }

    return one
        .chars()
        .into_iter()
        .zip(two.chars().into_iter())
        .map(|pair| {
            alphabet
                .get(hex_value(&alphabet, pair.0) ^ hex_value(&alphabet, pair.1))
                .unwrap()
        })
        .collect::<String>();
}

fn hex_value(alphabet: &Vec<char>, value: char) -> usize {
    alphabet
        .clone()
        .into_iter()
        .position(|x| x == value)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::fixed_xor;

    #[test]
    fn identical_input_outputs_zero_string() {
        let input = String::from("aaa");
        let output = String::from("aaa");

        assert_eq!(fixed_xor(input, output), "000");
    }

    #[test]
    fn example_input() {
        let input = String::from("457");
        let output = String::from("def");

        assert_eq!(fixed_xor(input, output), "9b8");
    }
}
