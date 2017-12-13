use std::collections::HashMap;

pub fn run(contents: &Vec<Vec<String>>) {
    let mut firewall: HashMap<usize, usize> = HashMap::new();
    let mut max: usize = 0;
    for layer in &contents[0] {
        let mut parts = layer.split(": ");
        let depth = parts.next().unwrap().parse::<usize>().unwrap();
        let range = parts.next().unwrap().parse::<usize>().unwrap();
        firewall.insert(depth, range);
        max = ::std::cmp::max(max, depth);
    }

    run1(firewall.clone(), max);
    run2(firewall, max);
}

fn run1(firewall: HashMap<usize, usize>, max: usize) {
    let mut score = 0;
    for depth in 0 .. max+1 {
        if let Some(range) = firewall.get(&depth) {
            if trip_scanner(depth, *range) {
                score = score + (depth * range);
            }
        }
    }
    println!("Part 1: {}", score);
}

fn run2(firewall: HashMap<usize, usize>, max: usize) {
    for sleep in 1..99999999 {
        let mut fail = false;
        for depth in 0..max+1 {
            let time = sleep + depth;
            if let Some(range) = firewall.get(&depth) {
                if trip_scanner(time, *range) {
                    fail = true;
                    break;
                }
            }
        }
        if fail == false {
            println!("Part 2: {}", sleep);
            break;
        }
    }
}

fn trip_scanner(time: usize, range: usize) -> bool {
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
