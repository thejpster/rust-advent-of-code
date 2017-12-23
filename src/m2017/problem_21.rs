#![allow(dead_code)]

use failure::Error;
use std::collections::HashMap;

type Pixel = bool;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct TwoSquare {
    pixels: [[Pixel; 2]; 2],
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct ThreeSquare {
    pixels: [[Pixel; 3]; 3],
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
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
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for TwoSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.pixels {
            for pixel in row {
                write!(f, "{}", if *pixel { '#' } else { '.' })?;
            }
            write!(f, "/")?;
        }
        Ok(())
    }
}

impl fmt::Debug for ThreeSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.pixels {
            for pixel in row {
                write!(f, "{}", if *pixel { '#' } else { '.' })?;
            }
            write!(f, "/")?;
        }
        Ok(())
    }
}

impl fmt::Debug for FourSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.pixels {
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
            let mut result = vec![];
            let x_count = self.size / 2;
            let y_count = x_count;
            for y in 0..y_count {
                for x in 0..x_count {
                    let offset = (y * self.size * 2) + (x * 2);
                    let offset2 = offset + self.size;
                    let sq = TwoSquare::new(&[
                        self.pixels[offset],
                        self.pixels[offset + 1],
                        self.pixels[offset2],
                        self.pixels[offset2 + 1],
                    ]);
                    result.push(sq);
                }
            }
            Some(result)
        } else {
            None
        }
    }

    fn decompose_three(&self) -> Option<Vec<ThreeSquare>> {
        if (self.size % 3) == 0 {
            let mut result = vec![];
            let x_count = self.size / 3;
            let y_count = x_count;
            for y in 0..y_count {
                for x in 0..x_count {
                    let offset = (y * self.size * 3) + (x * 3);
                    let offset2 = offset + self.size;
                    let offset3 = offset2 + self.size;
                    let sq = ThreeSquare::new(&[
                        self.pixels[offset],
                        self.pixels[offset + 1],
                        self.pixels[offset + 2],
                        self.pixels[offset2],
                        self.pixels[offset2 + 1],
                        self.pixels[offset2 + 2],
                        self.pixels[offset3],
                        self.pixels[offset3 + 1],
                        self.pixels[offset3 + 2],
                    ]);
                    result.push(sq);
                }
            }
            Some(result)
        } else {
            None
        }
    }

    fn create_from_threes(squares: &[ThreeSquare]) -> Image {
        let mut im = Image {
            pixels: Vec::new(),
            size: (squares.len() as f32 * 9.0).sqrt() as usize,
        };
        for input_line in squares.chunks(im.size / 3) {
            for in_row in 0..3 {
                for line in input_line.iter() {
                    for in_col in 0..3 {
                        im.pixels.push(line.pixels[in_row][in_col]);
                    }
                }
            }
        }
        im
    }

    fn create_from_fours(squares: &[FourSquare]) -> Image {
        let mut im = Image {
            pixels: Vec::new(),
            size: (squares.len() as f32 * 16.0).sqrt() as usize,
        };
        for input_line in squares.chunks(im.size / 4) {
            for in_row in 0..4 {
                for line in input_line.iter() {
                    for in_col in 0..4 {
                        im.pixels.push(line.pixels[in_row][in_col]);
                    }
                }
            }
        }
        im
    }

    fn count_pixels(&self) -> usize {
        self.pixels.iter().filter(|x| **x).count()
    }
}

impl FourSquare {
    fn new(pixels: &[bool]) -> FourSquare {
        assert_eq!(pixels.len(), 16);
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
        assert_eq!(pixels.len(), 9);
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
            self.rotate_90_left(),
            self.rotate_90_left().rotate_90_left(),
            self.rotate_90_left().rotate_90_left().rotate_90_left(),
            self.flip_x(),
            self.flip_x().rotate_90_left(),
            self.flip_x().rotate_90_left().rotate_90_left(),
            self.flip_x()
                .rotate_90_left()
                .rotate_90_left()
                .rotate_90_left(),
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
                for i in &input.produce_rotations() {
                    result.insert(*i, output);
                }
            }
        }
        result
    }
}

impl TwoSquare {
    fn new(pixels: &[bool]) -> TwoSquare {
        assert_eq!(pixels.len(), 4);
        TwoSquare {
            pixels: [[pixels[0], pixels[1]], [pixels[2], pixels[3]]],
        }
    }

    fn produce_rotations(&self) -> Vec<TwoSquare> {
        vec![
            *self,
            self.rotate_90_left(),
            self.rotate_90_left().rotate_90_left(),
            self.rotate_90_left().rotate_90_left().rotate_90_left(),
            self.flip_x(),
            self.flip_x().rotate_90_left(),
            self.flip_x().rotate_90_left().rotate_90_left(),
            self.flip_x()
                .rotate_90_left()
                .rotate_90_left()
                .rotate_90_left(),
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
                for i in &input.produce_rotations() {
                    result.insert(*i, output);
                }
            }
        }
        result
    }
}

pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    runi(contents, 5)?;
    runi(contents, 18)?;
    Ok(())
}

pub fn runi(contents: &[Vec<String>], iters: usize) -> Result<(), Error> {
    let two_rules = TwoSquare::parse(&contents[0]);
    let three_rules = ThreeSquare::parse(&contents[0]);

    let mut image = Image {
        size: 3,
        pixels: ".#...####".chars().map(|x| x == '#').collect(),
    };
    for i in 0..iters {
        println!("Iteration: {}", i);
        if let Some(sub) = image.decompose_two() {
            // for s in &sub {
            // }
            let squares: Vec<ThreeSquare> = sub.iter().map(|x| two_rules[x]).collect();
            image = Image::create_from_threes(&squares);
        } else if let Some(sub) = image.decompose_three() {
            // for s in &sub {
            // }
            let squares: Vec<FourSquare> = sub.iter().map(|x| three_rules[x]).collect();
            image = Image::create_from_fours(&squares);
        } else {
            panic!("Did not decompose!");
        }
        println!("Pixels: {}", image.count_pixels());
    }

    Ok(())
}
