use std::collections::HashSet;

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut count = 0;
    for line in &contents[0] {
        let mut dup = false;
        let mut set: HashSet<String> = HashSet::new();
        for word in line.split_whitespace() {
            if !set.insert(word.into()) {
                dup = true;
            }
        }
        if !dup {
            count += 1;
        }
    }
    println!("Count: {}", count);
    count = 0;
    for line in &contents[0] {
        let mut dup = false;
        let mut set: HashSet<String> = HashSet::new();
        for word in line.split_whitespace() {
            let mut chars = word.chars().collect::<Vec<_>>();
            chars.sort();
            let word: String = chars.iter().collect();
            if !set.insert(word) {
                dup = true;
            }
        }
        if !dup {
            count += 1;
        }
    }
    println!("Count: {}", count);
    Ok(())
}
