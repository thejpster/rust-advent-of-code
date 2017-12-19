pub fn run(contents: &[Vec<String>]) {
    let mut jumps: Vec<i64> = contents[0].iter().map(|x| x.parse().unwrap()).collect();
    let mut idx: i64 = 0;
    let mut steps = 0;
    while idx >= 0 && (idx as usize) < jumps.len() {
        let old_idx = idx as usize;
        idx = idx + jumps[old_idx];
        jumps[old_idx] = jumps[old_idx] + 1;
        steps += 1;
    }
    println!("steps: {}", steps);

    let mut jumps: Vec<i64> = contents[0].iter().map(|x| x.parse().unwrap()).collect();
    let mut idx: i64 = 0;
    let mut steps = 0;
    while idx >= 0 && (idx as usize) < jumps.len() {
        let old_idx = idx as usize;
        idx = idx + jumps[old_idx];
        if jumps[old_idx] >= 3 {
            jumps[old_idx] = jumps[old_idx] - 1;
        } else {
            jumps[old_idx] = jumps[old_idx] + 1;
        }
        steps += 1;
    }
    println!("steps: {}", steps);
}
