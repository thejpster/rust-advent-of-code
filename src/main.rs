use std::fs::File;
use std::io::prelude::*;

use std::env;

mod m2017;
mod m2016;

use m2017 as year;

enum Error {
    IOError(String, std::io::Error),
    BadNumber(std::num::ParseIntError),
    BadArgs(String)
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::BadNumber(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Error::IOError(ref filename, ref e) => write!(f, "Got '{}' handling file '{}'", e, filename),
            &Error::BadNumber(ref e) => write!(f, "Couldn't parse number, got {}", e),
            &Error::BadArgs(ref s) => write!(f, "Call: {} <problem> <file> [ <file> ... ]", s),
        }
    }
}

fn main() {
    let (problem, filenames) = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            panic!("Error parsing arguments.");
        }
    };
    let files: Vec<Vec<String>> = match filenames.iter().map(|name| open(name)).collect::<Result<Vec<_>, _>>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            panic!("Error opening file");
        }
    };
    match problem {
        1 => year::problem_1::run(&files),
        2 => year::problem_2::run(&files),
        n => panic!("Don't have problem {}", n),
    }
}

fn parse_args() -> Result<(u32, Vec<String>), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(Error::BadArgs(args[0].clone()));
    }
    let problem_number = &args[1];
    let file_names = args[2..].to_vec();
    let problem_number = problem_number.parse::<u32>()?;
    return Ok((problem_number, file_names));
}

fn open(filename: &str) -> Result<Vec<String>, Error>
{
    let mut file = File::open(filename).map_err(|e| Error::IOError(filename.into(), e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| Error::IOError(filename.into(), e))?;
    Ok(contents.lines().map(|x| x.to_string()).collect())
}
