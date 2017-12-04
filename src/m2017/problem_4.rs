use std::collections::HashSet;

pub fn run(contents: &Vec<Vec<String>>) {
    let mut count = 0;
    for line in &contents[0] {
        let mut dup = false;
        let mut hm: HashSet<String> = HashSet::new();
        for word in line.split_whitespace() {
            if let Some(_) = hm.get(word) {
                dup = true;
            }
            hm.insert(word.into());
        }
        if !dup {
            count = count + 1;
        }
    }
    println!("Count: {}", count);
    count = 0;
    for line in &contents[0] {
        let mut dup = false;
        let mut hm: HashSet<String> = HashSet::new();
        for word in line.split_whitespace() {
            let mut word = word.chars().collect::<Vec<_>>();
            word.sort_by(|a, b| b.cmp(a));
            let word: String = word.iter().clone().collect();
            if let Some(_) = hm.get(&word) {
                dup = true;
            }
            hm.insert(word.into());
        }
        if !dup {
            count = count + 1;
        }
    }
    println!("Count: {}", count);
}
