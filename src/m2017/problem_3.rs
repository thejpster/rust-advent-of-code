/// Spirals
///
/// 1
///
/// 5 4 3
/// 6 1 2
/// 7 8 9
///
/// 17 16 15 14 13
/// 18  5  4  3 12
/// 19  6  1  2 11
/// 20  7  8  9 10
//  21 22 23 24 25
///
/// For any n x n grid (for some odd n), the n+2 x n+2 grid starts with (n.n)
/// +1 and goes to (n+2).(n+2).
///
/// We need a 607 x 607 square.
///
/// 3          5                  7
/// 11222222   1333444444444444   155555666666666666666666
/// RULLDDRR | RUUULLLLDDDDRRRR | RUUUUULLLLLLDDDDDDRRRRRR
///
/// For square size N: 1 Right, N-2 Up, N-1 Left, N-1 Down, N-1 Right
/// Total steps = 4(N-1).
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct State {
    position: Position,
    num: usize,
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl ::std::fmt::Debug for Position {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn generate_path(edge_len: usize) -> Vec<Move> {
    let mut result = Vec::new();
    result.push(Move::Right);
    for _ in 1..edge_len - 1 {
        result.push(Move::Up);
    }
    for _ in 1..edge_len {
        result.push(Move::Left);
    }
    for _ in 1..edge_len {
        result.push(Move::Down);
    }
    for _ in 1..edge_len {
        result.push(Move::Right);
    }
    result
}

impl ::std::ops::Add<Move> for Position {
    type Output = Position;
    fn add(self, movement: Move) -> Position {
        match movement {
            Move::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Move::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Move::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Move::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl State {
    fn new() -> State {
        let s = State {
            position: Position { x: 0, y: 0 },
            num: 1,
        };
        s
    }

    fn go(&mut self, movement: Move) {
        self.num = self.num + 1;
        self.position = self.position + movement;
        if self.num == 368078 {
            println!(
                "{:?}, distance: {}",
                self,
                self.position.x.abs() + self.position.y.abs()
            );
            return;
        }
    }
}

pub fn run(_contents: &Vec<Vec<String>>) {
    let mut state = State::new();
    for i in 1..304 {
        let edge_len = (i * 2) + 1;
        let path = generate_path(edge_len);
        // println!("Edge: {} => Path: {:?}", edge_len, path);
        for step in path {
            state.go(step);
            // println!("State: {:?}", state);
        }
    }
}
