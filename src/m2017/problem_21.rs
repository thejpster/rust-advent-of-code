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
    pixels: [[Pixel; 4]; 4],
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
        if (self.size % 2) == 0 {
            let result = vec![];
            unimplemented!();
            Some(result)
        } else {
            None
        }
    }

    fn decompose_three(&self) -> Option<Vec<ThreeSquare>> {
        if (self.size % 3) == 0 {
            let result = vec![];
            unimplemented!();
            Some(result)
        } else {
            None
        }
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
    fn new(pixels: &[bool]) -> FourSquare {
        FourSquare {
            pixels: [
                [pixels[0], pixels[1], pixels[2], pixels[3]],
                [pixels[4], pixels[5], pixels[6], pixels[7]],
                [pixels[8], pixels[9], pixels[10], pixels[11]],
                [pixels[12], pixels[13], pixels[14], pixels[15]],
            ],
        }
    }
}

impl ThreeSquare {
    fn new(pixels: &[bool]) -> ThreeSquare {
        ThreeSquare {
            pixels: [
                [pixels[0], pixels[1], pixels[2]],
                [pixels[3], pixels[4], pixels[5]],
                [pixels[6], pixels[7], pixels[8]],
            ],
        }
    }

    fn produce_rotations(&self) -> Vec<ThreeSquare> {
        vec![
            *self,
            self.flip_x(),
            self.flip_y(),
            self.rotate_90_left(),
            self.rotate_90_left().rotate_90_left(),
            self.rotate_90_left().rotate_90_left().rotate_90_left(),
        ]
    }

    fn flip_x(&self) -> ThreeSquare {
        ThreeSquare {
            pixels: [
                [self.pixels[0][2], self.pixels[0][1], self.pixels[0][0]],
                [self.pixels[1][2], self.pixels[1][1], self.pixels[1][0]],
                [self.pixels[2][2], self.pixels[2][1], self.pixels[2][0]],
            ],
        }
    }

    fn rotate_90_left(&self) -> ThreeSquare {
        ThreeSquare {
            pixels: [
                [self.pixels[2][0], self.pixels[1][0], self.pixels[0][0]],
                [self.pixels[2][1], self.pixels[1][1], self.pixels[0][1]],
                [self.pixels[2][2], self.pixels[1][2], self.pixels[0][2]],
            ],
        }
    }

    fn flip_y(&self) -> ThreeSquare {
        ThreeSquare {
            pixels: [
                [self.pixels[2][0], self.pixels[2][1], self.pixels[2][2]],
                [self.pixels[1][0], self.pixels[1][1], self.pixels[1][2]],
                [self.pixels[0][0], self.pixels[0][1], self.pixels[0][2]],
            ],
        }
    }

    fn parse(lines: &[String]) -> HashMap<ThreeSquare, FourSquare> {
        let mut result = HashMap::new();
        for line in lines {
            let parts: Vec<&str> = line.split(" => ").collect();
            if parts[0].len() == 11 {
                let input = parts[0]
                    .chars()
                    .filter(|ch| *ch != '/')
                    .map(|ch| ch == '#')
                    .collect::<Vec<bool>>();
                let input = ThreeSquare::new(&input);
                let output = parts[1]
                    .chars()
                    .filter(|ch| *ch != '/')
                    .map(|ch| ch == '#')
                    .collect::<Vec<bool>>();
                let output = FourSquare::new(&output);
                for i in input.produce_rotations().iter() {
                    result.insert(*i, output);
                }
            }
        }
        result
    }
}

impl TwoSquare {
    fn new(pixels: &[bool]) -> TwoSquare {
        TwoSquare {
            pixels: [[pixels[0], pixels[1]], [pixels[2], pixels[3]]],
        }
    }

    fn produce_rotations(&self) -> Vec<TwoSquare> {
        vec![
            *self,
            self.flip_x(),
            self.flip_y(),
            self.rotate_90_left(),
            self.rotate_90_left().rotate_90_left(),
            self.rotate_90_left().rotate_90_left().rotate_90_left(),
        ]
    }

    fn flip_x(&self) -> TwoSquare {
        TwoSquare {
            pixels: [
                [self.pixels[0][1], self.pixels[0][0]],
                [self.pixels[1][1], self.pixels[1][0]],
            ],
        }
    }

    fn rotate_90_left(&self) -> TwoSquare {
        TwoSquare {
            pixels: [
                [self.pixels[1][0], self.pixels[0][0]],
                [self.pixels[1][1], self.pixels[0][1]],
            ],
        }
    }

    fn flip_y(&self) -> TwoSquare {
        TwoSquare {
            pixels: [
                [self.pixels[1][0], self.pixels[1][1]],
                [self.pixels[0][0], self.pixels[0][1]],
            ],
        }
    }

    fn parse(lines: &[String]) -> HashMap<TwoSquare, ThreeSquare> {
        let mut result = HashMap::new();
        for line in lines {
            let parts: Vec<&str> = line.split(" => ").collect();
            if parts[0].len() == 5 {
                let input = parts[0]
                    .chars()
                    .filter(|ch| *ch != '/')
                    .map(|ch| ch == '#')
                    .collect::<Vec<bool>>();
                let input = TwoSquare::new(&input);
                let output = parts[1]
                    .chars()
                    .filter(|ch| *ch != '/')
                    .map(|ch| ch == '#')
                    .collect::<Vec<bool>>();
                let output = ThreeSquare::new(&output);
                for i in input.produce_rotations().iter() {
                    result.insert(*i, output);
                }
            }
        }
        result
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
