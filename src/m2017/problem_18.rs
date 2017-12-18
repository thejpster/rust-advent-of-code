use std::collections::{HashMap, VecDeque};

struct CpuSingle<'a> {
    registers: HashMap<&'a str, i64>,
    snd: i64,
}

impl<'a> CpuSingle<'a> {
    fn new() -> CpuSingle<'a> {
        let mut c = CpuSingle {
            registers: HashMap::new(),
            snd: 0,
        };
        c.set("a", "0");
        c.set("b", "0");
        c.set("f", "0");
        c.set("i", "0");
        c.set("p", "0");
        c
    }

    fn get_reg(&self, reg: &'a str) -> i64 {
        match reg.parse() {
            Ok(x) => x,
            Err(_) => *self.registers.get(reg).unwrap(),
        }
    }

    fn set(&mut self, reg: &'a str, reg2: &'a str) {
        let x = self.get_reg(reg2);
        self.registers.insert(reg, x);
    }

    fn mul(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x = x * y;
        self.registers.insert(reg, x);
    }

    fn add(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x = x + y;
        self.registers.insert(reg, x);
    }

    fn modulo(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x = x % y;
        self.registers.insert(reg, x);
    }

    fn jgz(&mut self, reg: &'a str, reg2: &'a str) -> i64 {
        let x = self.get_reg(reg);
        if x > 0 {
            self.get_reg(reg2)
        } else {
            1
        }
    }

    fn snd(&mut self, reg: &'a str) {
        let s = self.get_reg(reg);
        self.snd = s;
    }

    fn rcv(&mut self, reg: &'a str) -> bool {
        let s = self.get_reg(reg);
        if s != 0 {
            println!("Rcv: {}", self.snd);
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Cpu<'a> {
    registers: HashMap<&'a str, i64>,
    queue: VecDeque<i64>,
    pc: usize,
    count: usize,
}

impl<'a> Cpu<'a> {
    fn new(p: i64) -> Cpu<'a> {
        let mut c = Cpu {
            registers: HashMap::new(),
            pc: 0,
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
            Err(_) => *self.registers.get(reg).unwrap(),
        }
    }

    fn set(&mut self, reg: &'a str, reg2: &'a str) {
        let x = self.get_reg(reg2);
        self.registers.insert(reg, x);
        self.pc = self.pc + 1;
    }

    fn mul(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x = x * y;
        self.registers.insert(reg, x);
        self.pc = self.pc + 1;
    }

    fn add(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x = x + y;
        self.registers.insert(reg, x);
        self.pc = self.pc + 1;
    }

    fn modulo(&mut self, reg: &'a str, reg2: &'a str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x = x % y;
        self.registers.insert(reg, x);
        self.pc = self.pc + 1;
    }

    fn jgz(&mut self, reg: &'a str, reg2: &'a str) {
        let x = self.get_reg(reg);
        let jump: i64 = if x > 0 { self.get_reg(reg2) } else { 1 };
        self.pc = (self.pc as i64 + jump) as usize;
    }

    fn snd(&mut self, reg: &'a str, cpu: &mut Cpu) {
        let s = self.get_reg(reg);
        cpu.queue.push_back(s);
        cpu.count = cpu.count + 1;
        self.pc = self.pc + 1;
    }

    fn rcv(&mut self, reg: &'a str) {
        if self.queue.len() != 0 {
            self.registers.insert(reg, self.queue.pop_front().unwrap());
            self.pc = self.pc + 1;
        }
    }
}

pub fn run(contents: &Vec<Vec<String>>) {
    run1(contents);
    run2(contents);
}

fn run1(contents: &Vec<Vec<String>>) {
    let mut cpu = CpuSingle::new();
    let mut pc: i64 = 0;
    loop {
        let line = &contents[0][pc as usize];
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut offset = 1;
        // println!("Running #{}: {:?}", pc, parts);
        match parts[0].as_ref() {
            "set" => cpu.set(parts[1], parts[2]),
            "mul" => cpu.mul(parts[1], parts[2]),
            "jgz" => offset = cpu.jgz(parts[1], parts[2]),
            "add" => cpu.add(parts[1], parts[2]),
            "snd" => cpu.snd(parts[1]),
            "rcv" => if cpu.rcv(parts[1]) {
                break;
            },
            "mod" => cpu.modulo(parts[1], parts[2]),
            _ => panic!("Unsuported line: {:?}", parts),
        }
        pc = pc + offset;
    }
}

fn run2(contents: &Vec<Vec<String>>) {
    let mut cpu0 = Cpu::new(0);
    let mut cpu1 = Cpu::new(1);
    let mut old_pcs = [0, 0];
    loop {
        for idx in 0..2 {
            let (cpu, other) = match idx {
                0 => (&mut cpu0, &mut cpu1),
                1 => (&mut cpu1, &mut cpu0),
                _ => panic!(),
            };
            let line = &contents[0][cpu.pc];
            let parts: Vec<&str> = line.split_whitespace().collect();
            // println!("Running #{} on {}: {:?}", cpu.pc, idx, parts);
            old_pcs[idx] = cpu.pc;
            match parts[0].as_ref() {
                "set" => cpu.set(parts[1], parts[2]),
                "mul" => cpu.mul(parts[1], parts[2]),
                "jgz" => cpu.jgz(parts[1], parts[2]),
                "add" => cpu.add(parts[1], parts[2]),
                "snd" => cpu.snd(parts[1], other),
                "rcv" => cpu.rcv(parts[1]),
                "mod" => cpu.modulo(parts[1], parts[2]),
                _ => panic!("Unsuported line: {:?}", parts),
            }
        }
        if old_pcs[0] == cpu0.pc && old_pcs[1] == cpu1.pc {
            break;
        }
    }
    println!("Part2 {}", cpu0.count);
}
