use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Play {
    value: u8,
    hand: Vec<char>,
    bid: u32,
}

pub fn p1() {
    let input = crate::read_file(7);

    let mut hands = vec![];
    for line in input.lines() {
        let hand: Vec<_> = line
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .to_string()
            .chars()
            .map(|c| match c {
                'A' => 'e',
                'K' => 'd',
                'Q' => 'c',
                'J' => 'b',
                'T' => 'a',
                c => c,
            })
            .collect();
        let bid = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut cards: HashMap<char, u8> = HashMap::new();
        for card in &hand {
            *cards.entry(*card).or_insert(0) += 1;
        }
        let max_n = *cards.values().max().unwrap();

        let value = if max_n == 5 {
            6
        } else if max_n == 4 {
            5
        } else if max_n == 3 && cards.len() == 2 {
            4
        } else if max_n == 3 {
            3
        } else if max_n == 2 && cards.len() == 3 {
            2
        } else if max_n == 2 {
            1
        } else {
            0
        };

        hands.push(Play { value, hand, bid });
    }

    hands.sort();
    let result: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum();
    println!("{}", result);
}

pub fn p2() {
    let input = crate::read_file(7);

    let mut hands = vec![];
    for line in input.lines() {
        let hand: Vec<_> = line
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .to_string()
            .chars()
            .map(|c| match c {
                'A' => 'e',
                'K' => 'd',
                'Q' => 'c',
                'J' => '1',
                'T' => 'a',
                c => c,
            })
            .collect();
        let bid = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut cards: HashMap<char, u8> = HashMap::new();
        for card in &hand {
            *cards.entry(*card).or_insert(0) += 1;
        }

        let mut max_c = '1';
        let mut max_n = 0;
        for (&c, &n) in &cards {
            if c != '1' && n > max_n {
                max_c = c;
                max_n = n;
            }
        }
        let jokers = cards.remove(&'1').unwrap_or(0);
        *cards.entry(max_c).or_insert(0) += jokers;
        max_n += jokers;

        let value = if max_n == 5 {
            6
        } else if max_n == 4 {
            5
        } else if max_n == 3 && cards.len() == 2 {
            4
        } else if max_n == 3 {
            3
        } else if max_n == 2 && cards.len() == 3 {
            2
        } else if max_n == 2 {
            1
        } else {
            0
        };

        hands.push(Play { value, hand, bid });
    }

    hands.sort();
    let result: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum();
    println!("{}", result);
}
