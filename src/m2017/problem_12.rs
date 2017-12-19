use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

pub fn run(contents: &[Vec<String>]) {
    let mut hm: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in &contents[0] {
        let mut parts = line.split(" <-> ");
        let primary: u32 = parts.next().unwrap().parse().unwrap();
        let secondary: Vec<u32> = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        match hm.entry(primary) {
            Entry::Vacant(e) => {
                e.insert(secondary);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().extend(secondary);
            }
        }
    }

    println!("Count ({}): {}", 0, count(&hm, &mut HashSet::new(), 0));
    let mut groups = 0;
    while !hm.is_empty() {
        let mut seen = HashSet::new();
        let search = *hm.keys().next().unwrap();
        count(&hm, &mut seen, search);
        groups += 1;
        for x in seen {
            hm.remove(&x);
        }
    }
    println!("Groups: {}", groups);
}

fn count(hm: &HashMap<u32, Vec<u32>>, seen: &mut HashSet<u32>, index: u32) -> u32 {
    assert!(seen.insert(index));
    let x = hm.get(&index).expect("None bi-dir assoc");
    let mut t = 1;
    for v in x {
        if !seen.contains(v) {
            t += count(hm, seen, *v);
        }
    }
    t
}
