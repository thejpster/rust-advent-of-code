use colored::*;
use failure::Error;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Track {
    Horizontal,
    Vertical,
    SouthWest,
    SouthEast,
    Intersection,
    Space,
}

#[derive(Clone, Copy)]
enum CartDir {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
struct Cart {
    intersections: usize,
    current_dir: CartDir,
    cycles: usize,
}

impl std::fmt::Debug for CartDir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            CartDir::North => "^",
            CartDir::South => "v",
            CartDir::West => "<",
            CartDir::East => ">",
        };
        write!(f, "{}", s.green().bold())
    }
}

impl std::fmt::Debug for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Track::Horizontal => write!(f, "-"),
            Track::Vertical => write!(f, "|"),
            Track::SouthEast => write!(f, "/"),
            Track::SouthWest => write!(f, "\\"),
            Track::Intersection => write!(f, "+"),
            Track::Space => write!(f, " "),
        }
    }
}

pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut track = Vec::new();
    let mut carts = HashMap::new();
    for (y, input_line) in contents[0].iter().enumerate() {
        let mut line = Vec::new();
        for (x, c) in input_line.chars().enumerate() {
            match c {
                '/' => line.push(Track::SouthEast),
                '\\' => line.push(Track::SouthWest),
                '|' => line.push(Track::Vertical),
                '-' => line.push(Track::Horizontal),
                ' ' => line.push(Track::Space),
                '+' => line.push(Track::Intersection),
                '>' => {
                    line.push(Track::Horizontal);
                    let c = Cart {
                        intersections: 0,
                        cycles: 0,
                        current_dir: CartDir::East,
                    };
                    carts.insert((x, y), c);
                }
                '<' => {
                    line.push(Track::Horizontal);
                    let c = Cart {
                        intersections: 0,
                        cycles: 0,
                        current_dir: CartDir::West,
                    };
                    carts.insert((x, y), c);
                }
                '^' => {
                    line.push(Track::Vertical);
                    let c = Cart {
                        intersections: 0,
                        cycles: 0,
                        current_dir: CartDir::North,
                    };
                    carts.insert((x, y), c);
                }
                'v' => {
                    line.push(Track::Vertical);
                    let c = Cart {
                        intersections: 0,
                        cycles: 0,
                        current_dir: CartDir::South,
                    };
                    carts.insert((x, y), c);
                }
                _ => panic!("Unknown track {}", c),
            }
        }
        track.push(line);
    }

    for cycle in 1.. {
        render_track(&track, &carts);
        move_carts(&track, &mut carts, cycle);
        if carts.len() == 1 {
            println!("Only one cart left! {:?}", carts);
            break;
        }
    }
    Ok(())
}

fn move_carts(track: &Vec<Vec<Track>>, carts: &mut HashMap<(usize, usize), Cart>, cycle: usize) {
    for (y, line) in track.iter().enumerate() {
        for (x, _piece) in line.iter().enumerate() {
            if let Some(mut c) = carts.remove(&(x, y)) {
                if c.cycles == cycle {
                    // Moved this one already
                    carts.insert((x, y), c);
                    continue;
                }
                let mut new_x = x;
                let mut new_y = y;
                c.cycles = cycle;
                // Move based on the way the cart is facing
                match c.current_dir {
                    CartDir::West => new_x -= 1,
                    CartDir::East => new_x += 1,
                    CartDir::North => new_y -= 1,
                    CartDir::South => new_y += 1,
                }
                // Detect collisions
                if let Some(_c) = carts.remove(&(new_x, new_y)) {
                    // panic!("Crash at {}, {}", new_x, new_y);
                    // Leave both cars removed
                    continue;
                }
                // Turn based on the place we land
                let track_piece = track[new_y][new_x];
                // println!("{}, {} moved to {}, {} which is {:?}", x, y, new_x, new_y, track_piece);
                match track_piece {
                    Track::Space => panic!("Cart {:?} on a space?", c),
                    Track::Horizontal => {
                        // Do nothing
                    }
                    Track::Vertical => {
                        // Do nothing
                    }
                    Track::SouthWest => {
                        c.current_dir = match c.current_dir {
                            CartDir::North => CartDir::West,
                            CartDir::East => CartDir::South,
                            CartDir::South => CartDir::East,
                            CartDir::West => CartDir::North,
                        };
                    }
                    Track::SouthEast => {
                        c.current_dir = match c.current_dir {
                            CartDir::North => CartDir::East,
                            CartDir::East => CartDir::North,
                            CartDir::South => CartDir::West,
                            CartDir::West => CartDir::South,
                        };
                    }
                    Track::Intersection => {
                        match c.intersections {
                            0 => {
                                // Go Left
                                c.current_dir = match c.current_dir {
                                    CartDir::North => CartDir::West,
                                    CartDir::West => CartDir::South,
                                    CartDir::South => CartDir::East,
                                    CartDir::East => CartDir::North,
                                };
                            }
                            1 => {
                                // Go straight
                            }
                            2 => {
                                // Go Right
                                c.current_dir = match c.current_dir {
                                    CartDir::North => CartDir::East,
                                    CartDir::East => CartDir::South,
                                    CartDir::South => CartDir::West,
                                    CartDir::West => CartDir::North,
                                };
                            }
                            _ => panic!("Bad modulo"),
                        }
                        c.intersections += 1;
                        if c.intersections == 3 {
                            c.intersections = 0;
                        }
                    }
                }
                // println!("Moved {:?} to {}, {}", c, new_x, new_y);
                carts.insert((new_x, new_y), c);
                // println!("Carts is now {:?}", carts);
            }
        }
    }
}

fn render_track(track: &Vec<Vec<Track>>, carts: &HashMap<(usize, usize), Cart>) {
    println!("Carts: {:?}", carts);
    // print!("\u{001B}[2J");
    // for (y, line) in track.iter().enumerate() {
    // 	for (x, col) in line.iter().enumerate() {
    // 		if let Some(c) = carts.get(&(x,y)) {
    // 			print!("{:?}", c.current_dir);
    // 		} else {
    // 			print!("{:?}", col);
    // 		}
    // 	}
    // 	println!();
    // }
}
