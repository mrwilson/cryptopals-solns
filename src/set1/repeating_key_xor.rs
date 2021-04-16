use std::borrow::Borrow;

pub fn repeating_key_xor(key: &str, text: &str) -> String {
    return text
        .as_bytes()
        .iter()
        .zip(key.as_bytes().iter().cycle().take(text.len()))
        .map(|pair| (pair.0 ^ pair.1))
        .map(|b| format!("{:02X}", b))
        .collect::<String>()
        .to_lowercase();
}

#[cfg(test)]
mod test {
    use super::repeating_key_xor;

    #[test]
    fn no_padding() {
        assert_eq!(
            repeating_key_xor("ICE","Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}
