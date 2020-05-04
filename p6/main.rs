use std::collections::HashSet;
use std::io;
use telnet::{Telnet, TelnetEvent};

use rayon::prelude::*;

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

const MAP_WIDTH: usize = 250;
const MAP_HEIGHT: usize = 250;

type Map = [[char; MAP_WIDTH]; MAP_HEIGHT];

fn main() -> io::Result<()> {
    let mut labyrinth: Map = [[' '; MAP_WIDTH]; MAP_HEIGHT];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut connection = Telnet::connect(("52.49.91.111", 2003), 256).expect("Error connecting");

    process(
        125,
        125,
        &mut visited,
        &mut labyrinth,
        &mut connection,
        (0, 0),
    );
    Ok(())
}

fn print_labyrinth(l: &mut Map) {
    let l_lines: Vec<String> = l
        .par_iter()
        .map(|x| {
            let arr_str: String = x.into_iter().collect();
            arr_str
        })
        .collect();
    println!("{}", l_lines.join("\n"));
}

fn process(
    x: usize,
    y: usize,
    visited: &mut HashSet<(usize, usize)>,
    labyrinth: &mut Map,
    connection: &mut Telnet,
    last_movement: (i32, i32),
) {
    //print_labyrinth(labyrinth);
    visited.insert((x, y));
    update(connection, labyrinth, x, y);
    let movements = generate_movements(x, y, labyrinth);
    for movement in movements {
        let xx = (x as i32 + movement.0) as usize;
        let yy = (y as i32 + movement.1) as usize;
        if !visited.contains(&(xx, yy)) {
            send_movement(movement, connection);
            process(xx, yy, visited, labyrinth, connection, movement);
        }
    }
    let xx = (x as i32 - last_movement.0) as usize;
    let yy = (y as i32 - last_movement.1) as usize;
    send_movement((-last_movement.0, -last_movement.1), connection);
    update(connection, labyrinth, xx, yy);
}

fn send_movement(movement: (i32, i32), connection: &mut Telnet) {
    connection
        .write(movement_to_command(movement.0, movement.1).as_bytes())
        .expect("Error sending command");
}

fn generate_movements(x: usize, y: usize, labyrinth: &mut Map) -> Vec<(i32, i32)> {
    let mut valid_movements: Vec<(i32, i32)> = Vec::new();
    for dx in -2..=2 {
        for dy in -2..=2 {
            if dx == 0 || dy == 0 {
                continue;
            }
            if i32::abs(dx) + i32::abs(dy) != 3 {
                continue;
            }
            if labyrinth[(y as i32 + dy) as usize][(x as i32 + dx) as usize] != '#' {
                valid_movements.push((dx, dy));
            }
        }
    }
    valid_movements
}

fn update(connection: &mut Telnet, labyrinth: &mut Map, x: usize, y: usize) -> () {
    let event = connection.read().expect("Read error");
    let response;
    match event {
        TelnetEvent::Data(buffer) => {
            response = String::from_utf8(buffer.to_vec()).unwrap();
        }
        _ => {
            response = String::from("Error");
        }
    }
    println!("{}", response);
    let mut lines = response.lines();
    for i in 0..5 {
        let mut chars = lines.next().unwrap().chars();
        for j in 0..5 {
            labyrinth[y + i - 2][x + j - 2] = chars.next().unwrap();
        }
    }
    while let Some(l) = lines.next() {
        if l.trim() != "" {
            println!("{}", l.trim());
            std::io::stdin().read_line(&mut String::new()).ok();
        }
    }
}

fn movement_to_command(dx: i32, dy: i32) -> String {
    let mut command = String::new();
    command.push_str(&i32::abs(dy).to_string());
    if dy < 0 {
        command.push_str("U");
    } else {
        command.push_str("D");
    }
    command.push_str(&i32::abs(dx).to_string());
    if dx < 0 {
        command.push_str("L");
    } else {
        command.push_str("R");
    }
    command.push_str("\n");
    command
}
