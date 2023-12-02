use std::iter::repeat;

pub fn p1() {
    let input = crate::read_file(1);
    let mut sum = 0;

    for line in input.lines() {
        let mut iter = line.chars().filter_map(|c| c.to_digit(10));
        let first_digit = iter.next().unwrap(); // There is always at least one digit
        let last_digit = match iter.last() {
            Some(digit) => digit,
            None => first_digit,
        };
        sum += first_digit * 10 + last_digit;
    }

    println!("{}", sum);
}

pub fn p2() {
    let input = crate::read_file(1);
    let mut sum = 0;

    for line in input.lines() {
        let mut iter = line
            .chars()
            .zip(line.chars().skip(1).chain(repeat('z')))
            .zip(line.chars().skip(2).chain(repeat('z')))
            .zip(line.chars().skip(3).chain(repeat('z')))
            .zip(line.chars().skip(4).chain(repeat('z')))
            .filter_map(|t| {
                if let Some(d) = t.0 .0 .0 .0.to_digit(10) {
                    return Some(d);
                }

                match t {
                    (((('o', 'n'), 'e'), _), _) => Some(1),
                    (((('t', 'w'), 'o'), _), _) => Some(2),
                    (((('t', 'h'), 'r'), 'e'), 'e') => Some(3),
                    (((('f', 'o'), 'u'), 'r'), _) => Some(4),
                    (((('f', 'i'), 'v'), 'e'), _) => Some(5),
                    (((('s', 'i'), 'x'), _), _) => Some(6),
                    (((('s', 'e'), 'v'), 'e'), 'n') => Some(7),
                    (((('e', 'i'), 'g'), 'h'), 't') => Some(8),
                    (((('n', 'i'), 'n'), 'e'), _) => Some(9),
                    _ => None,
                }
            });

        let first_digit = iter.next().unwrap(); // There is always at least one digit
        let last_digit = match iter.last() {
            Some(digit) => digit,
            None => first_digit,
        };
        sum += first_digit * 10 + last_digit;
    }

    println!("{}", sum);
}
