use crate::set1::base64::to_base64;
use crate::set1::hex::hex_value;

pub fn hex_to_base64(input: String) -> String {
    return String::from_utf8(to_base64(hex_value(input))).unwrap();
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
