const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
const NOPE: (i32, i32) = (0, 0);

pub fn p1() {
    let input = crate::read_file(10);

    let mut graph = vec![];
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, ch) in line.chars().enumerate() {
            row.push(match ch {
                '|' => [UP, DOWN],
                '-' => [LEFT, RIGHT],
                'L' => [UP, RIGHT],
                'J' => [UP, LEFT],
                '7' => [DOWN, LEFT],
                'F' => [DOWN, RIGHT],
                _ => [NOPE, NOPE],
            });

            if ch == 'S' {
                start = (x, y);
            }
        }
        graph.push(row);
    }

    let mut steps = 1;
    let mut last_dir = (0, 0);
    let mut pos = start;

    for (dx, dy) in [UP, RIGHT, DOWN, LEFT] {
        let (x, y) = (start.0 as i32 + dx, start.1 as i32 + dy);
        if !(0 <= x && (x as usize) < graph[0].len() && 0 <= y && (y as usize) < graph.len()) {
            continue;
        }

        let dirs = graph[y as usize][x as usize];
        if dirs[0] == (-dx, -dy) || dirs[1] == (-dx, -dy) {
            graph[pos.1][pos.0][0] = (dx, dy);
            last_dir = (-dx, -dy);
            pos = (x as usize, y as usize);
            break;
        }
    }

    while pos != start {
        let dirs = graph[pos.1][pos.0];
        let next = if dirs[0] == last_dir { 1 } else { 0 };
        pos = (
            (pos.0 as i32 + dirs[next].0) as usize,
            (pos.1 as i32 + dirs[next].1) as usize,
        );
        last_dir = (-dirs[next].0, -dirs[next].1);
        steps += 1
    }

    graph[pos.1][pos.0][1] = last_dir;
    println!("{}", steps / 2);
}

pub fn p2() {
    let input = crate::read_file(10);

    let mut graph = vec![];
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, ch) in line.chars().enumerate() {
            row.push(match ch {
                '|' => [UP, DOWN],
                '-' => [LEFT, RIGHT],
                'L' => [UP, RIGHT],
                'J' => [UP, LEFT],
                '7' => [DOWN, LEFT],
                'F' => [DOWN, RIGHT],
                _ => [NOPE, NOPE],
            });

            if ch == 'S' {
                start = (x, y);
            }
        }
        graph.push(row);
    }

    let mut last_dir = (0, 0);
    let mut pos = start;

    for (dx, dy) in [UP, RIGHT, DOWN, LEFT] {
        let (x, y) = (start.0 as i32 + dx, start.1 as i32 + dy);
        if !(0 <= x && (x as usize) < graph[0].len() && 0 <= y && (y as usize) < graph.len()) {
            continue;
        }

        let dirs = graph[y as usize][x as usize];
        if dirs[0] == (-dx, -dy) || dirs[1] == (-dx, -dy) {
            graph[start.1][start.0][0] = (dx, dy);
            last_dir = (-dx, -dy);
            pos = (x as usize, y as usize);
            break;
        }
    }

    let mut loop_tiles = std::collections::HashSet::from([start]);

    while pos != start {
        loop_tiles.insert(pos);
        let dirs = graph[pos.1][pos.0];
        let next = if dirs[0] == last_dir { 1 } else { 0 };
        pos = (
            (pos.0 as i32 + dirs[next].0) as usize,
            (pos.1 as i32 + dirs[next].1) as usize,
        );
        last_dir = (-dirs[next].0, -dirs[next].1);
    }

    graph[start.1][start.0][1] = last_dir;

    let mut count = 0;
    for y in 0..graph.len() {
        let mut inside = false;
        for x in 0..graph[0].len() {
            if loop_tiles.contains(&(x, y)) {
                if graph[y][x].contains(&UP) {
                    inside = !inside;
                }
            } else if inside {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
