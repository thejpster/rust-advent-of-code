use std::collections::{BTreeMap, HashMap};

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut map = HashMap::new();
    for line in contents[0].iter() {
        let parts: Vec<&str> = line.split(" ").collect();
        let piece_a = parts[1];
        let piece_b = parts[7];
        (*map.entry(piece_b).or_insert(BTreeMap::new())).insert(piece_a, ());
        map.entry(piece_a).or_insert(BTreeMap::new());
    }

    println!("Keys: {:?}", map.keys());
    print_tree(map.clone());
    println!();
    Ok(())
}

fn print_tree<'a>(mut map: HashMap<&'a str, BTreeMap<&'a str, ()>>) {
    loop {
        let mut candidates: Vec<&str> = Vec::new();
        for (k, v) in map.iter() {
            if v.len() == 0 {
                candidates.push(k.clone());
            }
        }
        candidates.sort();
        if candidates.len() == 0 {
            break;
        }
        let doing = candidates[0];
        print!("{}", doing);
        map.remove(doing);
        for (_k, v) in map.iter_mut() {
            v.remove(doing);
        }
    }
}
