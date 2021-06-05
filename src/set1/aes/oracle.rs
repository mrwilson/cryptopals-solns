use crate::set1::aes;
use crate::utils::random::{coin_flip, random_bytes, random_in_range};

pub fn encryption_oracle<T: AsRef<[u8]>>(input: T) -> (bool, Vec<u8>) {
    let key = random_bytes(16);

    let padding_size: usize = random_in_range(5, 10);

    let mut padded_input = random_bytes(padding_size);
    padded_input.extend(input.as_ref());
    padded_input.extend(random_bytes(padding_size));

    let use_cbc = coin_flip();

    if use_cbc {
        let iv = random_bytes(16);
        (use_cbc, aes::cbc::encrypt(key, padded_input, iv))
    } else {
        (use_cbc, aes::ecb::encrypt(key, padded_input))
    }
}

#[cfg(test)]
mod test {
    use crate::set1::aes::ecb::has_repeated_blocks;
    use crate::set1::aes::oracle::encryption_oracle;

    #[test]
    fn detect_oracle_choice() {
        let input: Vec<u8> = "A".repeat(43).into_bytes();

        for _ in 1..1000 {
            let (using_cbc, output) = encryption_oracle(&input);

            assert_ne!(using_cbc, has_repeated_blocks(output))
        }
    }
}
