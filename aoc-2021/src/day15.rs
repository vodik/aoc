use std::collections::{hash_map::Entry, BinaryHeap, HashMap};

const WIDTH: usize = 100;

type NodeIdx = usize;
type Vertex = u8;

#[derive(Debug, Default)]
pub struct Graph {
    vertices: Vec<(Vertex, Vec<NodeIdx>)>,
}

impl FromIterator<(Vertex, Vertex)> for Graph {
    fn from_iter<T: IntoIterator<Item = (Vertex, Vertex)>>(iter: T) -> Self {
        let mut graph = Self::default();
        let mut indicies = HashMap::new();

        for (a, b) in iter {
            let a_idx = *indicies
                .entry(a.clone())
                .or_insert_with(|| graph.add_node(a));
            let b_idx = *indicies
                .entry(b.clone())
                .or_insert_with(|| graph.add_node(b));

            graph.add_edge(a_idx, b_idx);
        }

        graph
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cell(NodeIdx, usize);

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1).map(|x| x.reverse())
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }

    fn add_node(&mut self, vertex: Vertex) -> NodeIdx {
        let index = self.vertices.len();
        self.vertices.push((vertex, Vec::new()));
        index
    }

    fn add_edge(&mut self, a: NodeIdx, b: NodeIdx) {
        self.vertices[a].1.push(b);
        self.vertices[b].1.push(a);
    }

    // fn vertex(&self, index: NodeIdx) -> &Vertex {
    //     &self.vertices[index].0
    // }

    // fn edges(&self, index: NodeIdx) -> impl Iterator<Item = &'_ NodeIdx> + '_ {
    //     self.vertices[index].1.iter()
    // }

    fn djikstra(&self, start: NodeIdx, goal: NodeIdx, board: &[u8], width: usize) -> usize {
        let mut heap = BinaryHeap::new();
        heap.push(Cell(start, 0));

        let mut came_from = HashMap::with_capacity(goal); // last node
        came_from.insert(start, None);

        let mut cost_so_far = HashMap::with_capacity(goal);
        cost_so_far.insert(start, 0usize);

        while let Some(Cell(idx, _)) = heap.pop() {
            if idx == goal {
                break;
            }

            for &next in neighbours2(idx, width).iter().flatten() {
                let new_cost: usize = cost_so_far[&idx] + board[next] as usize;
                let entry = cost_so_far.entry(next);
                match entry {
                    Entry::Occupied(mut entry) => {
                        if new_cost < *entry.get() {
                            entry.insert(new_cost);
                            let priority = new_cost;
                            heap.push(Cell(next, priority));
                            came_from.insert(next, Some(idx));
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(new_cost);
                        let priority = new_cost;
                        heap.push(Cell(next, priority));
                        came_from.insert(next, Some(idx));
                    }
                }
            }
        }

        // let mut x = goal;
        // while let Some(&Some(prev)) = came_from.get(&x) {
        //     eprint!(" <- {} ({})", board[prev], cost_so_far[&prev]);
        //     x = prev;
        // }
        // eprintln!();

        cost_so_far[&goal]
    }
}

fn neighbours(point: usize, width: usize) -> [Option<usize>; 2] {
    [
        point.checked_add(1).filter(|p| p % width != 0),
        point.checked_add(width).filter(|&p| p < width * width),
    ]
}

fn neighbours2(point: usize, width: usize) -> [Option<usize>; 4] {
    [
        point.checked_sub(width),
        point.checked_sub(1).filter(|p| p % width != width - 1),
        point.checked_add(1).filter(|p| p % width != 0),
        point.checked_add(width).filter(|&p| p < width * width),
    ]
}

pub fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|char| u8::try_from(char.to_digit(10).unwrap()).unwrap())
        })
        .collect()
}

fn solve(board: &[u8], width: usize) -> usize {
    let mut graph = Graph::new();
    board.iter().for_each(|&height| {
        graph.add_node(height);
    });

    for pos in 0..board.len() {
        for &n in neighbours(pos, width).iter().flatten() {
            graph.add_edge(pos, n);
        }
    }

    graph.djikstra(0, board.len() - 1, board, width)
}

pub fn part1(input: &[u8]) -> usize {
    solve(input, WIDTH)
}

pub fn part2(input: &[u8]) -> usize {
    // let mut board = vec![0; input.len() * 25];
    let mut board = Vec::with_capacity(input.len() * 25);
    for y in 0..WIDTH * 5 {
        for x in 0..WIDTH * 5 {
            let source = x % WIDTH + (y % WIDTH) * WIDTH;
            // let dest = x + y * WIDTH;
            let factor = x / WIDTH + y / WIDTH;
            // board[dest] = (input[source] + factor as u8) % 10;
            let v = input[source] + factor as u8;
            let v = (v - 1) % 9 + 1;
            // board[dest] = v;
            board.push(v);
        }
    }

    solve(&board, WIDTH * 5)
}
