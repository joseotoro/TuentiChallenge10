use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

use rayon::prelude::*;

#[derive(Debug)]
struct Input {
    n: i32,
    forbidden: HashSet<i32>,
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
    fn aux(
        n: i32,
        max: i32,
        forbidden: &HashSet<i32>,
        memoize: &mut BTreeMap<(i32, i32), u32>,
    ) -> u32 {
        if memoize.contains_key(&(n, max)) {
            *memoize.get(&(n, max)).unwrap()
        } else if n == 0 {
            1
        } else if n < 0 || max == 0 && n > 0 {
            0
        } else {
            let mut total = aux(n, max - 1, forbidden, memoize);
            if !forbidden.contains(&max) {
                total += aux(n - max, max, forbidden, memoize);
            }
            memoize.insert((n, max), total);
            total
        }
    }

    aux(input.n, input.n - 1, &input.forbidden, &mut BTreeMap::new()).to_string()
}

fn parse_input(filename: &str) -> Result<Vec<Input>, io::Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut rest = lines.skip(1);
    let mut input = Vec::new();

    while let Some(l) = rest.next() {
        let line = l?;
        let split: Vec<&str> = line.split(" ").collect();
        let n = split[0].parse().unwrap();
        let mut forbidden = HashSet::new();
        for i in 1..split.len() {
            forbidden.insert(split[i].parse().unwrap());
        }
        input.push(Input { n, forbidden });
    }
    Ok(input)
}
