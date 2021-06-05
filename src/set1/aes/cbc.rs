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

#[cfg(test)]
mod test {
    use crate::set1::aes;
    use crate::set1::base64::from_base64;
    use crate::set1::io::read_file;
    use crate::set2::pkcs7;
    use crate::utils::random::random_bytes;

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
    fn bit_flipping_attack() {
        let key: Vec<u8> = random_bytes(16);
        let iv: Vec<u8> = random_bytes(16);

        fn encrypt_input<T: AsRef<[u8]>>(input: T, key: Vec<u8>, iv: Vec<u8>) -> Vec<u8> {
            let before = "comment1=cooking%20MCs;userdata=";
            let after = ";comment2=%20like%20a%20pound%20of%20bacon";

            let combined_input: Vec<u8> = before
                .as_bytes()
                .to_vec()
                .into_iter()
                .chain(input.as_ref().to_vec())
                .chain(after.as_bytes().to_vec())
                .collect();

            aes::cbc::encrypt(key, pkcs7::pad(combined_input, 16), iv)
        }

        fn decrypt<T: AsRef<[u8]>, U: AsRef<[u8]>>(cipher_text: T, key: U, iv: U) -> Vec<u8> {
            aes::cbc::decrypt(key, cipher_text, iv)
        }

        let mut encrypted = encrypt_input("a".repeat(16), key.clone(), iv.clone());

        ";comment2=%20lik"
            .as_bytes()
            .to_vec()
            .into_iter()
            .zip(";admin=true;c=li".as_bytes().to_vec())
            .zip(32..=47)
            .for_each(|((original_byte, desired_byte), index)| {
                encrypted[index] ^= original_byte ^ desired_byte;
            });

        let decrypted_text = decrypt(encrypted, &key, &iv);

        let output = String::from_utf8_lossy(decrypted_text.as_slice());

        assert!(output.contains(";admin=true;"));
    }
}
