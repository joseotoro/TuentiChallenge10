use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

use rayon::prelude::*;

struct Input {
  matches: Vec<Match>,
}

#[derive(Debug)]
struct Match {
  p1: i32,
  p2: i32,
  player1_wins: bool,
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
  let mut losers = HashSet::new();
  for m in &input.matches {
    let loser = if m.player1_wins { m.p2 } else { m.p1 };
    losers.insert(loser);
  }
  let mut winner = 0;
  for i in 1..=losers.len() {
    if !losers.contains(&(i as i32)) {
      winner = i;
      break;
    }
  }
  winner.to_string()
}

fn parse_input(filename: &str) -> Result<Vec<Input>, io::Error> {
  let file = File::open(filename)?;
  let lines = BufReader::new(file).lines();
  let mut rest = lines.skip(1);
  let mut input = Vec::new();

  while let Some(n) = rest.next() {
    let mut matches: Vec<Match> = Vec::new();
    for _ in 0..n?.parse().unwrap() {
      let m = rest.next().unwrap()?;
      let split: Vec<&str> = m.split(" ").collect();
      matches.push(Match {
        p1: split[0].parse().unwrap(),
        p2: split[1].parse().unwrap(),
        player1_wins: split[2] == "1",
      })
    }
    input.push(Input { matches });
  }
  Ok(input)
}
