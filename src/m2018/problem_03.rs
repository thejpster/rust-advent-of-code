//! Day 3

use std::collections::HashMap;

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let squares: Vec<Square> = contents[0].iter().map(|line| parse(line)).collect();
    println!("{:#?}", squares);
    let mut map = HashMap::new();
    for sq in squares.iter() {
        for inch in sq.iter() {
            *map.entry(inch).or_insert(0) += 1;
        }
    }

    let mut total = 0;
    for count in map.values().cloned() {
        if count >= 2 {
            total += 1;
        }
    }

    println!("Total: {}", total);

    for sq in squares.iter() {
        let mut bad = false;
        for inch in sq.iter() {
            if map[&inch] != 1 {
                bad = true;
                break;
            }
        }
        if bad {
            for inch in sq.iter() {
                map.insert(inch, 0);
            }
        }
    }

    for (k, v) in map.iter() {
        if *v == 1 {
            println!("No overlap in {:?}", k);
            for sq in squares.iter() {
                for inch in sq.iter() {
                    if *k == inch {
                        println!("No overlap in {:?} in {:?}", inch, sq);
                        println!("ID: {}", sq.id);
                        return Ok(());
                    }
                }
            }
        }
    }
    Err(failure::err_msg("Oh no"))
}

impl Iterator for SquareIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.square.height {
            return None;
        }
        let result = (self.x + self.square.left, self.y + self.square.top);
        self.x += 1;
        if self.x == self.square.width {
            self.x = 0;
            self.y += 1;
        }
        Some(result)
    }
}

impl Square {
    fn iter(&self) -> SquareIter {
        SquareIter {
            square: self.clone(),
            x: 0,
            y: 0,
        }
    }
}

struct SquareIter {
    square: Square,
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Square {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn parse(line: &str) -> Square {
    let mut parts = line.split_whitespace();
    let id: u32 = parts.next().unwrap()[1..].parse().unwrap();
    parts.next();
    let mut left_top = parts.next().unwrap().split(",");
    let left: u32 = left_top.next().unwrap().parse().unwrap();
    let top: u32 = left_top
        .next()
        .unwrap()
        .trim_end_matches(':')
        .parse()
        .unwrap();
    let mut width_height = parts.next().unwrap().split("x");
    let width: u32 = width_height.next().unwrap().parse().unwrap();
    let height: u32 = width_height.next().unwrap().parse().unwrap();
    Square {
        id,
        left,
        top,
        width,
        height,
    }
}
