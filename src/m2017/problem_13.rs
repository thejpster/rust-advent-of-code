use std::collections::HashMap;

pub fn run(contents: &[Vec<String>]) {
    let mut firewall: HashMap<usize, usize> = HashMap::new();
    for layer in &contents[0] {
        let parts: Vec<usize> = layer.split(": ").map(|x| x.parse().unwrap()).collect();
        firewall.insert(parts[0], parts[1]);
    }

    run1(&firewall);
    run2(&firewall);
}

fn run1(firewall: &HashMap<usize, usize>) {
    let mut score = 0;
    for (depth, range) in firewall.iter() {
        if trip_scanner(*depth, *range) {
            score += depth * range;
        }
    }
    println!("Part 1: {}", score);
}

fn run2(firewall: &HashMap<usize, usize>) {
    for sleep in 1..99999999 {
        let mut fail = false;
        for (depth, range) in firewall.iter() {
            let time = sleep + depth;
            if trip_scanner(time, *range) {
                fail = true;
                break;
            }
        }
        if !fail {
            println!("Part 2: {}", sleep);
            break;
        }
    }
}

fn trip_scanner(time: usize, range: usize) -> bool {
    if range == 0 {
        return false;
    }
    if range == 1 {
        return true;
    }
    let steps = 2 * (range - 1);
    let pos = time % steps;
    pos == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(trip_scanner(0, 2), true);
        assert_eq!(trip_scanner(0, 3), true);
        assert_eq!(trip_scanner(0, 4), true);
    }

    #[test]
    fn test_scroll() {
        assert_eq!(trip_scanner(0, 3), true);
        assert_eq!(trip_scanner(1, 3), false);
        assert_eq!(trip_scanner(2, 3), false);
        assert_eq!(trip_scanner(3, 3), false);
        assert_eq!(trip_scanner(4, 3), true);
        assert_eq!(trip_scanner(5, 3), false);
    }
}
