use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

use rayon::prelude::*;

const QWERTY: &str = "-=qwertyuiop[]sdfghjkl;'zxcvbn,./_+QWERTYUIOP{}SDFGHJKL:\"ZXCVBN<>?";
const DVORAK: &str = "[]',.pyfgcrl/=oeuidhtns-;qjkxbwvz{}\"<>PYFGCRL?+OEUIDHTNS_:QJKXBWVZ";

fn main() -> io::Result<()> {
    let results: Vec<String> = parse_input("submitInput.txt")
        .unwrap()
        .par_iter()
        .map(process)
        .collect();

    let mut file = File::create("submitOutput.txt")?;
    for (case, result) in results.iter().enumerate() {
        file.write_all(format!("Case #{}: {}\n", case + 1, result).as_bytes())?;
    }
    Ok(())
}

fn process(input: &String) -> String {
    let mut result: String = String::new();
    for c in input.chars() {
        let cc = match DVORAK.find(c) {
            Some(index) => QWERTY.chars().nth(index).unwrap(),
            _ => c,
        };
        result.push(cc);
    }
    String::from(result)
}

fn parse_input(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut rest = lines.skip(1);
    let mut input = Vec::new();

    while let Some(l) = rest.next() {
        input.push(l?);
    }
    Ok(input)
}
