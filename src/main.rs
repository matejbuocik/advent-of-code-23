#![allow(dead_code)]

use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn read_file(n: u32) -> String {
    fs::read_to_string(format!("data/{}.txt", n)).unwrap()
}

fn main() {
    day08::p1();
    day08::p2();
}
