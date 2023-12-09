pub fn p1() {
    solution(false);
}

pub fn p2() {
    solution(true);
}

fn solution(reverse: bool) {
    let sequences: Vec<Vec<i64>> = crate::read_file(9)
        .lines()
        .map(|l| {
            let iter = l.split_ascii_whitespace().map(|n| n.parse().unwrap());
            if reverse {
                iter.rev().collect()
            } else {
                iter.collect()
            }
        })
        .collect();

    let mut result = 0;
    for sequence in sequences {
        let mut chain = vec![sequence];

        while !chain.last().unwrap().iter().all(|n| *n == 0) {
            let last = chain.last().unwrap();
            let mut numbers = vec![];

            for i in 0..last.len() - 1 {
                numbers.push(last[i + 1] - last[i]);
            }

            chain.push(numbers);
        }

        chain.last_mut().unwrap().push(0);
        for i in (0..chain.len() - 1).rev() {
            let new_val = chain[i].last().unwrap() + chain[i + 1].last().unwrap();
            chain[i].push(new_val);
        }

        result += chain[0].last().unwrap();
    }

    println!("{}", result);
}
