use std::collections::{HashMap, HashSet};

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let coord: Vec<(i32, i32)> = contents[0]
        .iter()
        .map(|x| {
            let mut i = x.split(",").map(|y| y.trim().parse().unwrap());
            (i.next().unwrap(), i.next().unwrap())
        }).collect();

    let mut map: HashMap<(i32, i32), usize> = HashMap::new();
    let mut counts: HashMap<usize, usize> = HashMap::new();

    for x in -1000..=1000 {
        for y in -1000..=1000 {
            let here = (x, y);
            let mut diffs = Vec::new();
            for (idx, (cx, cy)) in coord.iter().enumerate() {
                let this_delta = diff(here, (*cx, *cy));
                diffs.push((idx, this_delta));
                // println!("Delta from {:?} to {:?} is {:?}", (x, y), (cx, cy), this_delta);
            }
            // println!("Unsorted diffs: {:?}", diffs);
            diffs.sort_by_key(|x| x.1);
            // println!("Sorted diffs: {:?}", diffs);
            if diffs[0].1 == diffs[1].1 {
                // Duplicate - do nothing
                // println!("Space {:?} is tied", (x, y));
                map.insert(here, 0xFFFFFFFF);
            } else {
                // println!("Space {:?} is closest to {:?}", (x, y), diffs[0]);
                map.insert(here, diffs[0].0);
                *counts.entry(diffs[0].0).or_insert(0) += 1;
            }
        }
    }

    // for y in -1000..=1000 {
    //     for x in -1000..=1000 {
    //         print!("{}", match map.get(&(x,y)).unwrap() {
    //             0xFFFFFFFF => '.',
    //             s => (65 + *s as u8) as char,
    //         });
    //     }
    //     println!();
    // }

    // Now find the co-ordinates that are found at the edge of the map
    let mut infinite_indices = HashSet::new();
    for x in -1000..=1000 {
        for y in &[-1000, 1000] {
            infinite_indices.insert(map.get(&(x, *y)));
        }
    }

    for x in &[-1000, 1000] {
        for y in -1000..=1000 {
            infinite_indices.insert(map.get(&(*x, y)));
        }
    }

    println!("Infinite indexes: {:?}", infinite_indices);

    // Find the max count for an index that isn't found on the edge
    println!(
        "Part 1 is {:?}",
        counts
            .iter()
            .filter(|x| !infinite_indices.contains(&Some(x.0)))
            .map(|x| x.1)
            .max()
    );

    let mut region_size = 0;
    for x in -1000..=1000 {
        for y in -1000..=1000 {
            let here = (x, y);
            let mut total = 0;
            for (cx, cy) in coord.iter() {
                let this_delta = diff(here, (*cx, *cy));
                total += this_delta;
                // println!("Delta from {:?} to {:?} is {:?}", (x, y), (cx, cy), this_delta);
            }
            if total < 10_000 {
                region_size += 1;
            }
        }
    }

    println!("Region size: {}", region_size);

    Ok(())
}

fn diff(a: (i32, i32), b: (i32, i32)) -> i32 {
    let delta_x = a.0 - b.0;
    let delta_y = a.1 - b.1;
    delta_x.abs() + delta_y.abs()
}
