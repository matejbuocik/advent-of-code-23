use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
struct Range {
    from: i64,
    to: i64,
}
impl Range {
    fn new(from: i64, to: i64) -> Self {
        Range { from, to }
    }
}

#[derive(Clone)]
struct State {
    wf: String,
    x: Range,
    m: Range,
    a: Range,
    s: Range,
    ruleno: usize,
}
impl State {
    fn combinations(&self) -> i64 {
        if &self.wf == "R" {
            return 0;
        }

        (self.x.to - self.x.from + 1)
            * (self.m.to - self.m.from + 1)
            * (self.a.to - self.a.from + 1)
            * (self.s.to - self.s.from + 1)
    }
}

type StateSplitFn = dyn Fn(&State) -> Vec<State>;

fn get_rule_closure(str: &str) -> Box<StateSplitFn> {
    let vec = str.split(':').collect::<Vec<_>>();

    if vec.len() < 2 {
        // last rule, always pass
        let next_wf = vec[0].to_string();
        return Box::new(move |s: &State| {
            let mut state = s.clone();
            state.wf = next_wf.clone();
            state.ruleno = 0;
            vec![state]
        });
    }

    let category = vec[0][..1].to_string();
    let next_wf = vec[1].to_string();
    let comp = vec[0][1..2].to_string();
    let num = vec[0][2..].parse::<i64>().unwrap();

    Box::new(move |state: &State| {
        let mut state1 = state.clone();
        let mut state2 = state.clone();

        let (rng1, rng2) = match category.as_str() {
            "x" => (&mut state1.x, &mut state2.x),
            "m" => (&mut state1.m, &mut state2.m),
            "a" => (&mut state1.a, &mut state2.a),
            "s" => (&mut state1.s, &mut state2.s),
            _ => panic!(),
        };

        if (&comp == "<" && rng1.to < num) || (&comp == ">" && rng1.from > num) {
            // whole is in range
            state1.wf = next_wf.clone();
            state1.ruleno = 0;
            return vec![state1];
        } else if (&comp == "<" && rng1.from >= num) || (&comp == ">" && rng1.to <= num) {
            // nothing in range
            state1.ruleno += 1;
            return vec![state1];
        }

        match comp.as_str() {
            "<" => {
                rng1.to = num - 1;
                rng2.from = num;
            }
            ">" => {
                rng1.from = num + 1;
                rng2.to = num;
            }
            _ => panic!(),
        };

        state1.wf = next_wf.clone();
        state1.ruleno = 0;
        state2.ruleno += 1;

        vec![state1, state2]
    })
}

fn get_workflows(str: &str) -> HashMap<String, Vec<Box<StateSplitFn>>> {
    str.lines()
        .map(|line| {
            let vec = line.split('{').collect::<Vec<_>>();
            let label = vec[0].to_string();
            let rules = vec[1]
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|r| get_rule_closure(r))
                .collect();
            (label, rules)
        })
        .collect()
}

fn get_parts(str: &str) -> Vec<State> {
    let re = Regex::new(r"\d+").unwrap();
    str.lines()
        .map(|l| {
            let nums = re
                .find_iter(l)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            State {
                wf: String::from("in"),
                x: Range::new(nums[0], nums[0]),
                m: Range::new(nums[1], nums[1]),
                a: Range::new(nums[2], nums[2]),
                s: Range::new(nums[3], nums[3]),
                ruleno: 0,
            }
        })
        .collect()
}

pub fn p1() {
    let input = crate::read_file(19);
    let section = input.split("\n\n").collect::<Vec<_>>();
    let workflows = get_workflows(section[0]);
    let parts = get_parts(section[1]);

    let mut result = 0;

    for mut part in parts {
        while &part.wf != "A" && &part.wf != "R" {
            let rules = workflows.get(&part.wf).unwrap();
            part = (rules.get(part.ruleno).unwrap())(&part).pop().unwrap();
        }

        if &part.wf == "A" {
            result += part.x.from + part.m.from + part.a.from + part.s.from;
        }
    }

    println!("{}", result);
}

pub fn p2() {
    let input = crate::read_file(19);
    let section = input.split("\n\n").collect::<Vec<_>>();
    let workflows = get_workflows(section[0]);

    let mut result = 0;
    let mut states: Vec<State> = vec![State {
        wf: "in".to_string(),
        x: Range::new(1, 4000),
        m: Range::new(1, 4000),
        a: Range::new(1, 4000),
        s: Range::new(1, 4000),
        ruleno: 0,
    }];

    while let Some(state) = states.pop() {
        if state.wf == "A" || state.wf == "R" {
            result += state.combinations();
            continue;
        }

        let rules = workflows.get(&state.wf).unwrap();
        states.extend((rules.get(state.ruleno).unwrap())(&state));
    }

    println!("{}", result);
}
