use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Node {
    x: usize,
    y: usize,
    dir: Option<Dir>,
    steps: usize,
    prev: Option<Rc<Node>>,
}

impl Node {
    fn new(x: usize, y: usize, dir: Option<Dir>) -> Self {
        let steps = 1;
        let prev = None;
        Node {
            x,
            y,
            dir,
            steps,
            prev,
        }
    }
}

pub fn p1() {
    let input = crate::read_file(17);
    let graph: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let distance = dijkstra(
        Node::new(0, 0, None),
        Node::new(graph[0].len() - 1, graph.len() - 1, None),
        &graph,
        0,
        3,
    );
    println!("{}\n", distance);
}

fn dijkstra(
    mut start: Node,
    goal: Node,
    graph: &Vec<Vec<usize>>,
    min_steps: usize,
    max_steps: usize,
) -> usize {
    start.steps = 0;
    let mut dists = vec![vec![HashMap::new(); graph[0].len()]; graph.len()];
    let mut heap = BinaryHeap::from([(Reverse(0), Rc::new(start))]);

    while !heap.is_empty() {
        let (Reverse(dist), node) = heap.pop().unwrap();

        if node.x == goal.x && node.y == goal.y {
            draw_path(&node, graph);
            return dist;
        }

        let mut neighbors = vec![
            Node::new(node.x, node.y.wrapping_sub(1), Some(Dir::Up)),
            Node::new(node.x, node.y + 1, Some(Dir::Down)),
            Node::new(node.x.wrapping_sub(1), node.y, Some(Dir::Left)),
            Node::new(node.x + 1, node.y, Some(Dir::Right)),
        ];

        match node.dir {
            None => (),
            Some(Dir::Up) => {
                neighbors[0].steps += node.steps;
                neighbors[1].steps = usize::MAX;
                if node.steps < min_steps {
                    neighbors[2].steps = usize::MAX;
                    neighbors[3].steps = usize::MAX;
                }
            }
            Some(Dir::Down) => {
                neighbors[1].steps += node.steps;
                neighbors[0].steps = usize::MAX;
                if node.steps < min_steps {
                    neighbors[2].steps = usize::MAX;
                    neighbors[3].steps = usize::MAX;
                }
            }
            Some(Dir::Left) => {
                neighbors[2].steps += node.steps;
                neighbors[3].steps = usize::MAX;
                if node.steps < min_steps {
                    neighbors[0].steps = usize::MAX;
                    neighbors[1].steps = usize::MAX;
                }
            }
            Some(Dir::Right) => {
                neighbors[3].steps += node.steps;
                neighbors[2].steps = usize::MAX;
                if node.steps < min_steps {
                    neighbors[0].steps = usize::MAX;
                    neighbors[1].steps = usize::MAX;
                }
            }
        }

        for mut nei in neighbors {
            if nei.x >= graph[0].len() || nei.y >= graph.len() || nei.steps > max_steps {
                continue;
            }

            let score = dist + graph[nei.y][nei.x];
            let curr = dists[nei.y][nei.x]
                .entry((nei.dir.unwrap(), nei.steps))
                .or_insert(usize::MAX);
            if score < *curr {
                *curr = score;
                nei.prev = Some(node.clone());
                heap.push((Reverse(score), Rc::new(nei)));
            }
        }
    }

    unreachable!();
}

fn draw_path(end: &Rc<Node>, graph: &Vec<Vec<usize>>) {
    let mut drawing = vec![vec!['.'; graph[0].len()]; graph.len()];
    let mut node = end;
    while let Some(prev) = &node.prev {
        match node.dir {
            None => (),
            Some(Dir::Up) => drawing[node.y][node.x] = '^',
            Some(Dir::Down) => drawing[node.y][node.x] = 'v',
            Some(Dir::Left) => drawing[node.y][node.x] = '<',
            Some(Dir::Right) => drawing[node.y][node.x] = '>',
        }
        node = prev;
    }

    for (i, row) in drawing.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '.' {
                print!("{}", (graph[i][j] as u8 + b'0') as char);
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }
}

pub fn p2() {
    let input = crate::read_file(17);
    let graph: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let distance = dijkstra(
        Node::new(0, 0, None),
        Node::new(graph[0].len() - 1, graph.len() - 1, None),
        &graph,
        4,
        10,
    );
    println!("{}", distance);
}
