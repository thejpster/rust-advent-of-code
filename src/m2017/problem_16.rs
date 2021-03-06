use std::collections::HashMap;
use failure::Error;

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Debug)]
struct Buffer {
    index: HashMap<char, usize>,
    len: usize,
}

impl Buffer {
    fn new(len: u8) -> Buffer {
        let mut index = HashMap::new();
        for (i, c) in (0..len).map(|x| (x + 97).into()).enumerate() {
            index.insert(c, i);
        }
        Buffer {
            index: index,
            len: len as usize,
        }
    }

    fn rotate(&mut self, offset: usize) {
        // abcde offset 3 => cdeab
        let len = self.len;
        for i in self.index.values_mut() {
            *i = (*i + offset) % len;
        }
    }

    fn swap_by_index(&mut self, a: usize, b: usize) {
        let mut k_a = None;
        let mut k_b = None;
        for (k, v) in &self.index {
            if *v == a {
                k_a = Some(*k);
            }
            if *v == b {
                k_b = Some(*k);
            }
        }
        self.swap_by_char(k_a.unwrap(), k_b.unwrap());
    }

    fn swap_by_char(&mut self, a: char, b: char) {
        let i1 = self.index[&a];
        let i2 = self.index[&b];
        self.index.insert(a, i2);
        self.index.insert(b, i1);
    }

    fn as_string(&self) -> String {
        let mut buffer: Vec<(char, usize)> = self.index.iter().map(|(k, v)| (*k, *v)).collect();
        buffer.sort_by_key(|k| k.1);
        buffer.iter().map(|k| k.0).collect()
    }
}

pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let steps = contents[0][0].split(',');
    let steps: Vec<Move> = steps.map(|x| decode(x)).collect::<Result<_, _>>()?;
    let mut buffer = Buffer::new(16);
    let mut seen: Vec<String> = Vec::new();
    println!("Programs: {:?}", buffer);

    for i in 0.. {
        let s = buffer.as_string();
        if seen.contains(&s) {
            println!(
                "We've seen {} before. Cycle length: {}, Result: {}",
                s,
                i,
                seen[1_000_000_000 % i]
            );
            break;
        }
        seen.push(s);
        for step in &steps {
            match *step {
                Move::Exchange(a, b) => buffer.swap_by_index(a, b),
                Move::Spin(s) => buffer.rotate(s),
                Move::Partner(a, b) => buffer.swap_by_char(a, b),
            }
        }
    }
    println!("Part1: {}", seen[1]);
    Ok(())
}

fn decode(input: &str) -> Result<Move, Error> {
    match &input[0..1] {
        "s" => Ok(Move::Spin(input[1..].parse().unwrap())),
        "x" => {
            let mut parts = input[1..].split('/').map(|x| x.parse::<usize>().unwrap());
            Ok(Move::Exchange(parts.next().unwrap(), parts.next().unwrap()))
        }
        "p" => {
            let mut parts = input[1..].split('/').map(|x| x.chars().nth(0).unwrap());
            Ok(Move::Partner(parts.next().unwrap(), parts.next().unwrap()))
        }
        e => Err(format_err!("Bad input {}", e)),
    }
}
