use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content.into_bytes()
}

pub fn split<T: AsRef<[u8]>>(input: T, char: u8) -> Vec<Vec<u8>> {
    let mut output: Vec<Vec<u8>> = Vec::new();
    let mut intermediate: Vec<u8> = Vec::new();

    for c in input.as_ref() {
        if *c == char {
            output.push(intermediate.clone());
            intermediate.clear();
        } else {
            intermediate.push(*c);
        }
    }

    output.push(intermediate);

    output
}

#[cfg(test)]
mod test {
    use crate::set1::io::split;

    #[test]
    fn example_1() {
        let input = "ab\ncd";

        let output = split(input, '\n' as u8);

        assert_eq!(output[0], "ab".as_bytes());
        assert_eq!(output[1], "cd".as_bytes());
    }
}
