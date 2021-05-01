use std::cmp::Ordering::Equal;

use crate::set1::hamming::hamming;
use crate::set1::single_byte_xor::detect_single_byte_xor;

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

pub fn detect_repeating_key_xor(input: Vec<u8>) -> String {
    let mut predicted_key_sizes: Vec<(usize, f32)> = (2..40)
        .into_iter()
        .map(|size| {
            let chunks: Vec<&[u8]> = input.chunks(size).take(4).collect();
            let mut distance = 0f32;
            for i in 0..4 {
                for j in i..4 {
                    distance += hamming(chunks[i].to_vec(), chunks[j].to_vec()) as f32;
                }
            }
            (size, distance / size as f32)
        })
        .collect();

    predicted_key_sizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Equal));

    let transposed = transpose(input.clone(), predicted_key_sizes.first().unwrap().0);

    transposed
        .into_iter()
        .map(|block| detect_single_byte_xor(block) as char)
        .collect::<String>()
}

pub fn transpose(input: Vec<u8>, block_size: usize) -> Vec<Vec<u8>> {
    let blocks: Vec<Vec<u8>> = input
        .chunks(block_size)
        .map(|block| block.to_vec())
        .collect();

    (0..block_size)
        .into_iter()
        .map(|index| {
            blocks
                .clone()
                .iter()
                .filter(|block| block.len() > index)
                .map(|block| block.get(index).copied().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::repeating_key_xor;
    use crate::set1::repeating_key_xor::detect_repeating_key_xor;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn no_padding() {
        assert_eq!(
            repeating_key_xor("ICE","Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }

    #[test]
    fn break_repeating_key_xor() {
        let mut base64_encoded = String::new();
        let mut file = File::open("inputs/1_6.txt").unwrap();
        file.read_to_string(&mut base64_encoded).unwrap();
        base64_encoded = str::replace(&base64_encoded, "\n", "");

        assert_eq!(
            String::from("Terminator X: Bring the noise"),
            detect_repeating_key_xor(base64::decode(&base64_encoded).unwrap())
        )
    }
}
