use rayon::prelude::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

struct Input {
    query: String,
}

fn main() -> io::Result<()> {
    let ranking = build_ranking();
    let results: Vec<String> = parse_input("submitInput.txt")
        .unwrap()
        .par_iter()
        .map(|x| process(x, &ranking))
        .collect();

    let mut file = fs::File::create("submitOutput.txt")?;
    for (case, result) in results.iter().enumerate() {
        file.write_all(format!("Case #{}: {}\n", case + 1, result).as_bytes())?;
    }
    Ok(())
}

fn process(input: &Input, rank: &Vec<(String, u32)>) -> String {
    match input.query.parse::<usize>() {
        Ok(n) => {
            let entry: &(String, u32) = rank.get(n - 1).unwrap();
            format!("{} {}", entry.0, entry.1)
        }
        _ => {
            let index = rank
                .par_iter()
                .position_first(|x| x.0 == input.query)
                .unwrap();
            let entry: &(String, u32) = rank.get(index).unwrap();
            format!("{} #{}", entry.1, index + 1)
        }
    }
}

fn build_ranking() -> Vec<(String, u32)> {
    let re = Regex::new(r"([^a-zA-ZáéíñóúüÁÉÍÑÓÚÜ]|\s)+").unwrap();
    let book = fs::read_to_string("pg17013.txt").unwrap();
    let clean = re.replace_all(&book, " ");

    let mut count: BTreeMap<String, u32> = BTreeMap::new();
    for word in clean
        .split(" ")
        .filter(|x| x.chars().count() >= 3)
        .map(|x| x.to_lowercase())
    {
        *count.entry(word.into()).or_insert(0) += 1;
    }

    let mut count_vec: Vec<(String, u32)> = count.into_iter().collect();
    count_vec.sort_by(|x, y| {
        if x.1 != y.1 {
            y.1.cmp(&x.1)
        } else {
            x.0.cmp(&y.0)
        }
    });

    count_vec
}

fn parse_input(filename: &str) -> Result<Vec<Input>, io::Error> {
    let file = fs::File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut rest = lines.skip(1);
    let mut input = Vec::new();

    while let Some(l) = rest.next() {
        input.push(Input { query: l? });
    }
    Ok(input)
}
