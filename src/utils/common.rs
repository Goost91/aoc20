use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::fs;
use std::time::*;

#[macro_export]
macro_rules! csv_from {
    ($filename:expr, $type:tt) => {
        std::fs::read_to_string(format!("days/{}.txt", $filename))
            .unwrap()
            .trim()
            .split(",")
            .map(|val| val.parse::<$type>().unwrap())
            .collect::<Vec<$type>>()
        }
}

#[macro_export]
macro_rules! lines_from {
    ($filename:expr, $type:tt) => {
        std::fs::read_to_string(format!("days/{}.txt", $filename))
            .unwrap()
            .trim()
            .split("\r\n")
            .map(|val| val.parse::<$type>().unwrap())
            .collect::<Vec<$type>>()
        }
}

// old stuff
#[macro_export]
macro_rules! line_reader {
    ($x:expr) => {
        std::fs::read_to_string(format!("days/{}.txt", $x))
        .unwrap()
        .trim()
        .split("\r\n")
            .collect::<Vec<&str>>()
    }
}

#[macro_export]
macro_rules! parsed_line_reader {
    ($x:expr, $y:ty) => {
    BufReader::new(File::open(format!("input/day{}", $x)).unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<$y>().unwrap())
    }
}

#[macro_export]
macro_rules! split_reader {
    ($x:expr, $y:expr) => {
        fs::read_to_string(format!("input/day{}", $x)).unwrap().split($y)
    }
}


pub fn read_input(day: u8) -> BufReader<File> {
    BufReader::new(File::open(format!("input/day{}", day)).unwrap())
}

pub fn to_digits(input: &u32) -> Vec<u32> {
    input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

#[macro_export]
macro_rules! time_since {
    ($y:expr) => {

    println!("Time since start: {} sec", +);
    }
}