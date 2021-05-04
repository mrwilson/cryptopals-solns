extern crate openssl;

use crate::set1::aes;
use rand::{thread_rng, Rng};

pub fn encryption_oracle<T: AsRef<[u8]>>(input: T) -> (bool, Vec<u8>) {
    fn random_bytes(length: usize) -> Vec<u8> {
        return (0..length).map(|_| thread_rng().gen::<u8>()).collect();
    };

    let key = random_bytes(16);

    let padding_size: usize = thread_rng().gen_range(5..10);

    let mut padded_input = random_bytes(padding_size);
    padded_input.extend(input.as_ref());
    padded_input.extend(random_bytes(padding_size));

    let use_cbc: bool = thread_rng().gen();

    if use_cbc {
        let iv = random_bytes(16);
        (use_cbc, aes::cbc::encrypt(key, padded_input, iv))
    } else {
        (use_cbc, aes::ecb::encrypt(key, padded_input))
    }
}

mod ecb {
    use super::openssl::symm::{Cipher, Crypter, Mode};

    pub fn decrypt<T: AsRef<[u8]>, U: AsRef<[u8]>>(key: T, cipher_text: U) -> Vec<u8> {
        let cipher = Cipher::aes_128_ecb();

        let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key.as_ref(), None).unwrap();
        let mut result = vec![0; cipher_text.as_ref().len() + cipher.block_size()];

        let count = decrypter
            .update(&cipher_text.as_ref(), &mut result)
            .unwrap();

        let _ = decrypter.finalize(&mut result[count..]);

        result.truncate(cipher_text.as_ref().len());

        result
    }

    pub fn encrypt<T: AsRef<[u8]>, U: AsRef<[u8]>>(key: T, plaintext: U) -> Vec<u8> {
        let cipher = Cipher::aes_128_ecb();

        let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key.as_ref(), None).unwrap();
        let mut result = vec![0; plaintext.as_ref().len() + cipher.block_size()];

        let count = encrypter.update(&plaintext.as_ref(), &mut result).unwrap();

        let _ = encrypter.finalize(&mut result[count..]);

        result.truncate(plaintext.as_ref().len());

        result
    }
}

mod cbc {
    use crate::set1::aes;
    use crate::set1::fixed_xor::fixed_xor;
    use crate::set2::pkcs7;

    pub fn decrypt<T: AsRef<[u8]>, U: AsRef<[u8]>, V: AsRef<[u8]>>(
        key: T,
        cipher_text: U,
        iv: V,
    ) -> Vec<u8> {
        let blocks: Vec<&[u8]> = cipher_text.as_ref().chunks(16).collect();

        let mut previous_input: Vec<u8> = iv.as_ref().to_vec();
        let mut output = Vec::new();

        for block in blocks {
            let current_input = block.clone().to_vec();

            let decrypted = aes::ecb::decrypt(key.as_ref(), block);
            let plaintext = fixed_xor(previous_input.clone(), decrypted.clone());

            output.extend(plaintext.clone());

            previous_input = current_input;
        }

        pkcs7::unpad(output).unwrap()
    }

    pub fn encrypt<T: AsRef<[u8]>, U: AsRef<[u8]>, V: AsRef<[u8]>>(
        key: T,
        plaintext: U,
        iv: V,
    ) -> Vec<u8> {
        let padded_plaintext = pkcs7::pad(plaintext.as_ref(), 16);

        let blocks: Vec<&[u8]> = padded_plaintext.chunks(16).collect();

        let mut previous_input: Vec<u8> = iv.as_ref().to_vec();
        let mut output = Vec::new();

        for block in blocks {
            let encrypted = aes::ecb::encrypt(
                key.as_ref(),
                fixed_xor(&previous_input, &block.clone().to_vec()),
            );

            output.extend(encrypted.clone());
            previous_input = encrypted.clone();
        }

        output
    }

    pub fn has_repeated_blocks(decoded: Vec<u8>) -> bool {
        let mut blocks: Vec<&[u8]> = decoded.chunks(16).collect();

        let total_blocks = blocks.len();

        blocks.sort();
        blocks.dedup();

        let distinct_blocks = blocks.len();

        total_blocks != distinct_blocks
    }
}

#[cfg(test)]
mod test {
    use crate::set1::aes;
    use crate::set1::aes::cbc::has_repeated_blocks;
    use crate::set1::aes::encryption_oracle;
    use crate::set1::base64::from_base64;
    use crate::set1::io::{read_file, split};
    use crate::set2::pkcs7;
    use std::str::from_utf8;

    #[test]
    fn decrypt_test_file() {
        let key = "YELLOW SUBMARINE";

        let input: Vec<u8> = read_file("inputs/1_7.txt")
            .into_iter()
            .filter(|c| *c != b'\n')
            .collect();

        let output = aes::ecb::decrypt(key, from_base64(input));

        let as_text = from_utf8(output.as_slice()).unwrap();

        assert!(as_text.contains("Play that funky music white boy you say it, say it "))
    }

    #[test]
    fn find_aes_ecb_encrypted_entry() {
        let input: Vec<Vec<u8>> = split(read_file("inputs/1_8.txt"), b'\n');

        for ciphertext in input {
            let decoded = from_base64(ciphertext.clone());

            if has_repeated_blocks(decoded) {
                assert_eq!(ciphertext, "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a".as_bytes()
                );
            }
        }
    }

    #[test]
    fn reversibility() {
        let key = "YELLOW SUBMARINE";
        let input = "Penny Lane, is in my ear, and in my eye.";

        let output = aes::ecb::decrypt(key, aes::ecb::encrypt(key, pkcs7::pad(&input, 16)));

        assert_eq!(input.as_bytes(), pkcs7::unpad(output).unwrap());
    }

    #[test]
    fn decrypt_cbc_test_file() {
        let key = "YELLOW SUBMARINE";
        let iv = b"\x00".repeat(key.len());

        let input: Vec<u8> = read_file("inputs/2_2.txt")
            .into_iter()
            .filter(|c| *c != b'\n')
            .collect();

        let output = aes::cbc::decrypt(key, from_base64(input), iv);

        let as_text = String::from_utf8(output).unwrap();

        assert!(as_text.starts_with("I'm back and I'm ringin' the bell"));
    }

    #[test]
    fn reversibility_cbc() {
        let key = "YELLOW SUBMARINE";
        let input = "Penny Lane, is in my ear, and in my eye.";
        let iv = b"\x00".repeat(key.len());

        let output = aes::cbc::decrypt(key, aes::cbc::encrypt(key, &input, &iv), iv.clone());

        assert_eq!(input.as_bytes(), output)
    }

    #[test]
    fn detect_oracle_choice() {
        let input: Vec<u8> = "A".repeat(43).into_bytes();

        for _ in 1..1000 {
            let (using_cbc, output) = encryption_oracle(&input);

            assert_ne!(using_cbc, has_repeated_blocks(output))
        }
    }
}
