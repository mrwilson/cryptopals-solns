extern crate openssl;

use self::openssl::symm::{Cipher, Crypter, Mode};

pub fn decrypt_aes_ecb<T: AsRef<[u8]>, U: AsRef<[u8]>>(key: T, cipher_text: U) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key.as_ref(), None).unwrap();
    let mut result = vec![0; cipher_text.as_ref().len() + cipher.block_size()];

    decrypter
        .update(&cipher_text.as_ref(), &mut result)
        .unwrap();

    let len = decrypter.finalize(&mut result).unwrap();

    result.drain(0..=len);
    result
}

#[cfg(test)]
mod test {
    use crate::set1::aes::decrypt_aes_ecb;
    use crate::set1::base64::from_base64;
    use crate::set1::io::read_file;
    use std::fs::File;
    use std::io::Read;
    use std::str::from_utf8;

    #[test]
    fn decrypt_test_file() {
        let key = "YELLOW SUBMARINE";

        let input: Vec<u8> = read_file("inputs/1_7.txt")
            .into_iter()
            .filter(|c| *c != ('\n' as u8))
            .collect();

        let output = decrypt_aes_ecb(key, from_base64(input));

        let as_text = from_utf8(output.as_slice()).unwrap();

        assert!(as_text.contains("Play that funky music white boy you say it, say it "))
    }
}
