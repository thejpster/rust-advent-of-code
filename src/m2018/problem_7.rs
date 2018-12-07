use std::collections::{BTreeMap, HashMap};

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut map = HashMap::new();
    for line in contents[0].iter() {
        let parts: Vec<&str> = line.split(" ").collect();
        let piece_a = parts[1].chars().next().unwrap();
        let piece_b = parts[7].chars().next().unwrap();
        (*map.entry(piece_b).or_insert(BTreeMap::new())).insert(piece_a, ());
        map.entry(piece_a).or_insert(BTreeMap::new());
    }

    println!("Keys: {:?}", map.keys());
    print_tree(map.clone());
    let time = calc_parallel_time(map);
    println!("Time: {}", time);
    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Worker {
    Busy(char, u32),
    Idle
}

fn time_from_char(ch: char) -> u32 {
    let ascii = ch as u32;
    (ascii - ('A' as u32)) + 61
}

fn print_tree(mut map: HashMap<char, BTreeMap<char, ()>>) {
    loop {
        let mut candidates: Vec<char> = Vec::new();
        for (k, v) in map.iter() {
            if v.len() == 0 {
                candidates.push(*k);
            }
        }
        candidates.sort();
        if candidates.len() == 0 {
            break;
        }
        let doing = candidates[0];
        print!("{}", doing);
        map.remove(&doing);
        for (_k, v) in map.iter_mut() {
            v.remove(&doing);
        }
    }
    println!();
}

fn calc_parallel_time(mut map: HashMap<char, BTreeMap<char, ()>>) -> u32 {
    let mut workers: [Worker; 5] = [Worker::Idle; 5];
    let mut total = 0;
    loop {
        total += 1;
        let mut busy_workers = false;
        println!("Workers pre-work: {:?}", workers);
        for w in workers.iter_mut() {
            *w = match w {
                Worker::Busy(done, 1) => {
                    for (_k, v) in map.iter_mut() {
                        v.remove(&done);
                    }
                    Worker::Idle
                }
                Worker::Busy(ch, left) => {
                    busy_workers = true;
                    Worker::Busy(*ch, *left - 1)
                }
                Worker::Idle => {
                    Worker::Idle
                }
            }
        }
        // println!("Workers post-work: {:?}", workers);

        let mut candidates: Vec<char> = Vec::new();
        for (k, v) in map.iter() {
            if v.len() == 0 {
                candidates.push(*k);
            }
        }
        candidates.sort();
        if candidates.len() == 0 && !busy_workers{
            break;
        }
        for w in workers.iter_mut() {
            match w {
                Worker::Idle if candidates.len() > 0 => {
                    let doing = candidates[0];
                    candidates.remove(0);
                    map.remove(&doing);
                    *w = Worker::Busy(doing, time_from_char(doing));
                }
                _ => {}
            }
        }
    }
    total - 1
}
