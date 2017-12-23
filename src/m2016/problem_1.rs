#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct State {
    dir: Dir,
    pos: Position,
}

use failure::Error;
use failure;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    // Big one line string
    let first_line = &contents[0][0];
    let mut state = State {
        dir: Dir::Up,
        pos: Position::default(),
    };
    let turns: Vec<Turn> = first_line
        .split_whitespace()
        .map(|x| {
            // println!("Got {}", x);
            match x[1..].replace(",", "").parse() {
                Ok(val) => match &x[0..1] {
                    "R" => Ok(Turn::Right(val)),
                    "L" => Ok(Turn::Left(val)),
                    _ => Err(format_err!("Bad move {}", x)),
                },
                Err(e) => Err(e.into()),
            }
        })
        .collect::<Result<_, _>>()?;
    let mut visit = vec![];
    for turn in turns {
        // println!("Turn: {:?}", turn);
        let old_state = state;
        state += turn;
        visit.extend(steps(&old_state.pos, &state.pos));
    }
    println!("At {:?}", state);
    println!("Distance: {}", state.distance());

    for i in 0..visit.len() {
        for j in i + 1..visit.len() {
            if visit[i] == visit[j] {
                println!("I ({}) {:?}", i, visit[i]);
                println!("J ({}) {:?}", j, visit[j]);
                println!("Distance: {}", visit[i].distance());
                return Ok(());
            }
        }
    }
    Err(failure::err_msg("Shouldn't get here..."))
}

fn steps(pos1: &Position, pos2: &Position) -> Vec<Position> {
    if pos1.x == pos2.x {
        // walking along y
        if pos1.y < pos2.y {
            // walking forwards
            (pos1.y..pos2.y)
                .map(|y| Position { x: pos1.x, y })
                .collect()
        } else {
            // walking backwards
            let mut v = (pos2.y + 1..pos1.y + 1)
                .map(|y| Position { x: pos1.x, y })
                .collect::<Vec<_>>();
            v.reverse();
            v
        }
    } else if pos1.x < pos2.x {
        // walking forwards
        (pos1.x..pos2.x)
            .map(|x| Position { x, y: pos1.y })
            .collect()
    } else {
        // walking backwards
        let mut v = (pos2.x + 1..pos1.x + 1)
            .map(|x| Position { x, y: pos1.y })
            .collect::<Vec<_>>();
        v.reverse();
        v
    }
}

impl Position {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl State {
    fn distance(&self) -> i32 {
        self.pos.distance()
    }
}

impl ::std::ops::Add<Turn> for Dir {
    type Output = Self;
    fn add(self, rhs: Turn) -> Self::Output {
        match rhs {
            Turn::Left(_) => match self {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Up,
            },
            Turn::Right(_) => match self {
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
        let distance = match rhs {
            Turn::Left(x) | Turn::Right(x) => x,
        };
        match self.dir {
            Dir::Up => self.pos.x += distance,
            Dir::Down => self.pos.x -= distance,
            Dir::Left => self.pos.y -= distance,
            Dir::Right => self.pos.y += distance,
        }
    }
}
