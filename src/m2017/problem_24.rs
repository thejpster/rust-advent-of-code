use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let parts: Vec<(u32, u32)> = contents[0]
        .iter()
        .map(|line| {
            let mut ports = line.split('/').map(|x| x.parse().unwrap());
            (ports.next().unwrap(), ports.next().unwrap())
        })
        .collect();

    println!("Parts: {:?}", parts);
    let mut sums = Vec::new();
    walk(&parts, 0, 0, &mut sums);
    println!("Best sum: {:?}", sums.iter().max());
    let mut bridges: Vec<Vec<(u32, u32)>> = Vec::new();
    walk2(&parts, &Vec::new(), 0, &mut bridges);
    bridges.sort_by_key(|x| 0 - total(x) as i32);
    bridges.sort_by_key(|x| 0 - x.len() as i32);
    println!(
        "Best Bridge (len {}, sum: {}): {:?}",
        bridges[0].len(),
        total(&bridges[0]),
        bridges[0]
    );
    Ok(())
}

fn total(bridge: &[(u32, u32)]) -> u32 {
    bridge.iter().map(|x| x.0 + x.1).sum()
}

fn walk(ports: &[(u32, u32)], start: u32, sum: u32, sums: &mut Vec<u32>) {
    sums.push(sum);
    for port in ports {
        if port.0 == start {
            let copy: Vec<(u32, u32)> = ports.iter().filter(|x| *x != port).cloned().collect();
            walk(&copy, port.1, sum + port.0 + port.1, sums);
        } else if port.1 == start {
            let copy: Vec<(u32, u32)> = ports.iter().filter(|x| *x != port).cloned().collect();
            walk(&copy, port.0, sum + port.0 + port.1, sums);
        }
    }
}

fn walk2(
    ports: &[(u32, u32)],
    bridge: &[(u32, u32)],
    start: u32,
    bridges: &mut Vec<Vec<(u32, u32)>>,
) {
    bridges.push(bridge.to_owned());
    for port in ports {
        if port.0 == start {
            let copy: Vec<(u32, u32)> = ports.iter().filter(|x| *x != port).cloned().collect();
            let mut bridge_copy = bridge.to_owned();
            bridge_copy.push(*port);
            walk2(&copy, &bridge_copy, port.1, bridges);
        } else if port.1 == start {
            let copy: Vec<(u32, u32)> = ports.iter().filter(|x| *x != port).cloned().collect();
            let mut bridge_copy = bridge.to_owned();
            bridge_copy.push(*port);
            walk2(&copy, &bridge_copy, port.0, bridges);
        }
    }
}
