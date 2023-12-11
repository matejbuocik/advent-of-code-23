use std::cmp::{max, min};
use std::iter::repeat;

fn solution(expansion: usize) {
    let input = crate::read_file(11);
    let mut galaxies = vec![];

    let mut empty_rows: Vec<_> = repeat(true).take(input.lines().count()).collect();
    let mut empty_cols: Vec<_> = repeat(true)
        .take(input.lines().next().unwrap().chars().count())
        .collect();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((x, y));
                empty_rows[y] = false;
                empty_cols[x] = false;
            }
        }
    }

    let mut sum = 0;
    for (i, (x, y)) in galaxies.iter().enumerate() {
        for other in galaxies.iter().skip(i) {
            let (min_c, max_c) = (min(*x, other.0), max(*x, other.0));
            let (min_r, max_r) = (min(*y, other.1), max(*y, other.1));
            let skip_c = empty_cols
                .iter()
                .skip(min_c)
                .take(max_c - min_c)
                .fold(0, |acc, v| if *v { acc + expansion } else { acc });
            let skip_r = empty_rows
                .iter()
                .skip(min_r)
                .take(max_r - min_r)
                .fold(0, |acc, v| if *v { acc + expansion } else { acc });
            sum += max_c - min_c + max_r - min_r + skip_c + skip_r;
        }
    }

    println!("{}", sum);
}

pub fn p1() {
    solution(1);
}

pub fn p2() {
    solution(999999);
}
