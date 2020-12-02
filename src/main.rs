#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(dead_code)]

use itertools::Itertools;

pub mod days;
pub mod utils;

use days::day2::*;

fn main() {
    let mut entries = lines_from!("2a", String);
    entries.sort();

    println!("day2a: {}", day2a(&mut entries));
    println!("day2b: {}", day2b(&mut entries));
}
