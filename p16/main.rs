use lp_modeler::dsl::*;
use lp_modeler::solvers::{GlpkSolver, SolverTrait};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, Error, Write};
use std::thread;

const STACK_SIZE: usize = 1024 * 1024 * 1024 * 1024;

struct Input {
    num_floors: usize,
    groups: Vec<Group>,
}

struct Group {
    count: usize,
    floors: HashSet<usize>,
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap().ok();
}

fn run() -> Result<(), std::io::Error> {
    let outputs: Vec<String> = parse_input("submitInput.txt")?
        .par_iter_mut()
        .map(process)
        .collect();

    let mut f = File::create("submitOutput.txt")?;
    for (case, result) in outputs.iter().enumerate() {
        f.write_all(format!("Case #{}: {}\n", case + 1, result).as_bytes())?;
    }
    Ok(())
}

fn process(input: &mut Input) -> String {
    let mut vars: BTreeMap<String, LpInteger> = BTreeMap::new();
    let mut group_vars: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let mut floor_vars: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for f in 0..input.num_floors {
        floor_vars.insert(f, Vec::new());
    }

    for (group_id, group) in input.groups.iter().enumerate() {
        let mut g_vars: Vec<String> = Vec::new();
        for floor in group.floors.iter() {
            let var_name = format!("G{}_F{}", group_id, floor);
            let var: LpInteger = LpInteger::new(&var_name);
            g_vars.push(var_name.clone());
            floor_vars.get_mut(floor).unwrap().push(var_name.clone());
            vars.insert(var_name, var);
        }
        group_vars.insert(group_id, g_vars);
    }

    let mut problem = LpProblem::new("ProblemasTenemosTodos", LpObjective::Minimize);
    let ref bathrooms = LpInteger::new("wc");
    // minimize WCs
    problem += bathrooms;

    // at most N bathrooms used by floor
    for (_, var_names) in floor_vars.iter() {
        if var_names.len() > 0 {
            problem += (bathrooms - sum(var_names, |n| vars.get(n).unwrap())).ge(0);
        }
    }

    // all employees with WCs
    for (group_id, var_names) in group_vars.iter() {
        problem +=
            sum(var_names, |n| vars.get(n).unwrap()).equal(input.groups[*group_id].count as f32);
    }

    let solver = GlpkSolver::new();

    match solver.run(&problem) {
        Ok((_, var_values)) => {
            for (name, value) in var_values.iter() {
                if name == "wc" {
                    println!("{}", value);
                    return value.to_string();
                }
            }
        }
        Err(msg) => println!("{}", msg),
    }

    String::new()
}

fn parse_input(filename: &str) -> Result<Vec<Input>, Error> {
    let f = File::open(filename).unwrap();
    let br = std::io::BufReader::new(f);
    let mut lines = br.lines();
    lines.next();
    let mut inputs: Vec<Input> = Vec::new();

    while let Some(l) = lines.next() {
        let line = l?;
        let split = line.split(" ").collect::<Vec<&str>>();
        let num_floors: usize = split[0].parse().unwrap();
        let num_groups: usize = split[1].parse().unwrap();

        let mut groups: Vec<Group> = Vec::new();
        for _ in 0..num_groups {
            let line = lines.next().unwrap()?;
            let count: usize = line.split(" ").next().unwrap().parse().unwrap();
            let l_floors = lines.next().unwrap()?;
            let floors: HashSet<usize> = l_floors.split(" ").map(|x| x.parse().unwrap()).collect();
            groups.push(Group { count, floors });
        }
        inputs.push(Input { num_floors, groups });
    }

    Ok(inputs)
}
