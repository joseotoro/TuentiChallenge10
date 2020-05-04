use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

use rayon::prelude::*;

#[derive(Debug)]
struct Input {
    p1: String,
    p2: String,
}

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

fn process(input: &Input) -> String {
    let result: &str;
    if input.p1 == input.p2 {
        result = "-";
    } else if input.p1 == "P" && input.p2 == "S" {
        result = "S";
    } else if input.p1 == "P" && input.p2 == "R" {
        result = "P";
    } else if input.p1 == "R" && input.p2 == "S" {
        result = "R";
    } else {
        result = "-";
    }
    String::from(result)
}

fn parse_input(filename: &str) -> Result<Vec<Input>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut rest = lines.skip(1);
    let mut input = Vec::new();

    while let Some(l) = rest.next() {
        let line = l?;
        let mut split: Vec<&str> = line.split(" ").collect();
        split.sort();
        input.push(Input {
            p1: split[0].to_string(),
            p2: split[1].to_string(),
        });
    }
    Ok(input)
}
