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
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

pub fn read_file(n: u32) -> String {
    fs::read_to_string(format!("data/{}.txt", n)).unwrap()
}

fn main() {
    day17::p1();
    day17::p2();
}
