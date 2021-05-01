use crate::set1::hex::hex_value;

pub fn fixed_xor<T: AsRef<[u8]>>(one: T, two: T) -> Vec<u8> {
    return one
        .as_ref()
        .iter()
        .zip(two.as_ref().iter())
        .map(|pair| (pair.0 ^ pair.1))
        .collect();
}

#[cfg(test)]
mod test {
    use super::fixed_xor;
    use crate::set1::hex::hex_value;

    #[test]
    fn identical_input_outputs_zero_vector() {
        let input = hex_value("1c0111001f010100061a024b53535009181c");

        assert_eq!(fixed_xor(&input, &input), vec![0; input.len()]);
    }

    #[test]
    fn example_input() {
        assert_eq!(
            fixed_xor(
                hex_value("1c0111001f010100061a024b53535009181c"),
                hex_value("686974207468652062756c6c277320657965")
            ),
            hex_value("746865206b696420646f6e277420706c6179")
        );
    }
}
