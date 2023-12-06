pub fn p1() {
    let input = crate::read_file(6);
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut result = 1;
    for (&time, &distance) in lines[0].iter().zip(lines[1].iter()) {
        let bounds = (distance as f64 / time as f64).ceil() as u32;
        for t in bounds..=time - bounds {
            if t * (time - t) > distance {
                result *= (time + 1) - 2 * t;
                break;
            }
        }
    }

    println!("{}", result);
}

pub fn p2() {
    let input = crate::read_file(6);
    let lines: Vec<_> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect();

    let mut result = 0;
    let time = lines[0];
    let distance = lines[1];
    let bounds = (distance as f64 / time as f64).ceil() as u64;
    for t in bounds..=time - bounds {
        if t * (time - t) > distance {
            result = (time + 1) - 2 * t;
            break;
        }
    }

    println!("{}", result);
}
