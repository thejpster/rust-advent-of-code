use std::collections::HashMap;

pub fn run(contents: &[Vec<String>]) {
    let mut count = 0;
    for room in &contents[0] {
        // println!("Line: {}", room);
        count += check_csum(room);
    }
    println!("Count: {}", count);
}

fn shift_string(s: &str, shift: u32) -> String {
    String::from_utf8(
        s.bytes()
            .map(|c| {
                if c == 45 {
                    32
                } else {
                    let idx = c - 97;
                    let new_idx = (u32::from(idx) + shift) % 26;
                    (new_idx + 97) as u8
                }
            })
            .collect(),
    ).unwrap()
}

fn check_csum(word: &str) -> u32 {
    let parts: Vec<&str> = word.split('-').collect();
    let tail: Vec<&str> = parts[parts.len() - 1].split('[').collect();
    let room = parts[0..parts.len() - 1].join("");
    let roomdash = parts[0..parts.len() - 1].join("-");
    let mut hs = HashMap::new();
    for ch in room.chars() {
        let new_count = if let Some(c) = hs.get(&ch) { c + 1 } else { 1 };
        hs.insert(ch, new_count);
    }
    let mut pairs: Vec<(char, u32)> = hs.iter().map(|(k, v)| (*k, *v)).collect();
    pairs.sort_by(|a, b| {
        // Same count
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            // Different count
            b.1.cmp(&a.1)
        }
    });
    let key: String = pairs.iter().map(|x| x.0).take(5).collect();
    if format!("{}]", key) == tail[1] {
        // println!("***");
        // println!("Room: {:?}", room);
        // println!("Set: {:?}", hs);
        // println!("Pairs: {:?}", pairs);
        // println!("Key: {}", key);
        println!("Tail: {:?}", tail);
        let shift = tail[0].parse::<u32>().unwrap();
        println!("Shifted: {}", shift_string(&roomdash, shift));
        shift
    } else {
        0
    }
}
