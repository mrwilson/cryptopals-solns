use std::borrow::Borrow;

pub fn hamming(one: Vec<u8>, two: Vec<u8>) -> u8 {
    return one
        .iter()
        .zip(two.iter())
        .map(|pair| (pair.0 ^ pair.1))
        .map(|byte| u8::count_ones(byte) as u8)
        .sum();
}

#[cfg(test)]
mod test {
    use super::hamming;

    fn to_bytes(input: &str) -> Vec<u8> {
        return String::from(input).into_bytes();
    }

    #[test]
    fn equality_is_zero() {
        assert_eq!(hamming(to_bytes("foo"), to_bytes("foo")), 0);
    }

    #[test]
    fn example() {
        assert_eq!(
            hamming(to_bytes("this is a test"), to_bytes("wokka wokka!!!")),
            37
        );
    }
}
