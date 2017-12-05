pub fn run(contents: &Vec<Vec<String>>) {
    let mut jumps: Vec<i64> = contents[0].iter().map(|x| x.parse().unwrap()).collect();
    let mut idx: i64 = 0;
    let mut steps = 0;
    while idx >= 0 && (idx as usize) < jumps.len() {
        let old_idx = idx;
        idx = idx + jumps[idx as usize];
        jumps[old_idx as usize] = jumps[old_idx as usize] + 1;
        steps = steps + 1;
    }
    println!("steps: {}", steps);

    let mut jumps: Vec<i64> = contents[0].iter().map(|x| x.parse().unwrap()).collect();
    let mut idx: i64 = 0;
    let mut steps = 0;
    while idx >= 0 && (idx as usize) < jumps.len() {
        let old_idx = idx;
        idx = idx + jumps[idx as usize];
        if jumps[old_idx as usize] >= 3 {
            jumps[old_idx as usize] = jumps[old_idx as usize] - 1;
        } else {
            jumps[old_idx as usize] = jumps[old_idx as usize] + 1;
        }
        steps = steps + 1;
    }
    println!("steps: {}", steps);
}
