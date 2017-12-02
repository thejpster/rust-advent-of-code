use std::fs::File;
use std::io::prelude::*;

use std::env;

mod problem_1;
mod problem_2;

fn main() {
    let (problem, filenames) = parse_args();
    let files: Vec<Vec<String>> = filenames.iter().map(|name| open(name)).collect();

    match problem {
        1 => problem_1::run(&files),
        2 => problem_2::run(&files),
        _ => panic!("Don't have that problem"),
    }
}

fn parse_args() -> (u32, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let problem_number = &args[1];
    let file_names = args[2..].to_vec();
    let problem_number: u32 = problem_number.parse().unwrap();
    return (problem_number, file_names);
}

fn open<T>(filename: &T) -> Vec<String>
where
    T: AsRef<std::path::Path> + std::fmt::Display,
{
    let mut file = File::open(filename).expect(&format!("Can't open file {}", filename));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|x| x.to_string()).collect()
}
