use crc32fast::Hasher;
use std::fs::File;
use std::io::{BufRead, Error, Write};
use std::collections::HashSet;
use rayon::prelude::*;
use lazy_static::lazy_static;
use std::process::Command;

const GB: u64 = 1000000000;
const MB: u64 = 1000000;

lazy_static! {
    static ref GB_HASHER: Hasher = {
        let mut h = Hasher::new();
        let v1gb : Vec<u8> = vec![0; GB as usize];
        h.update(&v1gb);
        h
    };
    static ref MB_HASHER : Hasher = {
        let mut h = Hasher::new();
        let v1mb : Vec<u8> = vec![0; MB as usize];
        h.update(&v1mb);
        h
    };
}

struct Input {
    filename: String,
    edits: Vec<(u64, u8)>,
}

fn main() -> Result<(), std::io::Error> {
    let outputs: Vec<String> = parse_input("submitInput.txt")?
        .par_iter_mut()
        .map(process)
        .collect();

    let mut f = File::create("submitOutput.txt")?;
    for result in outputs.iter() {
        f.write_all(format!("{}\n", result).as_bytes())?;
    }
    Ok(())
}

struct HashBlock {
    end: u64,
    state: Hasher,
}

fn hash_blocks(file_size: u64, edits: &Vec<((u64, u8), usize)>) -> Vec<HashBlock> {
    let split_points: HashSet<u64> = edits.iter().map(|x| (x.0).0).collect();
    let mut sorted: Vec<u64> = split_points.into_iter().collect();
    sorted.sort();
    let mut current = 0;
    let mut blocks: Vec<HashBlock> = vec![];
    for split in sorted {
        if split == current {
            continue;
        }
        let total_bytes = split - current;
        let mut bytes = total_bytes;
        let mut hasher = Hasher::new();
        while bytes >= GB {
            hasher.combine(&GB_HASHER);
            bytes -= GB;
        }
        while bytes >= MB {
            hasher.combine(&MB_HASHER);
            bytes -= MB;
        }
        let buffer: Vec<u8> = vec![0; bytes as usize];
        hasher.update(&buffer);
        blocks.push(HashBlock{
            end: current + total_bytes,
            state: hasher
        });
        current += total_bytes;
    }
    if current != file_size {
        let mut hasher = Hasher::new();
        let buffer: Vec<u8> = vec![0; (file_size - current) as usize];
        hasher.update(&buffer);
        blocks.push(HashBlock{
            end: file_size,
            state: hasher
        });
    }
    blocks
}

fn process(input: &mut Input) -> String {
    extract_file(&input.filename);
    let file = File::open(&input.filename).unwrap();
    let size = File::metadata(&file).unwrap().len();
    delete_file(&input.filename);

    let edits = correct_offsets(&mut input.edits);
    let hash_blocks = hash_blocks(size, &edits);
    let mut results : Vec<String> = vec![];
    for i in 0..=edits.len() {
        let mut round_edits = edits.clone();
        round_edits.truncate(i);
        round_edits.sort_by_key(|x| x.1);
        let e: Vec<(u64, u8)> = round_edits.into_iter().map(|x| x.0).collect();
        let hash = hash(&e, &hash_blocks);
        let r = format!("{} {}: {:08x}", input.filename, i, hash);
        println!("{}", r);
        results.push(r);
    }
    results.join("\n")
}

fn hash(edits: &Vec<(u64, u8)>, hash_blocks: &Vec<HashBlock>) -> u32 {
    let mut hasher = Hasher::new();
    let mut current_offset: u64 = 0;
    let mut i: usize = 0;
    for edit in edits.iter() {
        while current_offset != edit.0 {
            let block = &hash_blocks[i];
            hasher.combine(&block.state);
            current_offset = block.end;
            i += 1;
        }
        hasher.update(&vec![edit.1]);
    }
    while i != hash_blocks.len() {
        let block = &hash_blocks[i];
        hasher.combine(&block.state);
        i += 1;
    }
    hasher.finalize()
}

fn correct_offsets(edits: &mut Vec<(u64, u8)>) -> Vec<((u64, u8), usize)> {
    let mut insertion_indexes = vec![usize::MAX; edits.len()];

    for i in 0..edits.len() {
        let mut min: u64 = u64::MAX;
        let mut min_index: usize = usize::MAX;
        for (j, edit) in edits.iter().enumerate() {
            if insertion_indexes[j] == usize::MAX && edit.0 <= min {
                min = edit.0;
                min_index = j;
            }
        }
        insertion_indexes[min_index] = i; 
    }
    let offsets: Vec<u64> = edits.iter().map(|x| x.0).collect();
    for (i, item) in edits.iter_mut().enumerate() {
        for i in 0..i {
            if *offsets.get(i).unwrap() < item.0 {
                item.0 -= 1;
            }
        }
    }

    edits.clone().into_iter().zip(insertion_indexes).collect()
}

fn parse_input(filename: &str) -> Result<Vec<Input>, Error> {
    let f = File::open(filename).unwrap();
    let br = std::io::BufReader::new(f);
    let mut lines = br.lines();
    let mut inputs: Vec<Input> = Vec::new();

    while let Some(l) = lines.next() {
        let line = l?;
        let split = line.split(" ").collect::<Vec<&str>>();
        let filename = split[0].to_string();
        let num_edits = split[1].parse().unwrap();

        let mut edits: Vec<(u64, u8)> = Vec::new();
        for _ in 0..num_edits {
            let line = lines.next().unwrap()?;
            let edit: Vec<&str> = line.split(" ").collect();
            edits.push((edit[0].parse().unwrap(), edit[1].parse().unwrap()));
        }
        inputs.push(Input { filename, edits });
    }

    Ok(inputs)
}

fn extract_file(filename: &str) {
    Command::new("tar")
        .args(&["xf", "animals.tar.gz", &format!("animals/./{}", filename)])
        .output()
        .expect("Error decompressing file");
    Command::new("mv")
        .args(&[&format!("animals/{}", filename), "."])
        .output()
        .expect("Error moving file");
}

fn delete_file(filename: &str) {
    Command::new("rm")
        .args(&["-fr", filename])
        .output()
        .expect("Error deleting file");
}
