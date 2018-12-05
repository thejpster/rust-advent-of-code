//! Code runner for my Advent of Code solutions.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

#![allow(dead_code)]

#[macro_use]
extern crate failure;
extern crate md5;

// mod m2017;
// mod m2016;
mod m2018;

use std::env;
use std::fs::File;
use std::io::prelude::*;

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
    IOError(String, #[cause] std::io::Error),
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
fn main() -> Result<(), Error> {
    let filenames = parse_args()?;
    let files: Vec<Vec<String>> = filenames
        .iter()
        .map(|name| open(name))
        .collect::<Result<_, _>>()?;
    m2018::problem_5::run(&files)
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

pub fn parse_numbers(lines: &[String]) -> Result<Vec<i64>, Error> {
    let numbers: Vec<i64> = lines.iter().map(|x| x.parse()).collect::<Result<_, _>>()?;
    Ok(numbers)
}

fn parse_args() -> Result<(Vec<String>), Error> {
    let args: Vec<String> = env::args().collect();
    let file_names = args[1..].to_vec();
    Ok(file_names)
}

fn open(filename: &str) -> Result<Vec<String>, Error> {
    let mut file = File::open(filename).map_err(|e| AppError::IOError(filename.into(), e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| AppError::IOError(filename.into(), e))?;
    Ok(contents.lines().map(|x| x.to_string()).collect())
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
