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
mod m2016;

use std::fs::File;
use std::io::prelude::*;
use std::env;


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
    #[fail(display = "you should call '{} <year> <problem> <file> [ <file> ... ]'", _0)]
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
    let (year, problem, filenames) = match parse_args() {
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
    match year {
        2016 => {
            match problem {
                1 => m2016::problem_1::run(&files),
                2 => m2016::problem_2::run(&files),
                3 => m2016::problem_3::run(&files),
                4 => m2016::problem_4::run(&files),
                5 => m2016::problem_5::run(&files),
                6 => m2016::problem_6::run(&files),
                7 => m2016::problem_7::run(&files),
                8 => m2016::problem_8::run(&files),
                9 => m2016::problem_9::run(&files),
                10 => m2016::problem_10::run(&files),
                11 => m2016::problem_11::run(&files),
                12 => m2016::problem_12::run(&files),
                13 => m2016::problem_13::run(&files),
                14 => m2016::problem_14::run(&files),
                15 => m2016::problem_15::run(&files),
                16 => m2016::problem_16::run(&files),
                17 => m2016::problem_17::run(&files),
                18 => m2016::problem_18::run(&files),
                19 => m2016::problem_19::run(&files),
                20 => m2016::problem_20::run(&files),
                21 => m2016::problem_21::run(&files),
                22 => m2016::problem_22::run(&files),
                23 => m2016::problem_23::run(&files),
                24 => m2016::problem_24::run(&files),
                _ => {
                    eprintln!("Don't have problem {} in year {}", problem, year);
                    std::process::exit(1);
                }
            }
        }
        2017 => {
            match problem {
                1 => m2017::problem_1::run(&files),
                2 => m2017::problem_2::run(&files),
                3 => m2017::problem_3::run(&files),
                4 => m2017::problem_4::run(&files),
                5 => m2017::problem_5::run(&files),
                6 => m2017::problem_6::run(&files),
                7 => m2017::problem_7::run(&files),
                8 => m2017::problem_8::run(&files),
                9 => m2017::problem_9::run(&files),
                10 => m2017::problem_10::run(&files),
                11 => m2017::problem_11::run(&files),
                12 => m2017::problem_12::run(&files),
                13 => m2017::problem_13::run(&files),
                14 => m2017::problem_14::run(&files),
                15 => m2017::problem_15::run(&files),
                16 => m2017::problem_16::run(&files),
                17 => m2017::problem_17::run(&files),
                18 => m2017::problem_18::run(&files),
                19 => m2017::problem_19::run(&files),
                20 => m2017::problem_20::run(&files),
                21 => m2017::problem_21::run(&files),
                22 => m2017::problem_22::run(&files),
                23 => m2017::problem_23::run(&files),
                24 => m2017::problem_24::run(&files),
                _ => {
                    eprintln!("Don't have problem {} in year {}", problem, year);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Don't have year {}", year);
            std::process::exit(1);
        }
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

fn parse_args() -> Result<(u32, u32, Vec<String>), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(AppError::BadArgs(args[0].clone()).into());
    }
    let year = args[1].parse::<u32>().map_err(|_| {
        AppError::BadNumber(args[1].clone())
    })?;
    let problem_number = args[2].parse::<u32>().map_err(|_| {
        AppError::BadNumber(args[2].clone())
    })?;
    let file_names = args[3..].to_vec();
    return Ok((year, problem_number, file_names));
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
