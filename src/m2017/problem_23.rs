#[derive(Debug)]
struct Cpu {
    registers: Vec<i64>,
    pc: usize,
    mul_count: usize,
}

impl Cpu {
    fn new(a: i64) -> Cpu {
        let mut c = Cpu {
            registers: vec![0; 8],
            pc: 0,
            mul_count: 0,
        };
        c.registers[0] = a;
        c
    }

    fn reg_map(reg: &str) -> usize {
        match reg {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => panic!("Bad register"),
        }
    }

    fn get_reg(&self, reg: &str) -> i64 {
        match reg.parse() {
            Ok(x) => x,
            Err(_) => self.registers[Cpu::reg_map(reg)],
        }
    }

    fn set(&mut self, reg: &str, reg2: &str) {
        let x = self.get_reg(reg2);
        self.registers[Cpu::reg_map(reg)] = x;
        self.pc += 1;
    }

    fn mul(&mut self, reg: &str, reg2: &str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x *= y;
        self.registers[Cpu::reg_map(reg)] = x;
        self.pc += 1;
    }

    fn sub(&mut self, reg: &str, reg2: &str) {
        let mut x = self.get_reg(reg);
        let y = self.get_reg(reg2);
        x -= y;
        self.registers[Cpu::reg_map(reg)] = x;
        self.pc += 1;
    }

    fn jnz(&mut self, reg: &str, reg2: &str) {
        let x = self.get_reg(reg);
        let jump: i64 =
            if x != 0 { self.get_reg(reg2) } else { 1 };
        self.pc = (self.pc as i64 + jump) as usize;
    }

    fn run1(&mut self, line: &str) -> bool {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // println!("Running #{}: {:?}", self.pc, parts);
        match parts[0] {
            "set" => self.set(parts[1], parts[2]),
            "mul" => {
                self.mul_count += 1;
                self.mul(parts[1], parts[2])
            }
            "jnz" => self.jnz(parts[1], parts[2]),
            "sub" => self.sub(parts[1], parts[2]),
            _ => panic!("Unsuported line: {:?}", parts),
        }
        false
    }
}

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    run1(contents)?;
    run2(contents)?;
    Ok(())
}

fn run1(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut cpu = Cpu::new(0);
    loop {
        if cpu.pc >= contents[0].len() {
            break;
        }
        let line = &contents[0][cpu.pc];
        if cpu.run1(line) {
            break;
        }
    }
    println!("Mul count: {}", cpu.mul_count);
    Ok(())
}

fn is_prime(num: u32) -> bool {
    for factor in 2..num {
        if (num % factor) == 0 {
            return false;
        }
    }
    true
}

fn run2(_contents: &[Vec<String>]) -> Result<(), Error> {
    let mut result = 0;
    let mut i = 108_100u32;
    while i <= 125_100u32 {
        if !is_prime(i) {
            result += 1;
        }
        i += 17;
    }
    println!("Result: {}", result);
    Ok(())
}
