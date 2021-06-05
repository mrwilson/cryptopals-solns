pub fn pad<T: AsRef<[u8]>>(input: T, block_size: usize) -> Vec<u8> {
    let input_size = input.as_ref().len();

    if input_size % block_size == 0 {
        return input.as_ref().to_vec();
    }

    let mut output = input.as_ref().to_vec();

    let padding = block_size - (input_size % block_size);

    output.extend(vec![padding as u8; padding]);

    output
}

pub fn unpad<T: AsRef<[u8]>>(input: T) -> Option<Vec<u8>> {
    let mut input_vec = input.as_ref().to_vec();
    let mut counter = 0;

    let last = input_vec[input_vec.len() - 1];

    while let Some(byte) = input_vec.pop() {
        if counter < last && byte != last {
            return None;
        }

        counter += 1;

        if counter >= last {
            break;
        }
    }

    Some(input_vec)
}

#[cfg(test)]
mod test {
    use crate::set2::pkcs7;

    #[test]
    fn no_need_for_padding() {
        let input = vec![0; 10];

        assert_eq!(pkcs7::pad(&input, 5), input);
    }

    #[test]
    fn padding_with_one_byte() {
        let input = vec![0; 9];

        let mut output = pkcs7::pad(&input, 5);

        assert_eq!(output.len(), 10usize);

        match output.pop() {
            Some(byte) => assert_eq!(byte, 1u8),
            _ => panic!("Should have been able to pull last byte"),
        }
    }

    #[test]
    fn invalid_padding() {
        let mut input = vec![0; 9];
        input.push(2u8);

        assert!(pkcs7::unpad(&input).is_none());
    }

    #[test]
    fn padding_with_two_bytes() {
        let input = vec![0; 8];

        let mut output = pkcs7::pad(&input, 5);

        assert_eq!(output.len(), 10usize);

        match (output.pop(), output.pop()) {
            (Some(byte1), Some(byte2)) => {
                assert_eq!(byte1, 2u8);
                assert_eq!(byte2, 2u8);
            }
            _ => panic!("Should have been able to pull last bytes"),
        }
    }

    #[test]
    fn reversibility() {
        let input = vec![0; 8];

        let output = pkcs7::pad(&input, 5);

        assert_eq!(pkcs7::unpad(output).unwrap(), input);
    }
}
