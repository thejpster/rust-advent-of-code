use failure::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Turing {
    tape: HashMap<i64, bool>,
    state: State,
    idx: i64,
}

impl Turing {
    fn run(&mut self) {
        self.state = match self.state {
            State::A => self.run_a(),
            State::B => self.run_b(),
            State::C => self.run_c(),
            State::D => self.run_d(),
            State::E => self.run_e(),
            State::F => self.run_f(),
        }
    }

    fn get(&mut self) -> bool {
        *self.tape.get(&self.idx).unwrap_or(&false)
    }

    fn set(&mut self, value: bool) {
        self.tape.insert(self.idx, value);
    }

    fn run_a(&mut self) -> State {
        if !self.get() {
            self.set(true);
            self.idx += 1;
            State::B
        } else {
            self.set(false);
            self.idx -= 1;
            State::B
        }
    }

    fn run_b(&mut self) -> State {
        if !self.get() {
            self.set(true);
            self.idx -= 1;
            State::C
        } else {
            self.set(false);
            self.idx += 1;
            State::E
        }
    }

    fn run_c(&mut self) -> State {
        if !self.get() {
            self.set(true);
            self.idx += 1;
            State::E
        } else {
            self.set(false);
            self.idx -= 1;
            State::D
        }
    }

    fn run_d(&mut self) -> State {
        self.set(true);
        self.idx -= 1;
        State::A
    }

    fn run_e(&mut self) -> State {
        if !self.get() {
            self.set(false);
            self.idx += 1;
            State::A
        } else {
            self.set(false);
            self.idx += 1;
            State::F
        }
    }

    fn run_f(&mut self) -> State {
        if !self.get() {
            self.set(true);
            self.idx += 1;
            State::E
        } else {
            self.set(true);
            self.idx += 1;
            State::A
        }
    }

    fn checksum(&self) -> i64 {
        self.tape.values().map(|x| if *x { 1 } else { 0 }).sum()
    }
}

const LIMIT: usize = 12_683_008;

pub fn run(_contents: &[Vec<String>]) -> Result<(), Error> {
    let mut t = Turing {
        tape: HashMap::new(),
        state: State::A,
        idx: 0,
    };

    for _ in 0..LIMIT {
        t.run();
    }
    println!("Checksum: {}", t.checksum());

    Ok(())
}
