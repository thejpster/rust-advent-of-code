use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct State {
    dir: Dir,
    pos: Position,
    board: HashMap<Position, Infected>,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
enum Infected {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl ::std::ops::Add<Turn> for Dir {
    type Output = Self;
    fn add(self, rhs: Turn) -> Self::Output {
        match rhs {
            Turn::Left => match self {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Up,
            },
            Turn::Right => match self {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            },
        }
    }
}

impl ::std::ops::AddAssign<Turn> for State {
    fn add_assign(&mut self, rhs: Turn) {
        self.dir = self.dir + rhs;
        match self.dir {
            Dir::Up => self.pos.y -= 1,
            Dir::Down => self.pos.y += 1,
            Dir::Left => self.pos.x -= 1,
            Dir::Right => self.pos.x += 1,
        }
    }
}

impl ::std::ops::AddAssign<i32> for State {
    fn add_assign(&mut self, rhs: i32) {
        match self.dir {
            Dir::Up => self.pos.y -= rhs,
            Dir::Down => self.pos.y += rhs,
            Dir::Left => self.pos.x -= rhs,
            Dir::Right => self.pos.x += rhs,
        }
    }
}

impl ::std::fmt::Display for Infected {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Infected::Clean => write!(f, "."),
            Infected::Infected => write!(f, "#"),
            Infected::Weakened => write!(f, "W"),
            Infected::Flagged => write!(f, "F"),
        }
    }
}

impl State {
    fn get(&self) -> Infected {
        *self.board.get(&self.pos).unwrap_or(&Infected::Clean)
    }

    fn set(&mut self, x: Infected) {
        let pos = self.pos;
        self.board.insert(pos, x);
    }
}

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    part1(contents)?;
    part2(contents)?;
    Ok(())
}

fn part2(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut board = HashMap::new();
    for (row, line) in contents[0].iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let pos = Position {
                x: col as i32,
                y: row as i32,
            };
            board.insert(
                pos,
                if ch == '#' {
                    Infected::Infected
                } else {
                    Infected::Clean
                },
            );
        }
    }
    let centre = contents[0].len() / 2;
    let mut state = State {
        board: board,
        // assume symmetry
        pos: Position {
            x: centre as i32,
            y: centre as i32,
        },
        dir: Dir::Up,
    };

    let mut count = 0;
    for _ in 0..10_000_000 {
        match state.get() {
            Infected::Infected => {
                state.set(Infected::Flagged);
                state += Turn::Right;
            }
            Infected::Flagged => {
                state.set(Infected::Clean);
                let new_dir = match state.dir {
                    Dir::Up => Dir::Down,
                    Dir::Down => Dir::Up,
                    Dir::Left => Dir::Right,
                    Dir::Right => Dir::Left,
                };
                state.dir = new_dir;
                state += 1;
            }
            Infected::Weakened => {
                state.set(Infected::Infected);
                state += 1;
                count += 1;
            }
            Infected::Clean => {
                state.set(Infected::Weakened);
                state += Turn::Left;
            }
        }
    }
    println!("Part 2 Count: {}", count);
    Ok(())
}

fn part1(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut board = HashMap::new();
    for (row, line) in contents[0].iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let pos = Position {
                x: col as i32,
                y: row as i32,
            };
            board.insert(
                pos,
                if ch == '#' {
                    Infected::Infected
                } else {
                    Infected::Clean
                },
            );
        }
    }
    // assume symmetry
    let centre = contents[0].len() / 2;
    let mut state = State {
        board: board,
        pos: Position {
            x: centre as i32,
            y: centre as i32,
        },
        dir: Dir::Up,
    };

    let mut count = 0;
    for _ in 0..10_000 {
        if state.get() == Infected::Infected {
            state.set(Infected::Clean);
            state += Turn::Right;
        } else {
            state.set(Infected::Infected);
            state += Turn::Left;
            count += 1;
        }
    }
    println!("Part 1 Count: {}", count);
    Ok(())
}
