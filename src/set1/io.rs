use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content.into_bytes()
}
