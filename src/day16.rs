use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    x: usize,
    y: usize,
    dir: Dir,
}

impl State {
    fn new(x: usize, y: usize, dir: Dir) -> Self {
        State { x, y, dir }
    }

    fn next(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Self {
                x: self.x,
                y: self.y.wrapping_sub(1),
                dir,
            },
            Dir::Down => Self {
                x: self.x,
                y: self.y + 1,
                dir,
            },
            Dir::Left => Self {
                x: self.x.wrapping_sub(1),
                y: self.y,
                dir,
            },
            Dir::Right => Self {
                x: self.x + 1,
                y: self.y,
                dir,
            },
        }
    }
}

pub fn p1() {
    let input = crate::read_file(16);
    let field: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut energized: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|_| false).collect())
        .collect();
    let mut cache = HashSet::new();

    solve(
        &field,
        &mut energized,
        &mut cache,
        State::new(0, 0, Dir::Right),
    );

    let result = energized.iter().flatten().filter(|&&b| b).count();
    println!("{}", result);
}

fn solve(
    field: &Vec<Vec<char>>,
    energized: &mut Vec<Vec<bool>>,
    cache: &mut HashSet<State>,
    curr: State,
) {
    if curr.y >= field.len() || curr.x >= field[0].len() {
        return;
    }

    if cache.contains(&curr) {
        return;
    }

    energized[curr.y][curr.x] = true;
    let next_dir;

    match field[curr.y][curr.x] {
        '/' => {
            next_dir = vec![match curr.dir {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            }];
        }
        '\\' => {
            next_dir = vec![match curr.dir {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            }];
        }
        '|' => {
            if curr.dir == Dir::Left || curr.dir == Dir::Right {
                next_dir = vec![Dir::Up, Dir::Down];
            } else {
                next_dir = vec![curr.dir];
            }
        }
        '-' => {
            if curr.dir == Dir::Up || curr.dir == Dir::Down {
                next_dir = vec![Dir::Left, Dir::Right];
            } else {
                next_dir = vec![curr.dir];
            }
        }
        _ => next_dir = vec![curr.dir],
    }

    cache.insert(curr);
    for dir in next_dir {
        solve(field, energized, cache, curr.next(dir));
    }
}

pub fn p2() {
    let input = crate::read_file(16);
    let field: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut max = 0;

    let mut states = vec![];
    for r in 0..field.len() {
        states.push(State::new(0, r, Dir::Right));
        states.push(State::new(field[0].len() - 1, r, Dir::Left));
    }
    for c in 0..field[0].len() {
        states.push(State::new(c, 0, Dir::Down));
        states.push(State::new(c, field.len() - 1, Dir::Up));
    }

    for state in states {
        let mut cache = HashSet::new();
        let mut energized: Vec<Vec<_>> = input
            .lines()
            .map(|l| l.chars().map(|_| false).collect())
            .collect();
        solve(&field, &mut energized, &mut cache, state);
        let count = energized.iter().flatten().filter(|&&b| b).count();
        if count > max {
            max = count;
        }
    }

    println!("{}", max);
}
