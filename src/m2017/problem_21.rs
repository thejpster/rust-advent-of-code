#![allow(dead_code)]

use failure::Error;
use std::collections::HashMap;

type Pixel = bool;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct TwoSquare {
    pixels: [[Pixel; 2]; 2],
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct ThreeSquare {
    pixels: [[Pixel; 3]; 3],
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct FourSquare {
    pixels: [[Pixel; 3]; 3],
}

struct Image {
    pixels: Vec<Pixel>,
    size: usize,
}

use std::fmt;
impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in (&self.pixels).chunks(self.size) {
            for pixel in row {
                write!(f, "{}", if *pixel { '#' } else { '.' })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Image {
    fn decompose_two(&self) -> Option<Vec<TwoSquare>> {
        unimplemented!();
    }

    fn decompose_three(&self) -> Option<Vec<ThreeSquare>> {
        unimplemented!();
    }

    fn create_from_threes(_squares: &[ThreeSquare]) -> Image {
        unimplemented!();
    }

    fn create_from_fours(_squares: &[FourSquare]) -> Image {
        unimplemented!();
    }

    fn count_pixels(&self) -> usize {
        0
    }
}

impl FourSquare {
	fn new(_pixels: &[bool]) -> FourSquare {
		unimplemented!();
	}

    fn produce_rotations(&self) -> Vec<ThreeSquare> {
        unimplemented!();
    }

    fn flip_x(&self) -> ThreeSquare {
        unimplemented!();
    }

    fn rotate_90_left(&self) -> ThreeSquare {
        unimplemented!();
    }

    fn flip_y(&self) -> ThreeSquare {
        unimplemented!();
    }
}

impl ThreeSquare {
	fn new(_pixels: &[bool]) -> ThreeSquare {
		unimplemented!();
	}

    fn produce_rotations(&self) -> Vec<ThreeSquare> {
        unimplemented!();
    }

    fn flip_x(&self) -> ThreeSquare {
        unimplemented!();
    }

    fn rotate_90_left(&self) -> ThreeSquare {
        unimplemented!();
    }

    fn flip_y(&self) -> ThreeSquare {
        unimplemented!();
    }

    fn map(&self) -> FourSquare {
        unimplemented!();
    }

    fn parse(_lines: &[String]) -> HashMap<ThreeSquare, FourSquare> {
        unimplemented!();
    }
}

impl TwoSquare {
	fn new(_pixels: &[bool]) -> TwoSquare {
		unimplemented!();
	}

    fn produce_rotations(&self) -> Vec<TwoSquare> {
        unimplemented!();
    }

    fn flip_x(&self) -> TwoSquare {
        unimplemented!();
    }

    fn rotate_90_left(&self) -> TwoSquare {
        unimplemented!();
    }

    fn flip_y(&self) -> TwoSquare {
        unimplemented!();
    }

    fn map(&self) -> ThreeSquare {
        unimplemented!();
    }

    fn parse(_lines: &[String]) -> HashMap<TwoSquare, ThreeSquare> {
        unimplemented!();
    }
}

pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let two_rules = TwoSquare::parse(&contents[0]);
    let three_rules = ThreeSquare::parse(&contents[0]);

    let mut image = Image {
        size: 3,
        pixels: ".#...####".chars().map(|x| x == '#').collect(),
    };
    println!("Image:\n{:?}", image);
    println!("Pixels: {}", image.count_pixels());
    for _ in 0..5 {
        if let Some(sub) = image.decompose_three() {
            let squares: Vec<FourSquare> = sub.iter().map(|x| three_rules[x]).collect();
            image = Image::create_from_fours(&squares);
        } else if let Some(sub) = image.decompose_two() {
            let squares: Vec<ThreeSquare> = sub.iter().map(|x| two_rules[x]).collect();
            image = Image::create_from_threes(&squares);
        }
        println!("Image:\n{:?}", image);
        println!("Pixels: {}", image.count_pixels());
    }

    Ok(())
}
