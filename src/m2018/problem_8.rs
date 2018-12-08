use std::collections::HashMap;

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let numbers: Vec<u32> = contents[0][0]
        .split_whitespace()
        .map(|x| x.parse())
        .collect::<Result<_, _>>()?;

    let (used, sum) = part1(&numbers);
    assert_eq!(used, numbers.len());
    println!("Metadata sum : {}", sum);
    let (used, sum) = part2(&numbers);
    assert_eq!(used, numbers.len());
    println!("Root sum : {}", sum);
    Ok(())
}

fn part1(numbers: &[u32]) -> (usize, u32) {
    let num_children = numbers[0];
    let num_metadata = numbers[1] as usize;
    let mut used = 2;
    let mut sum = 0;
    for _child in 0..num_children {
        let (child_used, child_sum) = part1(&numbers[used..]);
        used += child_used;
        sum += child_sum;
    }
    if num_metadata > 0 {
        let sum_metadata: u32 = numbers[used..used + num_metadata].iter().sum();
        sum += sum_metadata;
    }
    used += num_metadata;
    (used, sum)
}

fn part2(numbers: &[u32]) -> (usize, u32) {
    let num_children = numbers[0];
    let num_metadata = numbers[1] as usize;
    println!("Looking at {}, {}", num_children, num_metadata);
    let mut used = 2;
    let mut sum = 0;
    let mut map = HashMap::new();
    for child in 1..=num_children {
        let (child_used, child_sum) = part2(&numbers[used..]);
        used += child_used;
        map.insert(child, child_sum);
    }
    if num_children == 0 {
        // If a node has no child nodes, its value is the sum of its metadata
        // entries.
        let sum_metadata: u32 = numbers[used..used + num_metadata].iter().sum();
        sum += sum_metadata;
    } else {
        // However, if a node does have child nodes, the metadata entries
        // become indexes which refer to those child nodes. A metadata entry
        // of 1 refers to the first child node, 2 to the second, 3 to the
        // third, and so on. The value of this node is the sum of the values
        // of the child nodes referenced by the metadata entries.
        for meta in 0..num_metadata {
            let meta_idx = numbers[used + meta];
            let this_sum = map.get(&meta_idx).cloned().unwrap_or(0);
            sum += this_sum;
        }
    }

    used += num_metadata;
    println!(
        "{}, {} returns {}, {}",
        num_children, num_metadata, used, sum
    );
    (used, sum)
}
