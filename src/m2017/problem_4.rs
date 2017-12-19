use std::collections::HashSet;

pub fn run(contents: &[Vec<String>]) {
    let mut count = 0;
    for line in &contents[0] {
        let mut dup = false;
        let mut hm: HashSet<String> = HashSet::new();
        for word in line.split_whitespace() {
            dup |= hm.contains(word);
            hm.insert(word.into());
        }
        if !dup {
            count += 1;
        }
    }
    println!("Count: {}", count);
    count = 0;
    for line in &contents[0] {
        let mut dup = false;
        let mut hm: HashSet<String> = HashSet::new();
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();
            let word: String = chars.iter().collect();
            dup |= hm.contains(&word);
            hm.insert(word);
        }
        if !dup {
            count += 1;
        }
    }
    println!("Count: {}", count);
}
