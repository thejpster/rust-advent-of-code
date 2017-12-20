use std::ops::Add;
use std::cmp::max;
use std::default::Default;

#[derive(Debug, Default)]
struct Pos(i64, i64);

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Pos {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    play("ne,ne,ne");
    play("ne,ne,sw,sw");
    play("ne,ne,s,s");
    play("se,sw,se,sw,sw");
    play(&contents[0][0]);
    Ok(())
}

fn play(steps_joined: &str) {
    let steps: Vec<&str> = steps_joined.split(',').collect();
    let mut pos = Pos::default();
    let mut max_distance = 0;
    for step in steps {
        // [Axial co-ords](https://www.redblobgames.com/grids/hexagons/#coordinates-axial)
        pos = pos + match step {
            "n" => Pos(0, -1),
            "s" => Pos(0, 1),
            "ne" => Pos(1, -1),
            "nw" => Pos(-1, 0),
            "se" => Pos(1, 0),
            "sw" => Pos(-1, 1),
            _ => unreachable!(),
        };
        max_distance = max(max_distance, pos.distance());
    }
    println!(
        "Distance: {:?} = {} (max {})",
        pos,
        pos.distance(),
        max_distance
    );
}

impl Pos {
    fn distance(&self) -> i64 {
        (self.0.abs() + (self.0 + self.1).abs() + self.1.abs()) / 2
    }
}
