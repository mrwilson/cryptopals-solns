use openssl::symm::{Cipher, Crypter, Mode};

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

pub fn has_repeated_blocks(decoded: Vec<u8>) -> bool {
    let mut blocks: Vec<&[u8]> = decoded.chunks(16).collect();

    let total_blocks = blocks.len();

    blocks.sort();
    blocks.dedup();

    let distinct_blocks = blocks.len();

    total_blocks != distinct_blocks
}

#[cfg(test)]
mod test {
    use crate::set1::aes;
    use crate::set1::aes::ecb::has_repeated_blocks;
    use crate::set1::base64::from_base64;
    use crate::set1::io::{read_file, split};
    use crate::set2::pkcs7;
    use crate::utils::random::{random_bytes, random_in_range};
    use std::collections::HashMap;
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
    fn byte_by_byte_decryption() {
        let key = [
            162, 16, 247, 214, 196, 106, 167, 142, 100, 136, 17, 82, 127, 118, 107, 212,
        ]
        .to_vec();

        let mut derived_text: Vec<u8> = vec![0; 16];

        loop {
            let lookup = create_lookup(&derived_text[derived_text.len() - 15..], &key);

            let nudge = vec![0; 15 - (derived_text.len() % 16)];

            let output = encrypt_with_unknown_string(nudge, &key);
            let encryption: Vec<&[u8]> = output.chunks(16).collect();

            let block = encryption
                .get((derived_text.len() / 16) - 1)
                .unwrap()
                .to_vec();

            if lookup.contains_key(&block) {
                derived_text.push(*lookup.get(&block).unwrap());
            } else {
                derived_text = derived_text.drain(16..).into_iter().collect();
                break;
            }
        }

        let recovered_text = String::from_utf8(derived_text).unwrap();

        assert_eq!(
            "Rollin' in my 5.0\n\
            With my rag-top down so my hair can blow\n\
            The girlies on standby waving just to say hi\n\
            Did you stop? No, I just drove by\n",
            recovered_text
        );

        fn create_lookup(input: &[u8], key: &Vec<u8>) -> HashMap<Vec<u8>, u8> {
            (0..=128u8)
                .map(|i| {
                    let mut block = input.clone().to_vec();
                    block.push(i);
                    (aes::ecb::encrypt(key, block), i)
                })
                .collect()
        }

        fn encrypt_with_unknown_string<T: AsRef<[u8]>>(input: T, key: &[u8]) -> Vec<u8> {
            let prepend = from_base64(
                "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIG\
                Rvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGll\
                cyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQ\
                pEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK",
            )
            .clone();

            let text: Vec<u8> = input
                .as_ref()
                .iter()
                .chain(prepend.iter())
                .cloned()
                .collect();

            aes::ecb::encrypt(key, &text)
        }
    }

    #[test]
    fn byte_by_byte_decryption_2() {
        let key = [
            162, 16, 247, 214, 196, 106, 167, 142, 100, 136, 17, 82, 127, 118, 107, 212,
        ]
        .to_vec();

        let random_starting_bytes: Vec<u8> = random_bytes(random_in_range(0, 100));

        // Uh-oh! There's some random junk at the beginning of the ECB input

        // We can discover its length by repeatedly padding with increasingly long strings of "a"
        // until we get a duplicate block.

        // This will tell us both the required length to pad out the block (as length % 16)
        // and the length of the junk (by finding the indices of the repeated blocks)

        // Then it's almost identical to the previous byte-by-byte but with some additional offset
        // to ignore the preceding junk

        let mut length_to_pad_out_block = 0;
        let mut blocks_of_junk = 0;

        for i in 32..=47 {
            let output = encrypt_with_unknown_string_and_preceding_bytes(
                &random_starting_bytes,
                "a".repeat(i),
                &key,
            );

            if aes::ecb::has_repeated_blocks(output.clone()) {
                println!("Repeated blocks detected after padding with {0}", i);
                length_to_pad_out_block = i % 16;

                let chunks: Vec<&[u8]> = output.chunks(16).collect();

                for i in 0..chunks.len() - 2 {
                    if &chunks[i] == &chunks[i + 1] {
                        println!("Repeated blocks found at ({0},{1})", i, i + 1);
                        blocks_of_junk = i
                    }
                }

                break;
            }
        }

        let mut derived_text: Vec<u8> = vec![0; 16];

        loop {
            let lookup = create_lookup(&derived_text[derived_text.len() - 15..], &key);

            println!("{:?}", &derived_text);

            let input: Vec<u8> = vec![1u8; length_to_pad_out_block]
                .iter()
                .chain(vec![0u8; 15 - (derived_text.len() % 16)].iter())
                .map(|x| *x)
                .collect();

            let output = encrypt_with_unknown_string_and_preceding_bytes(
                &random_starting_bytes,
                input,
                &key,
            );
            let encryption: Vec<&[u8]> = output.chunks(16).collect();

            let block = encryption
                .get(blocks_of_junk + (derived_text.len() / 16) - 1)
                .unwrap()
                .to_vec();

            if lookup.contains_key(&block) {
                derived_text.push(*lookup.get(&block).unwrap());
            } else {
                derived_text = derived_text.drain(16..).into_iter().collect();
                break;
            }
        }

        let recovered_text = String::from_utf8(derived_text).unwrap();

        assert_eq!(
            "Rollin' in my 5.0\n\
            With my rag-top down so my hair can blow\n\
            The girlies on standby waving just to say hi\n\
            Did you stop? No, I just drove by\n",
            recovered_text
        );

        fn create_lookup(input: &[u8], key: &Vec<u8>) -> HashMap<Vec<u8>, u8> {
            (0..=128u8)
                .map(|i| {
                    let mut block = input.clone().to_vec();
                    block.push(i);
                    (aes::ecb::encrypt(key, block), i)
                })
                .collect()
        }

        fn encrypt_with_unknown_string_and_preceding_bytes<T: AsRef<[u8]>>(
            random_starting_bytes: &Vec<u8>,
            input: T,
            key: &[u8],
        ) -> Vec<u8> {
            let prepend = from_base64(
                "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIG\
            Rvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGll\
            cyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQ\
            pEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK",
            )
            .clone();

            let text: Vec<u8> = random_starting_bytes
                .iter()
                .chain(input.as_ref().iter())
                .chain(prepend.iter())
                .cloned()
                .collect();

            aes::ecb::encrypt(key, &text)
        }
    }
}
