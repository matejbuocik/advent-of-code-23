use std::cmp::min;
use std::collections::HashSet;
use std::iter::{once, repeat};

pub fn p1() {
    let input = crate::read_file(4);
    let mut sum = 0;

    for line in input.lines() {
        let mut winning: HashSet<u32> = HashSet::new();
        let mut number = 0;
        let mut hits: u32 = 0;
        let mut my_numbers = false;

        for ch in line.chars().skip_while(|c| *c != ':').chain(once('\n')) {
            if let Some(digit) = ch.to_digit(10) {
                number = number * 10 + digit;
            } else {
                if !my_numbers && number != 0 {
                    winning.insert(number);
                } else if my_numbers && winning.contains(&number) {
                    hits += 1;
                }

                my_numbers = my_numbers || ch == '|';
                number = 0;
            }
        }

        sum += if hits == 0 { 0 } else { 1 << (hits - 1) };
    }

    println!("{}", sum);
}

pub fn p2() {
    let input = crate::read_file(4);
    let mut sum = 0;
    let mut counts = std::collections::VecDeque::new();

    for line in input.lines() {
        let mut winning: HashSet<u32> = HashSet::new();
        let mut number = 0;
        let mut hits = 0;
        let mut my_numbers = false;

        for ch in line.chars().skip_while(|c| *c != ':').chain(once('\n')) {
            if let Some(digit) = ch.to_digit(10) {
                number = number * 10 + digit;
            } else {
                if !my_numbers && number != 0 {
                    winning.insert(number);
                } else if my_numbers && winning.contains(&number) {
                    hits += 1;
                }

                my_numbers = my_numbers || ch == '|';
                number = 0;
            }
        }

        let count = counts.pop_front().unwrap_or(1);
        sum += count;

        for i in 0..min(hits, counts.len()) {
            counts[i] += count;
        }
        counts.extend(repeat(1 + count).take(hits.saturating_sub(counts.len())));
    }

    println!("{}", sum);
}
