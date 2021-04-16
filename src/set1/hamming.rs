use std::borrow::Borrow;

pub fn hamming(one: &str, two: &str) -> u8 {
    return one
        .as_bytes()
        .iter()
        .zip(two.as_bytes().iter())
        .map(|pair| (pair.0 ^ pair.1))
        .map(|byte| u8::count_ones(byte) as u8)
        .sum();
}

#[cfg(test)]
mod test {
    use super::hamming;

    #[test]
    fn equality_is_zero() {
        assert_eq!(hamming("foo", "foo"), 0);
    }

    #[test]
    fn example() {
        assert_eq!(hamming("this is a test", "wokka wokka!!!"), 37);
    }
}
