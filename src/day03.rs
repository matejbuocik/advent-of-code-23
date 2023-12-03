use std::cmp::min;
use std::collections::HashMap;

pub fn p1() {
    let input: Vec<(usize, Vec<(usize, char)>)> = crate::read_file(3)
        .lines()
        .map(|l| l.chars().enumerate().collect())
        .enumerate()
        .collect();

    let mut sum = 0;
    let mut number = 0;
    let mut symbol = false;

    for (y, line) in &input {
        for (x, ch) in line {
            if let Some(digit) = ch.to_digit(10) {
                number = number * 10 + digit;

                let from_x = if number == digit {
                    // first digit of a number
                    x.saturating_sub(1)
                } else {
                    *x + 1
                };
                let from_y = y.saturating_sub(1);
                let to_x = min(line.len() - 1, x + 1);
                let to_y = min(input.len() - 1, y + 1);
                for i in from_y..=to_y {
                    for j in from_x..=to_x {
                        symbol =
                            symbol || !(input[i].1[j].1.is_ascii_digit() || input[i].1[j].1 == '.');
                    }
                }
            } else {
                if number != 0 && symbol {
                    sum += number;
                }
                number = 0;
                symbol = false;
            }
        }
    }

    println!("{}", sum);
}

pub fn p2() {
    let input: Vec<(usize, Vec<(usize, char)>)> = crate::read_file(3)
        .lines()
        .map(|l| l.chars().enumerate().collect())
        .enumerate()
        .collect();

    let mut number = 0;
    let mut neighbor_stars = vec![];
    let mut stars: HashMap<(usize, usize), (u32, u32)> = HashMap::new();

    for (y, line) in &input {
        for (x, ch) in line {
            if let Some(digit) = ch.to_digit(10) {
                number = number * 10 + digit;

                let from_x = if number == digit {
                    // first digit of a number
                    x.saturating_sub(1)
                } else {
                    *x + 1
                };
                let to_x = min(line.len() - 1, x + 1);
                let from_y = y.saturating_sub(1);
                let to_y = min(input.len() - 1, y + 1);
                for i in from_y..=to_y {
                    for j in from_x..=to_x {
                        if input[i].1[j].1 == '*' {
                            neighbor_stars.push((i, j));
                        }
                    }
                }
            } else {
                if number != 0 {
                    for pos in &neighbor_stars {
                        let entry = stars.entry(*pos).or_insert((0, 1));
                        entry.0 += 1;
                        entry.1 *= number;
                    }
                }
                neighbor_stars.clear();
                number = 0;
            }
        }
    }

    let sum = stars
        .values()
        .fold(0, |acc, (n, mult)| if *n == 2 { acc + mult } else { acc });
    println!("{}", sum);
}
