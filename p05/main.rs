use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

use rayon::prelude::*;

#[derive(Debug)]
struct Input {
    n: u64,
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
    match input.n {
        0..=19 | 30..=39 | 59 => String::from("IMPOSSIBLE"),
        _ => (input.n / 20).to_string(),
    }
}

fn parse_input(filename: &str) -> Result<Vec<Input>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut rest = lines.skip(1);
    let mut input = Vec::new();

    while let Some(l) = rest.next() {
        input.push(Input {
            n: l?.parse().unwrap(),
        });
    }
    Ok(input)
}
