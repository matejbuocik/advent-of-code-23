fn solution(note: &Vec<Vec<char>>, avoid: Option<usize>) -> usize {
    let avoid = avoid.unwrap_or_default();

    // horizontal line
    for hor in 0..note.len() - 1 {
        if line_ok(note, hor) && (hor + 1) * 100 != avoid {
            return (hor + 1) * 100;
        }
    }

    // vertical line
    let mut transponed: Vec<Vec<char>> = (0..note[0].len()).map(|_| vec![]).collect();
    for row in note {
        for (j, col) in transponed.iter_mut().enumerate() {
            col.push(row[j]);
        }
    }

    for vert in 0..note[0].len() - 1 {
        if line_ok(&transponed, vert) && vert + 1 != avoid {
            return vert + 1;
        }
    }

    0
}

fn line_ok(line: &Vec<Vec<char>>, vert: usize) -> bool {
    let mut i = vert;
    let mut j = vert + 1;

    while i < line.len() && j < line.len() {
        if line[i] != line[j] {
            return false;
        }

        i = i.wrapping_sub(1);
        j += 1;
    }

    true
}

pub fn p1() {
    let input = crate::read_file(13);
    let notes: Vec<Vec<Vec<_>>> = input
        .split("\n\n")
        .map(|n| n.split('\n').map(|l| l.chars().collect()).collect())
        .collect();
    let mut sum = 0;

    for note in &notes {
        sum += solution(note, None);
    }

    println!("{}", sum);
}

pub fn p2() {
    let input = crate::read_file(13);
    let notes: Vec<Vec<Vec<_>>> = input
        .split("\n\n")
        .map(|n| n.split('\n').map(|l| l.chars().collect()).collect())
        .collect();
    let mut sum = 0;

    let mut og_lines = vec![];
    for note in &notes {
        og_lines.push(solution(note, None));
    }

    'notefor: for (index, mut note) in notes.into_iter().enumerate() {
        for i in 0..note.len() {
            for j in 0..note[0].len() {
                note[i][j] = if note[i][j] == '#' { '.' } else { '#' };

                let n = solution(&note, Some(og_lines[index]));
                if n > 0 {
                    sum += n;
                    continue 'notefor;
                }

                note[i][j] = if note[i][j] == '#' { '.' } else { '#' };
            }
        }
    }

    println!("{}", sum);
}
