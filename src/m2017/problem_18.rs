use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Cpu<'a> {
    registers: HashMap<&'a str, i64>,
    queue: VecDeque<i64>,
    pc: usize,
    count: usize,
    snd: i64,
}

impl<'a> Cpu<'a> {
    fn new(p: i64) -> Cpu<'a> {
        let mut c = Cpu {
            registers: HashMap::new(),
            pc: 0,
            snd: 0,
            queue: VecDeque::new(),
            count: 0,
        };
        c.registers.insert("a", 0);
        c.registers.insert("b", 0);
        c.registers.insert("f", 0);
        c.registers.insert("i", 0);
        c.registers.insert("p", p);
        c
    }

    fn get_reg(&self, reg: &'a str) -> i64 {
        match reg.parse() {
            Ok(x) => x,
            Err(_) => self.registers[reg],
        }
    }

    fn set(&mut self, reg: &'a str, reg2: &'a str) {
        let x = self.get_reg(reg2);
        self.registers.insert(reg, x);
        self.pc += 1;
    }

    fn mul(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x *= y;
        self.registers.insert(reg, x);
        self.pc += 1;
    }

    fn add(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x += y;
        self.registers.insert(reg, x);
        self.pc += 1;
    }

    fn modulo(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x %= y;
        self.registers.insert(reg, x);
        self.pc += 1;
    }

    fn jgz(&mut self, reg: &'a str, reg2: &'a str) {
        let x = self.get_reg(reg);
        let jump: i64 = if x > 0 { self.get_reg(reg2) } else { 1 };
        self.pc = (self.pc as i64 + jump) as usize;
    }

    fn snd1(&mut self, reg: &'a str) {
        let s = self.get_reg(reg);
        self.snd = s;
        self.pc += 1;
    }

    fn rcv1(&mut self, reg: &'a str) -> bool {
        let s = self.get_reg(reg);
        self.pc += 1;
        if s != 0 {
            println!("Rcv: {}", self.snd);
            true
        } else {
            false
        }
    }

    fn snd(&mut self, reg: &'a str, cpu: &mut Cpu) {
        let s = self.get_reg(reg);
        cpu.queue.push_back(s);
        cpu.count += 1;
        self.pc += 1;
    }

    fn rcv(&mut self, reg: &'a str) {
        if let Some(x) = self.queue.pop_front() {
            self.registers.insert(reg, x);
            self.pc += 1;
        }
    }

    fn run1(&mut self, line: &'a str) -> bool {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // println!("Running #{}: {:?}", self.pc, parts);
        match parts[0] {
            "set" => self.set(parts[1], parts[2]),
            "mul" => self.mul(parts[1], parts[2]),
            "jgz" => self.jgz(parts[1], parts[2]),
            "add" => self.add(parts[1], parts[2]),
            "snd" => self.snd1(parts[1]),
            "rcv" => if self.rcv1(parts[1]) {
                return true;
            },
            "mod" => self.modulo(parts[1], parts[2]),
            _ => panic!("Unsuported line: {:?}", parts),
        }
        false
    }

    fn run2(&mut self, line: &'a str, other: &mut Cpu) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // println!("Running #{}: {:?}", self.pc, parts);
        match parts[0] {
            "set" => self.set(parts[1], parts[2]),
            "mul" => self.mul(parts[1], parts[2]),
            "jgz" => self.jgz(parts[1], parts[2]),
            "add" => self.add(parts[1], parts[2]),
            "snd" => self.snd(parts[1], other),
            "rcv" => self.rcv(parts[1]),
            "mod" => self.modulo(parts[1], parts[2]),
            _ => panic!("Unsuported line: {:?}", parts),
        }
    }
}

pub fn run(contents: &[Vec<String>]) {
    run1(contents);
    run2(contents);
}

fn run1(contents: &[Vec<String>]) {
    let mut cpu = Cpu::new(0);
    loop {
        let line = &contents[0][cpu.pc];
        if cpu.run1(line) {
            break;
        }
    }
}

fn run2(contents: &[Vec<String>]) {
    let mut cpu0 = Cpu::new(0);
    let mut cpu1 = Cpu::new(1);
    let mut old_pcs = [0, 0];
    loop {
        for (idx, old_pc) in old_pcs.iter_mut().enumerate() {
            let (cpu, other) = match idx {
                0 => (&mut cpu0, &mut cpu1),
                1 => (&mut cpu1, &mut cpu0),
                _ => panic!(),
            };
            let line = &contents[0][cpu.pc];
            *old_pc = cpu.pc;
            cpu.run2(line, other);
        }
        if old_pcs[0] == cpu0.pc && old_pcs[1] == cpu1.pc {
            break;
        }
    }
    println!("Part2 {}", cpu0.count);
}
