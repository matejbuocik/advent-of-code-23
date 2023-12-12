use std::collections::HashMap;
use std::iter::repeat;

type Cache = HashMap<(usize, usize, u64), u64>;

pub fn p1() {
    let input = crate::read_file(12);
    let mut sum = 0;

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        let chars: Vec<char> = iter.next().unwrap().chars().collect();
        let counts: Vec<u64> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut cache = Cache::new();
        let options = solve(&mut cache, &chars, &counts, 0, 0, 0);
        sum += options
    }

    println!("{}", sum);
}

pub fn p2() {
    let input = crate::read_file(12);
    let mut sum = 0;

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        let chars = iter.next().unwrap();
        let chars = repeat(chars)
            .take(5)
            .collect::<Vec<_>>()
            .join("?")
            .chars()
            .collect();
        let counts: Vec<u64> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let counts = repeat(counts).take(5).collect::<Vec<_>>().concat();

        let mut cache = Cache::new();
        let options = solve(&mut cache, &chars, &counts, 0, 0, 0);
        sum += options
    }

    println!("{}", sum);
}

fn solve(
    cache: &mut Cache,
    chars: &Vec<char>,
    counts: &Vec<u64>,
    i: usize,
    ci: usize,
    count: u64,
) -> u64 {
    if let Some(&value) = cache.get(&(i, ci, count)) {
        return value;
    }

    if i == chars.len() {
        if (ci == counts.len() && count == 0) || (ci == counts.len() - 1 && count == counts[ci]) {
            return 1;
        } else {
            return 0;
        }
    }

    let mut options = 0;
    for ch in ['#', '.'] {
        if chars[i] == ch || chars[i] == '?' {
            if ch == '#' {
                options += solve(cache, chars, counts, i + 1, ci, count + 1);
            } else if ch == '.' && count == 0 {
                options += solve(cache, chars, counts, i + 1, ci, 0);
            } else if ch == '.' && count != 0 && counts.get(ci) == Some(&count) {
                options += solve(cache, chars, counts, i + 1, ci + 1, 0);
            }
        }
    }

    cache.insert((i, ci, count), options);
    options
}
