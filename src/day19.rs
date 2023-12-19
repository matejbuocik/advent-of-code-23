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

struct Rule {
    cond: Box<dyn Fn(&Part) -> bool>,
    next_wf: String,
}

impl Rule {
    fn parse(str: &str) -> Option<Self> {
        let vec = str.split(':').collect::<Vec<_>>();
        if vec.len() < 2 {
            return None;
        }

        let condition = vec[0];
        let next_wf = vec[1].to_string();

        let category = condition[..1].to_string();
        let comp = condition[1..2].to_string();
        let num = condition[2..].parse::<i64>().unwrap();

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

        Some(Rule { cond, next_wf })
    }
}

struct WorkFlow {
    rules: Vec<Rule>,
    next_wf: String,
}

impl WorkFlow {
    fn new() -> Self {
        Self {
            rules: vec![],
            next_wf: String::new(),
        }
    }

    fn parse(str: &str) -> HashMap<String, Self> {
        let mut dict = HashMap::new();

        for line in str.lines() {
            let vec = line.split('{').collect::<Vec<_>>();
            let label = vec[0].to_string();

            let mut wf = WorkFlow::new();

            for rule_str in vec[1].strip_suffix('}').unwrap().split(',') {
                match Rule::parse(rule_str) {
                    Some(rule) => wf.rules.push(rule),
                    None => wf.next_wf = rule_str.to_string(), // Rule without condition
                }
            }

            dict.insert(label, wf);
        }

        dict
    }
}

pub fn p1() {
    let input = crate::read_file(19);
    let section = input.split("\n\n").collect::<Vec<_>>();
    let workflows = WorkFlow::parse(section[0]);
    let parts = Part::parse(section[1]);

    let mut result = 0;

    for part in parts {
        let mut curr = "in";

        'in_workflows: while curr != "A" && curr != "R" {
            let wf = workflows.get(curr).unwrap();

            for rule in &wf.rules {
                if (*rule.cond)(&part) {
                    curr = rule.next_wf.as_str();
                    continue 'in_workflows;
                }
            }

            curr = wf.next_wf.as_str();
        }

        if curr == "A" {
            result += part.x + part.m + part.a + part.s;
        }
    }

    println!("{}", result);
}

pub fn p2() {}
