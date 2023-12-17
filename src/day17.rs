use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    dir: Option<Dir>,
    steps: usize,
}

impl Node {
    fn new(x: usize, y: usize) -> Self {
        let steps = 1;
        let dir = None;
        Node { x, y, dir, steps }
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
        Node::new(0, 0),
        Node::new(graph[0].len() - 1, graph.len() - 1),
        &graph,
    );
    println!("{}", distance);
}

fn dijkstra(start: Node, goal: Node, graph: &Vec<Vec<usize>>) -> usize {
    let mut heap = BinaryHeap::from([(Reverse(0), start)]);
    let mut dists = vec![vec![usize::MAX; graph[0].len()]; graph.len()];
    dists[start.y][start.x] = 0;

    while !heap.is_empty() {
        let (Reverse(dist), node) = heap.pop().unwrap();

        if node.x == goal.x && node.y == goal.y {
            return dist;
        }

        let neighbors = [
            Node::new(node.x + 1, node.y),
            Node::new(node.x, node.y + 1),
            Node::new(node.x.wrapping_sub(1), node.y),
            Node::new(node.x, node.y.wrapping_sub(1)),
        ];

        for nei in neighbors {
            if nei.x >= graph.len() || nei.y >= graph.len() {
                continue;
            }

            let score = dist + graph[nei.y][nei.x];
            if score < dists[nei.y][nei.x] {
                dists[nei.y][nei.x] = score;
                heap.push((Reverse(score), nei));
            }
        }
    }

    0
}

pub fn p2() {}
