struct Mapping {
    src: Range,
    dest: Range,
}

impl Mapping {
    fn map(&self, from: Range) -> Vec<Range> {
        let mut output = vec![];

        if self.src.start <= from.start && self.src.end < from.end {
            // map from left to middle of range
            output.push(Range {
                start: self.dest.start + from.start - self.src.start,
                end: self.dest.end,
            });
            output.push(Range {
                start: self.src.end,
                end: from.end,
            });
        } else if self.src.start <= from.start && from.end <= self.src.end {
            // map wider than whole range
            output.push(Range {
                start: self.dest.start + from.start - self.src.start,
                end: self.dest.start + from.end - self.src.start,
            });
        } else if self.src.start < from.end && from.end < self.src.end {
            // map from middle to the right of range
            output.push(Range {
                start: from.start,
                end: self.src.start,
            });
            output.push(Range {
                start: self.dest.start,
                end: self.dest.start + from.end - self.src.start,
            });
        } else {
            // map in the middle of the range
            output.push(Range {
                start: from.start,
                end: self.src.start,
            });
            output.push(Range {
                start: self.dest.start,
                end: self.dest.end,
            });
            output.push(Range {
                start: self.src.end,
                end: from.end,
            });
        }

        output
    }
}

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}

fn get_maps(input: &str) -> Vec<Vec<Mapping>> {
    let mut maps = vec![];

    for line in input.lines().skip(2) {
        if line.ends_with(':') {
            // new map
            maps.push(vec![]);
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let values: Vec<u64> = line.split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
        let mapping = Mapping {
            src: Range {
                start: values[1],
                end: values[1] + values[2],
            },
            dest: Range {
                start: values[0],
                end: values[0] + values[2],
            },
        };

        maps.last_mut().unwrap().push(mapping);
    }

    for map in &mut maps {
        map.sort_by(|m1, m2| m1.src.start.cmp(&m2.src.start))
    }

    maps
}

pub fn p1() {
    let input = crate::read_file(5);
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap());

    let maps = get_maps(&input);
    let mut lowest_location = u64::MAX;

    for seed in seeds {
        let mut current = seed;

        for map in &maps {
            for mapping in map {
                if mapping.src.start <= current && current < mapping.src.end {
                    current = mapping.dest.start + current - mapping.src.start;
                    break;
                }
            }
        }

        if current < lowest_location {
            lowest_location = current;
        }
    }

    println!("{}", lowest_location);
}

pub fn p2() {
    let input = crate::read_file(5);
    let iter = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap());
    let mut ranges: Vec<Range> = iter
        .clone()
        .step_by(2)
        .zip(iter.skip(1).step_by(2))
        .map(|(start, count)| Range {
            start,
            end: start + count,
        })
        .collect();
    let maps = get_maps(&input);

    for map in &maps {
        let mut new_ranges: Vec<Range> = vec![];

        for range in &ranges {
            new_ranges.push(range.clone());

            for mapping in map {
                if mapping.src.end < range.start {
                    // map is not touching range
                    continue;
                }

                if mapping.src.start >= range.end {
                    // map is after range, no need to look further
                    break;
                }

                // dividing range
                let current = new_ranges.pop().unwrap();
                new_ranges.extend(mapping.map(current));
            }
        }

        ranges = new_ranges;
    }

    let minimal = ranges
        .iter()
        .min_by(|x, y| x.start.cmp(&y.start))
        .unwrap()
        .start;
    println!("{}", minimal);
}
