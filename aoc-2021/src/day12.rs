use std::collections::HashMap;

type NodeIdx = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Vertex {
    Start,
    End,
    BigCave(String),
    SmallCave(String),
}

impl Vertex {
    fn from(s: &str) -> Self {
        match s {
            "start" => Vertex::Start,
            "end" => Vertex::End,
            label if label.bytes().all(|b| b.is_ascii_uppercase()) => {
                Vertex::BigCave(label.to_string())
            }
            label if label.bytes().all(|b| b.is_ascii_lowercase()) => {
                Vertex::SmallCave(label.to_string())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Graph {
    vertices: Vec<(Vertex, Vec<NodeIdx>)>,
    start: Option<NodeIdx>,
}

impl FromIterator<(Vertex, Vertex)> for Graph {
    fn from_iter<T: IntoIterator<Item = (Vertex, Vertex)>>(iter: T) -> Self {
        let mut graph = Self::default();
        let mut indicies = HashMap::new();

        for (a, b) in iter {
            let is_start = matches!(&a, Vertex::Start);

            let a_idx = *indicies
                .entry(a.clone())
                .or_insert_with(|| graph.add_node(a));
            let b_idx = *indicies
                .entry(b.clone())
                .or_insert_with(|| graph.add_node(b));

            graph.add_edge(a_idx, b_idx);

            if is_start {
                graph.start = Some(a_idx);
            }
        }

        graph
    }
}

impl Graph {
    fn add_node(&mut self, vertex: Vertex) -> NodeIdx {
        let index = self.vertices.len();
        self.vertices.push((vertex, Vec::new()));
        index
    }

    fn add_edge(&mut self, a: NodeIdx, b: NodeIdx) {
        self.vertices[a].1.push(b);
        self.vertices[b].1.push(a);
    }

    fn vertex(&self, index: NodeIdx) -> &Vertex {
        &self.vertices[index].0
    }

    fn edges(&self, index: NodeIdx) -> impl Iterator<Item = &'_ NodeIdx> + '_ {
        self.vertices[index].1.iter()
    }

    fn paths_from(&self, index: NodeIdx, revisit: usize) -> usize {
        let mut stack = Vec::with_capacity(30);
        stack.extend(self.edges(index).map(|&vertex| (vertex, 0u32, revisit)));

        let mut paths = 0;
        while let Some((index, visited, revisit)) = stack.pop() {
            match self.vertex(index) {
                Vertex::Start => {}
                Vertex::End => {
                    paths += 1;
                }
                Vertex::BigCave(_) => {
                    stack.extend(self.edges(index).map(|&succ| (succ, visited, revisit)))
                }
                Vertex::SmallCave(_) => {
                    let mask = 1 << index;
                    if visited & mask != 0 {
                        if revisit != 0 {
                            stack.extend(
                                self.edges(index).map(|&succ| (succ, visited, revisit - 1)),
                            );
                        }
                    } else {
                        stack.extend(
                            self.edges(index)
                                .map(|&succ| (succ, visited | mask, revisit)),
                        );
                    }
                }
            }
        }
        paths
    }
}

pub fn parse_input(input: &str) -> Graph {
    input
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_once('-')?;
            Some((Vertex::from(a), Vertex::from(b)))
        })
        .collect()
}

pub fn part1(graph: &Graph) -> usize {
    graph.paths_from(graph.start.unwrap(), 0)
}

pub fn part2(graph: &Graph) -> usize {
    graph.paths_from(graph.start.unwrap(), 1)
}
