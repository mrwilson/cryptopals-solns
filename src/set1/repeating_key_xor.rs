use std::cmp::Ordering::Equal;

use crate::set1::hamming::hamming;
use crate::set1::single_byte_xor::detect_single_byte_xor;
use std::ops::Range;

pub fn repeating_key_xor<T: AsRef<[u8]>>(key: T, text: T) -> Vec<u8> {
    return text
        .as_ref()
        .iter()
        .zip(key.as_ref().iter().cycle())
        .map(|pair| (pair.0 ^ pair.1))
        .collect();
}

pub fn detect_repeating_key_xor<T: AsRef<[u8]>>(input: T, key_sizes: Range<usize>) -> Vec<u8> {
    let mut predicted_key_sizes: Vec<(usize, f32)> = key_sizes
        .into_iter()
        .map(|size| (size, keysize_fitness(&input, size)))
        .collect();

    predicted_key_sizes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Equal));

    transpose(&input, predicted_key_sizes.first().unwrap().0)
        .into_iter()
        .map(|block| detect_single_byte_xor(block))
        .collect()
}

fn keysize_fitness<T: AsRef<[u8]>>(input: &T, size: usize) -> f32 {
    let chunks: Vec<&[u8]> = input.as_ref().chunks(size).take(4).collect();

    let mut distance = 0u16;

    for i in 0..4 {
        for j in i..4 {
            distance += hamming(chunks[i].to_vec(), chunks[j].to_vec()) as u16;
        }
    }

    distance as f32 / size as f32
}

fn transpose<T: AsRef<[u8]>>(input: T, block_size: usize) -> Vec<Vec<u8>> {
    let blocks: Vec<Vec<u8>> = input
        .as_ref()
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
    use crate::set1::base64::from_base64;
    use crate::set1::hex::hex_value;
    use crate::set1::repeating_key_xor::detect_repeating_key_xor;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn no_padding() {
        let key = "ICE";
        let text = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

        let expected_output = hex_value("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
        assert_eq!(repeating_key_xor(key, text), expected_output);
    }

    #[test]
    fn reversibility() {
        let key = "Sugar Hill Gang";
        let text = "Now what you hear is not a test I'm rappin' to the beat \
And me, the groove, and my friends are gonna try to move your feet \
See, I am Wonder Mike, and I'd like to say hello \
To the black, to the white, the red \
And the brown, the purple and yellow \
But first I gotta bang bang the boogie to the boogie \
Say up jump the boogie to the bang bang boogie \
Let's rock, you don't stop \
Rock the riddle that will make your body rock \
Well, so far you've heard my voice, but I brought two friends along \
And next on the mike is my man Hank, come on Hank, sing that song";

        let ciphertext = repeating_key_xor(key, text);

        let predicted_key = detect_repeating_key_xor(ciphertext, 10..20);

        assert_eq!(predicted_key.as_slice(), key.as_bytes());
    }

    #[test]
    #[ignore]
    fn break_repeating_key_xor() {
        let mut base64_encoded = String::new();
        let mut file = File::open("inputs/1_6.txt").unwrap();
        file.read_to_string(&mut base64_encoded).unwrap();
        base64_encoded = str::replace(&base64_encoded, "\n", "");

        assert_eq!(
            "Terminator X: Bring the noise".as_bytes().to_vec(),
            detect_repeating_key_xor(from_base64(base64_encoded), 2..40)
        )
    }
}
