use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

use rayon::prelude::*;

fn main() -> io::Result<()> {
    let results: Vec<String> = parse_input("submitInput.txt")
        .unwrap()
        .par_iter()
        .map(|x| process(*x))
        .collect();

    let mut file = File::create("submitOutput.txt")?;
    for (case, result) in results.iter().enumerate() {
        file.write_all(format!("Case #{}: {}\n", case + 1, result).as_bytes())?;
    }
    Ok(())
}

fn process(input: u64) -> String {
    if input < 43 {
        return String::from("IMPOSSIBLE");
    }

    let mut height = 0;
    let mut incr = 1000000;
    while incr > 0 {
        match tower(height + incr, 1) {
            n if n > input => {
                incr /= 2;
            }
            _ => {
                height += incr;
            }
        }
    }

    for i in 2.. {
        if tower(height, i) > input {
            return format!("{} {}", height, tower(height, i - 1));
        }
    }
    String::from("Error")
}

fn tower(height: u64, size: u64) -> u64 {
    let mut cycle: std::iter::Cycle<std::slice::Iter<i64>> = [-2, 1].iter().cycle();
    let mut current_height: i64 = height as i64;
    let mut total = height * num::integer::div_floor((1 + size).pow(2), 4);

    for step in 1.. {
        let next = *cycle.next().unwrap();
        current_height += next;
        if current_height <= 0 {
            break;
        }
        total += current_height as u64 * (step * 8 + 2 * (size - 1));
    }
    total
}

fn parse_input(filename: &str) -> Result<Vec<u64>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut rest = lines.skip(1);
    let mut input = Vec::new();

    while let Some(l) = rest.next() {
        input.push(l?.parse().unwrap());
    }
    Ok(input)
}
