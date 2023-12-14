pub fn p1() {
    let input = crate::read_file(14);
    let matrix: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = matrix.len();
    let mut sum = 0;

    for c in 0..matrix[0].len() {
        let mut empty = 0;
        for (r, row) in matrix.iter().enumerate() {
            match row[c] {
                '.' => empty += 1,
                'O' => sum += rows - r + empty,
                _ => empty = 0,
            }
        }
    }

    println!("{}", sum);
}

pub fn p2() {
    let input = crate::read_file(14);
    let mut matrix: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut cache = std::collections::HashMap::new();

    let mut cycle_len = 0;
    let mut cycle_start = 0;

    let mut saved_pos = vec![];
    for i in 0..1000000000 {
        let positions = cycle(&mut matrix);

        if let Some(&n) = cache.get(&positions) {
            cycle_start = n;
            cycle_len = i - n;
            saved_pos = positions;
            break;
        }

        cache.insert(positions, i);
    }

    for _ in 0..(1000000000 - cycle_start - 1) % cycle_len {
        saved_pos = cycle(&mut matrix);
    }

    let rows = matrix.len();
    let sum = saved_pos.iter().fold(0, |acc, (r, _)| acc + rows - r);
    println!("{}", sum);
}

fn cycle(matrix: &mut Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // north
    for c in 0..cols {
        let mut empty = 0;
        for r in 0..rows {
            match matrix[r][c] {
                '.' => empty += 1,
                'O' if empty > 0 => {
                    matrix[r - empty][c] = 'O';
                    matrix[r][c] = '.';
                }
                _ => empty = 0,
            }
        }
    }

    // west
    for row in matrix.iter_mut() {
        let mut empty = 0;
        for c in 0..cols {
            match row[c] {
                '.' => empty += 1,
                'O' if empty > 0 => {
                    row[c - empty] = 'O';
                    row[c] = '.';
                }
                _ => empty = 0,
            }
        }
    }

    // south
    for c in 0..cols {
        let mut empty = 0;
        for r in (0..rows).rev() {
            match matrix[r][c] {
                '.' => empty += 1,
                'O' if empty > 0 => {
                    matrix[r + empty][c] = 'O';
                    matrix[r][c] = '.';
                }
                _ => empty = 0,
            }
        }
    }

    // save positions of rolling stones
    let mut positions = vec![];
    // east
    for (r, row) in matrix.iter_mut().enumerate() {
        let mut empty = 0;
        for c in (0..cols).rev() {
            match row[c] {
                '.' => empty += 1,
                'O' => {
                    if empty > 0 {
                        row[c + empty] = 'O';
                        row[c] = '.';
                        positions.push((r, c + empty));
                    } else {
                        positions.push((r, c))
                    }
                }
                _ => empty = 0,
            }
        }
    }

    positions
}
