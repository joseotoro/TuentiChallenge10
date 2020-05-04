use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io;
use telnet::{Telnet, TelnetEvent};

fn read(connection: &mut Telnet) -> String {
    let event = connection.read().expect("Read error");
    match event {
        TelnetEvent::Data(buffer) => String::from_utf8(buffer.to_vec()).unwrap(),
        _ => String::default(),
    }
}

fn main() -> io::Result<()> {
    let mut connection = Telnet::connect(("52.49.91.111", 2092), 8192).expect("Error connecting");
    let mut rest_connections: Vec<Telnet> = Vec::new();
    let mut servers_id: Vec<u32> = Vec::new();
    for _ in 0..6 {
        let mut c = Telnet::connect(("52.49.91.111", 2092), 8192).expect("Error connecting");
        let l = read(&mut c);
        let server_id: u32 = l.lines().next().unwrap().split(": ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        rest_connections.push(c);
        servers_id.push(server_id);
        println!("{}", server_id)
    }
    let mut r: String;
    let mut servers: Vec<u32> = Vec::new();
    let mut value: u32 = 1;
    let mut promises: HashSet<u32> = HashSet::new();
    let mut current_owner: u32 = 0;
    let mut sent = false;
    loop {
        r = read(&mut connection);
        for line in r.lines() {
            println!("{}", line);
            match line {
                l if l.contains("LEARN") => {
                    sent = false;
                    current_owner = line.split("owner: ").collect::<Vec<&str>>()[1]
                        .split("}")
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap();
                    promises.clear();
                    servers = line.split("[").collect::<Vec<&str>>()[1]
                        .split("]")
                        .next()
                        .unwrap()
                        .split(",")
                        .map(|x| x.parse().unwrap())
                        .collect();
                    value += 1;
                    for server in servers.iter() {
                        send_prepare(value, *server, &mut connection);
                    }
                    for connection in rest_connections.iter_mut() {
                        send_promise(value, connection);
                    }
                }
                l if l.contains("9 -> ACCEPTED") => {}
                l if l.contains("-> ACCEPTED") => {}
                l if l.contains("-> PROMISE") => {
                    let server: u32 = l.split(" ").collect::<Vec<&str>>()[2].parse().unwrap();
                    let mut new_servers = servers.clone();
                    // Add new servers.
                    for s in servers_id.iter() {
                        if !new_servers.contains(&s) {
                            new_servers.push(*s);
                            break;
                        }
                    }
                    // No change, remove old members.
                    if new_servers.len() == servers.len() {
                        for i in 0.. {
                            let s = *servers.get(i).unwrap();
                            if s != 9 && !servers_id.contains(&s) {
                                new_servers.remove(i);
                                break;
                            } else {
                                // First member known, start sending accept 9.
                                current_owner = 9;
                                break;
                            }
                        }
                    }
                    promises.insert(server);
                    if promises.len() >= servers.len() / 2 && !sent {
                        sent = true;
                        for server in promises.iter() {
                            send_accept(
                                value,
                                9,
                                &new_servers,
                                current_owner,
                                *server,
                                &mut connection,
                            );
                        }
                        for connection in rest_connections.iter_mut() {
                            for server in servers.iter() {
                                send_accepted(*server, &servers, connection);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn send_prepare(value: u32, dest: u32, connection: &mut Telnet) {
    connection
        .write(format!("PREPARE {{{},9}} -> {}\n", value, dest).as_bytes())
        .expect("Error sending command");
}

fn send_promise(value: u32, connection: &mut Telnet) {
    connection
        .write(format!("PROMISE {{{},9}} -> 9\n", value).as_bytes())
        .expect("Error sending command");
}

fn send_accepted(dest: u32, servers: &Vec<u32>, connection: &mut Telnet) {
    let servers: Vec<String> = servers.into_iter().map(|i| i.to_string()).collect();
    connection
        .write(
            format!(
                "ACCEPTED {{servers: [{}], secret_owner: 9}} -> {}",
                servers.join(","),
                dest
            )
            .as_bytes(),
        )
        .expect("Error sending command");
}

fn send_accept(
    v: u32,
    from: u32,
    servers: &Vec<u32>,
    secret_owner: u32,
    dest: u32,
    connection: &mut Telnet,
) {
    let servers: Vec<String> = servers.into_iter().map(|i| i.to_string()).collect();
    let s = format!(
        "ACCEPT {{id: {{{},{}}}, value: {{servers: [{}], secret_owner: {}}}}} -> {}\n",
        v,
        from,
        servers.join(","),
        secret_owner,
        dest,
    );
    connection
        .write(s.as_bytes())
        .expect("Error sending command");
}
