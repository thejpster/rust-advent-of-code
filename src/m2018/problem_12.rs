use failure::Error;
use std::collections::HashMap;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut padding = 5;
    let input = &contents[0];
    let initial_state: Vec<bool> = input[0][15..].chars().map(|x| x == '#').collect();
    println!("initial_state = {:?}", initial_state);
    let mut map = HashMap::new();
    for line in input[2..].iter() {
        println!("{} = {}", &line[0..5], &line[9..10]);
        let a: Vec<bool> = line[0..5].chars().map(|x| x == '#').collect();
        let b = if &line[9..10] == "#" { true } else { false };
        map.insert(a, b);
    }
    // println!("map: {:?}", map);

    // stretch out a bit
    let mut state = vec![false; padding];
    state.extend(initial_state);
    state.extend(vec![false; padding]);
    let max_steps: usize = 50_000_000_000;
    for step in 0..max_steps {
        // println!("state {:02}: {:?}", step, state.iter().map(|x| if *x { '#' } else {'.'}).collect::<String>());
        let mut new_state = state.clone();
        let mut total = 0;
        for x in 2..state.len() - 2 {
            let slice = vec![
                state[x - 2],
                state[x - 1],
                state[x],
                state[x + 1],
                state[x + 2],
            ];
            new_state[x] = *map.get(&slice).unwrap_or(&false);
            let index = x as i32 - padding as i32;
            if new_state[x] {
                total = total + index;
            }
        }
        if new_state[0..padding].iter().any(|x| *x) {
            padding += 5;
            new_state.insert(0, false);
            new_state.insert(0, false);
            new_state.insert(0, false);
            new_state.insert(0, false);
            new_state.insert(0, false);
        }
        if new_state[new_state.len() - 5..].iter().any(|x| *x) {
            new_state.push(false);
            new_state.push(false);
            new_state.push(false);
            new_state.push(false);
            new_state.push(false);
        }
        println!(
            "state {} total: {}, padding: {}, len: {}",
            step + 1,
            total,
            padding,
            new_state.len()
        );
        state = new_state;
    }

    println!(
        "state {}: {:?}",
        max_steps,
        state
            .iter()
            .map(|x| if *x { '#' } else { '.' })
            .collect::<String>()
    );

    let mut total = 0;
    for x in 0..state.len() {
        let index = x as i32 - padding as i32;
        if state[x] == true {
            total += index;
        }
    }
    println!("Total: {}", total);

    Ok(())
}
