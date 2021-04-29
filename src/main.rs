mod set1;

use crate::set1::bhattacharya::bhattacharya;
use crate::set1::single_byte_xor::*;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    if let Ok(lines) = read_lines("./4.txt") {

        let mut output: Vec<(String, u8, f32)> = lines.into_iter()
            .map(|line| {
                return detect_single_byte_xor(&line.unwrap());
            })
            .collect();

        output.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Equal));

        let winner =  output.last().unwrap().clone();
        println!("{0}, {1}, {2}", winner.0, winner.1 as char, winner.2)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
