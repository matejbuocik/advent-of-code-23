use regex::Regex;
use std::collections::HashMap;

pub fn p1() {
    let input = crate::read_file(8);
    let mut directions = input.lines().next().unwrap().chars().cycle();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let map_regex = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();

    for line in input.lines().skip(2) {
        let (_, [from, to_l, to_r]) = map_regex.captures(line).unwrap().extract();
        map.insert(from, (to_l, to_r));
    }

    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        current = match directions.next().unwrap() {
            'R' => map.get(current).unwrap().1,
            _ => map.get(current).unwrap().0,
        };
        steps += 1;
    }

    println!("{}", steps);
}

pub fn p2() {
    let input = crate::read_file(8);
    let mut directions = input.lines().next().unwrap().chars().cycle();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let map_regex = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();
    let mut nodes = vec![];
    let mut steps: Vec<Option<usize>> = vec![];

    for line in input.lines().skip(2) {
        let (_, [from, to_l, to_r]) = map_regex.captures(line).unwrap().extract();
        map.insert(from, (to_l, to_r));

        if from.ends_with('A') {
            nodes.push(from);
            steps.push(None);
        }
    }

    let mut step = 0;
    while !steps.iter().all(|s| s.is_some()) {
        let direction = directions.next().unwrap();
        step += 1;

        for i in 0..nodes.len() {
            nodes[i] = match direction {
                'R' => map.get(nodes[i]).unwrap().1,
                _ => map.get(nodes[i]).unwrap().0,
            };
            if nodes[i].ends_with('Z') {
                steps[i] = Some(step);
            }
        }
    }

    let solution = steps.iter().fold(1, |acc, s| lcm(acc, s.unwrap()));
    println!("{}", solution);
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        (max, min) = (min, max);
    }

    loop {
        (max, min) = (min, max % min);
        if min == 0 {
            return max;
        }
    }
}
