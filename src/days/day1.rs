use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::fs;
use std::time::*;

#[macro_use]
use crate::*;


pub fn day1e() -> u32 {
    let mut input = lines_from!("1e", u32);
    input.sort();

    for x in input.clone() {
        for y in input.clone() {
            if x+y == 2020 {
                return x*y;
            }
        }
    }

    0
}

pub fn day1a(entries: &mut Vec<u32>) -> u32 {
    for x in entries.clone() {
        for y in entries.clone() {
            if x+y == 2020 {
                return x*y;
            }
        }
    }

    0
}