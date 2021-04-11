pub fn fixed_xor(one: String, two: String) -> String {
    if one == two {
        return String::from_utf8(vec![b'0'; one.len()]).unwrap();
    }

    return String::from("")
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

}