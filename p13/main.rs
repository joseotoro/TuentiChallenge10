use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let results: Vec<String> = parse_input("testInput.txt")
        .unwrap()
        .iter()
        .map(|x| process(*x))
        .collect();

    let mut file = File::create("testOutput.txt")?;
    for (case, result) in results.iter().enumerate() {
        file.write_all(format!("Case #{}: {}\n", case + 1, result).as_bytes())?;
    }
    Ok(())
}

fn process(input: u64) -> String {
    if input < 43 {
        return String::from("IMPOSSIBLE");
    }

    let mut low = 0;
    let mut high = num::integer::sqrt(input) * 2;
    let mut mid = 0;
    while low <= high {
        mid = (low + high) / 2;
        match tower(mid, 1) {
            n if n > input => {
                high = mid - 1;
            }
            n if n < input => {
                low = mid + 1;
            }
            n => return format!("{} {}", mid, n),
        }
    }
    while tower(mid, 1) > input {
        mid -= 1;
    }

    for i in 2.. {
        if tower(mid, i) > input {
            return format!("{} {}", mid, tower(mid, i - 1));
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
