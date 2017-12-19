#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Piece {
    Cross,
    LeftRight,
    UpDown,
    Space,
    Letter(char),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Location {
    row: usize,
    col: usize,
}

impl ::std::ops::Add<Direction> for Location {
    type Output = Location;

    fn add(self, rhs: Direction) -> Location {
        match rhs {
            Direction::Up => Location {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Location {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Location {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Location {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

impl ::std::fmt::Display for Piece {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Piece::UpDown => write!(f, "|"),
            Piece::LeftRight => write!(f, "-"),
            Piece::Cross => write!(f, "+"),
            Piece::Space => write!(f, " "),
            Piece::Letter(ch) => write!(f, "{}", ch),
        }
    }
}

pub fn run(contents: &Vec<Vec<String>>) {
    let mut map: Vec<Vec<Piece>> = Vec::new();
    for line in &contents[0] {
        let mut line_vec = Vec::new();
        for square in line.chars() {
            match square {
                '|' => line_vec.push(Piece::UpDown),
                ' ' => line_vec.push(Piece::Space),
                '+' => line_vec.push(Piece::Cross),
                '-' => line_vec.push(Piece::LeftRight),
                'A'...'Z' => line_vec.push(Piece::Letter(square)),
                _ => panic!("Bad char {}", square),
            }
        }
        map.push(line_vec);
    }

    for line in &map {
        println!(
            "{}",
            line.iter().map(|x| format!("{}", x)).collect::<String>()
        );
    }

    let mut location = Location {
        row: 0,
        col: map[0].iter().position(|&sq| sq == Piece::UpDown).unwrap(),
    };
    let mut letters = String::new();
    let mut steps = 0;
    let mut dir = Direction::Down;
    loop {
        if let Piece::Letter(ch) = map[location.row][location.col] {
            letters.push(ch);
        }
        // println!("At {:?} = {}", location, map[location.row][location.col]);
        if let Some(d) = calc_new_dir(&map, &location, dir) {
            dir = d;
            steps = steps + 1;
            location = location + dir;
        } else {
            break;
        }
    }
    println!("Letters: {}", letters);
    println!("Steps: {}", steps);
}

/// Which way next?
fn calc_new_dir(map: &Vec<Vec<Piece>>, location: &Location, dir: Direction) -> Option<Direction> {
    match map[location.row][location.col] {
        Piece::Space => None,
        Piece::Cross => {
            // If you walk up/down into a cross, you want the left/right
            // if you walk left/right into a cross, you want the up/down
            match dir {
                Direction::Up | Direction::Down => {
                    if Piece::LeftRight == map[location.row][location.col + 1] {
                        Some(Direction::Right)
                    } else if let Piece::Letter(_) = map[location.row][location.col + 1] {
                        Some(Direction::Right)
                    } else if Piece::LeftRight == map[location.row][location.col - 1] {
                        Some(Direction::Left)
                    } else if let Piece::Letter(_) = map[location.row][location.col - 1] {
                        Some(Direction::Left)
                    } else {
                        panic!("Am lost!");
                    }
                }
                Direction::Left | Direction::Right => {
                    if Piece::UpDown == map[location.row - 1][location.col] {
                        Some(Direction::Up)
                    } else if let Piece::Letter(_) = map[location.row - 1][location.col] {
                        Some(Direction::Up)
                    } else if Piece::UpDown == map[location.row + 1][location.col] {
                        Some(Direction::Down)
                    } else if let Piece::Letter(_) = map[location.row + 1][location.col] {
                        Some(Direction::Down)
                    } else {
                        panic!("Am lost!");
                    }
                }
            }
        }
        _ => {
            Some(dir) // keep going
        }
    }
}
