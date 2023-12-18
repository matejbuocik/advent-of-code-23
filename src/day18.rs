use regex::Regex;

pub fn p1() {
    solve(|dir, n, _| {
        let dir = dir.chars().next().unwrap();
        let n: i64 = n.parse().unwrap();
        (dir, n)
    })
}

pub fn p2() {
    solve(|_, _, hex| {
        let dir = match &hex[5..] {
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => panic!(),
        };
        let n = i64::from_str_radix(&hex[..5], 16).unwrap();
        (dir, n)
    })
}

fn solve<F: Fn(&str, &str, &str) -> (char, i64)>(get_dir_n: F) {
    let input = crate::read_file(18);
    let re = Regex::new(r"^(U|D|L|R) (\d+) \(#(.+)\)$").unwrap();

    let mut curr = (0, 0);
    let mut interior = 0;
    let mut border = 0;

    for [dir, n, hex] in input.lines().map(|l| re.captures(l).unwrap().extract().1) {
        let (dir, n) = get_dir_n(dir, n, hex);

        let next = match dir {
            'U' => (curr.0, curr.1 - n),
            'D' => (curr.0, curr.1 + n),
            'L' => (curr.0 - n, curr.1),
            'R' => (curr.0 + n, curr.1),
            _ => panic!(),
        };

        // Shoelace formula https://en.wikipedia.org/wiki/Shoelace_formula
        interior += curr.0 * next.1 - curr.1 * next.0;
        border += n;
        curr = next;
    }

    // Interior can be negative
    if interior < 0 {
        interior = -interior;
    }

    // Pick's theorem https://en.wikipedia.org/wiki/Pick%27s_theorem
    let result = (interior >> 1) + (border >> 1) + 1;
    println!("{}", result);
}
