use std::collections::BinaryHeap;

const WIDTH: usize = 100;

type NodeIdx = usize;

pub fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|char| u8::try_from(char.to_digit(10).unwrap()).unwrap())
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct NodeCost(NodeIdx, usize);

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1).map(|x| x.reverse())
    }
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn neighbours(point: usize, width: usize) -> [Option<usize>; 4] {
    [
        point.checked_sub(width),
        point.checked_sub(1).filter(|p| p % width != width - 1),
        point.checked_add(1).filter(|p| p % width != 0),
        point.checked_add(width).filter(|&p| p < width * width),
    ]
}

fn djikstra(start: NodeIdx, goal: NodeIdx, board: &[u8], width: usize) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(NodeCost(start, 0));

    let mut costs = vec![0; board.len()];
    while let Some(NodeCost(index, cost)) = heap.pop() {
        if index == goal {
            break;
        }

        heap.extend(
            neighbours(index, width)
                .iter()
                .flatten()
                .filter_map(|&next| {
                    let new_cost = cost + board[next] as usize;
                    let current_cost = costs[next];
                    (current_cost == 0 || new_cost < current_cost).then(|| {
                        costs[next] = new_cost;
                        NodeCost(next, new_cost)
                    })
                }),
        );
    }

    costs[goal]
}

fn solve(board: &[u8], width: usize) -> usize {
    djikstra(0, board.len() - 1, board, width)
}

pub fn part1(input: &[u8]) -> usize {
    solve(input, WIDTH)
}

pub fn part2(input: &[u8]) -> usize {
    let mut board = Vec::with_capacity(input.len() * 25);

    for y in 0..WIDTH * 5 {
        for x in 0..WIDTH * 5 {
            let source = x % WIDTH + (y % WIDTH) * WIDTH;
            let factor = x / WIDTH + y / WIDTH;

            let v = input[source] + factor as u8;
            let v = (v - 1) % 9 + 1;
            board.push(v);
        }
    }

    solve(&board, WIDTH * 5)
}
