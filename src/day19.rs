use regex::Regex;
use std::collections::HashMap;

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn new(x: i64, m: i64, a: i64, s: i64) -> Self {
        Part { x, m, a, s }
    }

    fn parse(str: &str) -> Vec<Self> {
        let mut vec = vec![];
        let re = Regex::new(r"\d+").unwrap();

        for line in str.lines() {
            let nums = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let part = Self::new(nums[0], nums[1], nums[2], nums[3]);
            vec.push(part);
        }

        vec
    }
}

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
                "x" => p.x,
                "m" => p.m,
                "a" => p.a,
                "s" => p.s,
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

pub fn p1() {
    let input = crate::read_file(19);
    let section = input.split("\n\n").collect::<Vec<_>>();
    let workflows = get_workflows(section[0]);
    let parts = Part::parse(section[1]);

    let mut result = 0;

    for part in parts {
        let mut curr = "in";

        while curr != "A" && curr != "R" {
            let rules = workflows.get(curr).unwrap();

            for rule in rules {
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
            result += part.x + part.m + part.a + part.s;
        }
    }

    println!("{}", result);
}

pub fn p2() {
    let input = crate::read_file(19);
    let section = input.split("\n\n").collect::<Vec<_>>();
}
