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

#[derive(Debug, Clone, Copy)]
struct State(Dir, i32, i32);

pub fn run(contents: &Vec<Vec<String>>) {
    // Big one line string
    let first_line = &contents[0][0];
    let mut state = State(Dir::Up, 0, 0);
    let turns = first_line.split_whitespace().map(|x| {
        // println!("Got {}", x);
        let val = &x[1..].replace(",", "");
        match &x[0..1] {
            "R" => Turn::Right(val.parse().unwrap()),
            "L" => Turn::Left(val.parse().unwrap()),
            _ => panic!("Bad move {}", x),
        }
    });
    for turn in turns {
        state = state + turn
    }
    println!("At {:?}", state);
    println!("Distance: {}", state.1.abs() + state.2.abs())
}

impl ::std::ops::Add<Turn> for Dir {
    type Output = Self;
    fn add(self, rhs: Turn) -> Self::Output {
        match rhs {
            Turn::Left(_) => {
                match self {
                    Dir::Up => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Down => Dir::Right,
                    Dir::Right => Dir::Up,
                }
            }
            Turn::Right(_) => {
                match self {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                }
            }
        }
    }
}

impl ::std::ops::Add<Turn> for State {
    type Output = Self;
    fn add(mut self, rhs: Turn) -> Self::Output {
        self.0 = self.0 + rhs;
        let distance = match rhs {
            Turn::Left(x) => x,
            Turn::Right(x) => x,
        };
        match self.0 {
            Dir::Up => self.1 = self.1 + distance,
            Dir::Down => self.1 = self.1 - distance,
            Dir::Left => self.2 = self.2 - distance,
            Dir::Right => self.2 = self.2 + distance,
        }
        self
    }
}
