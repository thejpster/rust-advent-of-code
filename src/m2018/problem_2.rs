//! Day 2

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    assert_eq!(score("abcdef"), (0, 0));
    assert_eq!(score("bababc"), (1, 1));
    assert_eq!(score("abbcde"), (1, 0));
    assert_eq!(score("abcccd"), (0, 1));
    assert_eq!(score("aabcdd"), (1, 0));
    assert_eq!(score("abcdee"), (1, 0));
    assert_eq!(score("ababab"), (0, 1));
    let mut two_total = 0;
    let mut three_total = 0;
    for word in &contents[0] {
        let (two, three) = score(word);
        println!("Word: {}", word);
        two_total += two;
        three_total += three;
    }
    println!("Csum: {}", two_total * three_total);

    for (idx, first) in contents[0].iter().enumerate() {
        for second in contents[0][idx..].iter() {
            let mut diffs = 0;
            for (a, b) in first.chars().zip(second.chars()) {
                if a != b {
                    diffs += 1;
                }
            }
            if diffs == 1 {
                for (a, b) in first.chars().zip(second.chars()) {
                    if a == b {
                        print!("{}", a);
                    }
                }
                println!();
                break;
            }
        }
    }

    Ok(())
}



fn score(input: &str) -> (usize, usize) {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    for c in input.chars() {
        *m.entry(c).or_insert(0) += 1;
    }
    let mut has_two = 0;
    let mut has_three = 0;
    for &v in m.values () {
        if v == 2 {
            has_two = 1;
        }
        if v == 3 {
            has_three = 1;
        }
    }
    (has_two, has_three)
}
