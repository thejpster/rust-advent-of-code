use std::collections::HashMap;

pub fn run(contents: &[Vec<String>]) {
    let mut hs = HashMap::new();
    let mut buckets: Vec<u32> = contents[0][0]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Buckets: {:?}", buckets);

    // Find index with highest number of blocks
    let num_items = buckets.len();
    for i in 1.. {
        let max = *buckets.iter().max().unwrap();
        let idx = buckets.iter().position(|x| *x == max).unwrap();
        // println!("*****\nBuckets: {:?}", buckets);
        // println!("Idx: {} = {}", idx, buckets[idx]);
        let mut item = buckets[idx];
        buckets[idx] = 0;
        let mut inner_idx = idx;
        while item > 0 {
            inner_idx = (inner_idx + 1) % num_items;
            buckets[inner_idx] += 1;
            item -= 1;
        }
        if let Some(x) = hs.get(&buckets) {
            println!("Cycles: {} .. {} ({})", i, x, i - x);
            break;
        }
        hs.insert(buckets.clone(), i);
    }
}
