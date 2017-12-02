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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct State(Dir, Position);

pub fn run(contents: &Vec<Vec<String>>) {
    // Big one line string
    let first_line = &contents[0][0];
    let mut state = State(Dir::Up, Position { x: 0, y: 0 });
    let turns = first_line.split_whitespace().map(|x| {
        // println!("Got {}", x);
        let val = &x[1..].replace(",", "");
        match &x[0..1] {
            "R" => Turn::Right(val.parse().unwrap()),
            "L" => Turn::Left(val.parse().unwrap()),
            _ => panic!("Bad move {}", x),
        }
    });
    let mut visit = vec! [ ];
    for turn in turns {
        // println!("Turn: {:?}", turn);
        let old_state = state.clone();
        state = state + turn;
        visit.extend(steps(&old_state.1, &state.1));
    }
    println!("At {:?}", state);
    println!("Distance: {}", state.distance());

    for i in 0..visit.len() {
        for j in i + 1..visit.len() {
            if visit[i] == visit[j] {
                println!("I ({}) {:?}", i, visit[i]);
                println!("J ({}) {:?}", j, visit[j]);
                println!("Distance: {}", visit[i].distance());
                return;
            }
        }
    }
}

fn steps(pos1: &Position, pos2: &Position) -> Vec<Position> {
    let result = if pos1.x == pos2.x {
        // walking along y
        if pos1.y < pos2.y {
            // walking forwards
            (pos1.y .. pos2.y).map(|y| Position { x: pos1.x, y }).collect()
        } else {
            // walking backwards
            let mut v = (pos2.y + 1 .. pos1.y + 1).map(|y| Position { x: pos1.x, y }).collect::<Vec<_>>();
            v.reverse();
            v
        }

    } else {
        // walking along x
        if pos1.x < pos2.x {
            // walking forwards
            (pos1.x .. pos2.x).map(|x| Position { x, y: pos1.y }).collect()
        } else {
            // walking backwards
            let mut v = (pos2.x + 1 .. pos1.x + 1).map(|x| Position { x, y: pos1.y }).collect::<Vec<_>>();
            v.reverse();
            v
        }
    };
    result
}

impl Position {
    fn distance(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

impl State {
    fn distance(&self) -> i32 {
        return self.1.distance();
    }
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
            Dir::Up => self.1.x = self.1.x + distance,
            Dir::Down => self.1.x = self.1.x - distance,
            Dir::Left => self.1.y = self.1.y - distance,
            Dir::Right => self.1.y = self.1.y + distance,
        }
        self
    }
}
