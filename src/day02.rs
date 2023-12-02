use regex::Regex;

pub fn p1() {
    let input = crate::read_file(2);

    let id_r = Regex::new(r"Game (\d+)").unwrap();
    let red_r = Regex::new(r"(\d+) red").unwrap();
    let green_r = Regex::new(r"(\d+) green").unwrap();
    let blue_r = Regex::new(r"(\d+) blue").unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let id: u32 = id_r.captures(line).unwrap()[1].parse().unwrap();
        let red = red_r
            .captures_iter(line)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap();
        let green = green_r
            .captures_iter(line)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap();
        let blue = blue_r
            .captures_iter(line)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap();
        if red <= 12 && green <= 13 && blue <= 14 {
            sum += id;
        }
    }

    println!("{}", sum);
}

pub fn p2() {
    let input = crate::read_file(2);

    let red_r = Regex::new(r"(\d+) red").unwrap();
    let green_r = Regex::new(r"(\d+) green").unwrap();
    let blue_r = Regex::new(r"(\d+) blue").unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let red = red_r
            .captures_iter(line)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap();
        let green = green_r
            .captures_iter(line)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap();
        let blue = blue_r
            .captures_iter(line)
            .map(|c| c[1].parse::<u32>().unwrap())
            .max()
            .unwrap();

        sum += red * green * blue;
    }

    println!("{}", sum);
}
