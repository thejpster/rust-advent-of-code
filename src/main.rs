//! Code runner for my Advent of Code solutions.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate failure;
#[macro_use]
extern crate failure_derive;

mod m2017;
// mod m2016;

use std::fs::File;
use std::io::prelude::*;
use std::env;

use m2017 as year;

use failure::Error;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

#[derive(Debug, Fail)]
enum AppError {
    #[fail(display = "got '{}' from file '{}'", _1, _0)]
    IOError(
        String,
        #[cause]
        std::io::Error
    ),
    #[fail(display = "{} doesn't look like a number", _0)]
    BadNumber(String),
    #[fail(display = "you should call '{} <problem> <file> [ <file> ... ]'", _0)]
    BadArgs(String),
}

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Entry point to the command line program.
///
/// Parses the arguments, reads the input files and executes the specified
/// problem.
fn main() {
    let (problem, filenames) = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            std::process::exit(1);
        }
    };
    let files: Vec<Vec<String>> = match filenames
        .iter()
        .map(|name| open(name))
        .collect::<Result<Vec<_>, _>>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };
    match problem {
        1 => year::problem_1::run(&files),
        2 => year::problem_2::run(&files),
        n => {
            eprintln!("Don't have problem {}", n);
            std::process::exit(1);
        }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

fn parse_args() -> Result<(u32, Vec<String>), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(AppError::BadArgs(args[0].clone()).into());
    }
    let problem_number = &args[1];
    let file_names = args[2..].to_vec();
    let problem_number = problem_number.parse::<u32>().map_err(|_| {
        AppError::BadNumber(problem_number.clone())
    })?;
    return Ok((problem_number, file_names));
}

fn open(filename: &str) -> Result<Vec<String>, Error> {
    let mut file = File::open(filename).map_err(|e| {
        AppError::IOError(filename.into(), e)
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| {
        AppError::IOError(filename.into(), e)
    })?;
    Ok(contents.lines().map(|x| x.to_string()).collect())
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
