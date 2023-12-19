use regex::Regex;
use std::collections::HashMap;

type Part = (i64, i64, i64, i64);
type Range = (Part, Part);
type Function = dyn Fn(&Part) -> bool;

struct Rule {
    cond: Option<Box<Function>>,
    next_wf: String,
}

impl Rule {
    fn parse(str: &str) -> Self {
        let vec = str.split(':').collect::<Vec<_>>();
        if vec.len() < 2 {
            return Rule {
                cond: None,
                next_wf: vec[0].to_string(),
            };
        }

        let category = vec[0][..1].to_string();
        let comp = vec[0][1..2].to_string();
        let num = vec[0][2..].parse::<i64>().unwrap();

        let cond = Box::new(move |p: &Part| {
            let n = match category.as_str() {
                "x" => p.0,
                "m" => p.1,
                "a" => p.2,
                "s" => p.3,
                _ => panic!(),
            };

            match comp.as_str() {
                "<" => n < num,
                ">" => n > num,
                _ => panic!(),
            }
        });

        Rule {
            cond: Some(cond),
            next_wf: vec[1].to_string(),
        }
    }
}

fn get_workflows(str: &str) -> HashMap<String, Vec<Rule>> {
    let mut map = HashMap::new();

    for line in str.lines() {
        let vec = line.split('{').collect::<Vec<_>>();
        let label = vec[0].to_string();

        let mut rules = vec![];
        vec[1]
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .for_each(|r| rules.push(Rule::parse(r)));

        map.insert(label, rules);
    }

    map
}

fn get_parts(str: &str) -> Vec<Part> {
    let re = Regex::new(r"\d+").unwrap();
    str.lines()
        .map(|l| {
            let nums = re
                .find_iter(l)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (nums[0], nums[1], nums[2], nums[3])
        })
        .collect()
}

pub fn p1() {
    let input = crate::read_file(19);
    let section = input.split("\n\n").collect::<Vec<_>>();
    let workflows = get_workflows(section[0]);
    let parts = get_parts(section[1]);

    let mut result = 0;

    for part in parts {
        let mut curr = "in";

        while curr != "A" && curr != "R" {
            for rule in workflows.get(curr).unwrap() {
                match &rule.cond {
                    None => {
                        curr = rule.next_wf.as_str();
                    }
                    Some(cond) if (*cond)(&part) => {
                        curr = rule.next_wf.as_str();
                        break;
                    }
                    _ => (),
                }
            }
        }

        if curr == "A" {
            result += part.0 + part.1 + part.2 + part.3;
        }
    }

    println!("{}", result);
}

pub fn p2() {
    let input = crate::read_file(19);
    let section = input.split("\n\n").collect::<Vec<_>>();
    let workflows = get_workflows(section[0]);

    let mut result = 0;
    let mut states: Vec<(&str, Range)> = vec![("A", ((1, 1, 1, 1), (4000, 4000, 4000, 4000)))];

    while let Some(state) = states.pop() {
        let (label, (from, to)) = state;

        if label == "A" {
            result += (to.0 - from.0 + 1)
                * (to.1 - from.1 + 1)
                * (to.2 - from.2 + 1)
                * (to.3 - from.3 + 1);
            continue;
        } else if label == "R" {
            continue;
        }

        for rule in workflows.get(label).unwrap() {
            match &rule.cond {
                None => {
                    curr = rule.next_wf.as_str();
                }
                Some(cond) if (*cond)(&part) => {
                    curr = rule.next_wf.as_str();
                    break;
                }
                _ => (),
            }
        }
    }

    println!("{}", result);
}
