use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vertex {
    Start,
    End,
    BigCave(String),
    SmallCave(String),
}

impl Vertex {
    fn from(s: &str) -> Option<Self> {
        match s {
            "start" => Some(Vertex::Start),
            "end" => Some(Vertex::End),
            s if s.bytes().all(|b| (b'A'..=b'Z').contains(&b)) => {
                Some(Vertex::BigCave(s.to_string()))
            }
            s if s.bytes().all(|b| (b'a'..=b'z').contains(&b)) => {
                Some(Vertex::SmallCave(s.to_string()))
            }
            _ => None,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<(Vertex, Vertex)> {
    input
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_once('-')?;
            Some((Vertex::from(a).unwrap(), Vertex::from(b).unwrap()))
        })
        .collect()
}

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn from(graph: &[(Vertex, Vertex)]) -> Self {
        let mut vertices = Vec::new();
        for (a, b) in graph {
            if !vertices.contains(a) {
                vertices.push(a.clone());
            }
            if !vertices.contains(b) {
                vertices.push(b.clone());
            }
        }

        let mut edges = vec![Vec::new(); vertices.len()];
        for (a, b) in graph {
            let a_idx = vertices.iter().position(|v| v == a).unwrap();
            let b_idx = vertices.iter().position(|v| v == b).unwrap();

            edges[a_idx].push(b_idx);
            edges[b_idx].push(a_idx);
        }

        Self { vertices, edges }
    }
}

fn paths(graph: &Graph, vertex: usize, charge: bool) -> usize {
    let mut paths = 0;
    let visited = Rc::new(vec![false; graph.verticies.len()]);
    let mut stack: Vec<(usize, Rc<Vec<bool>>, bool)> = graph.edges[vertex]
        .iter()
        .map(|&vertex| (vertex, visited.clone(), charge))
        .collect();

    while let Some((vertex, visited, charge)) = stack.pop() {
        match &graph.vertices[vertex] {
            Vertex::Start => {}
            Vertex::End => {
                paths += 1;
            }
            Vertex::BigCave(_) => stack.extend(
                graph.edges[vertex]
                    .iter()
                    .map(|&vertex| (vertex, visited.clone(), charge)),
            ),
            Vertex::SmallCave(_) => {
                if visited[vertex] {
                    if charge {
                        stack.extend(
                            graph.edges[vertex]
                                .iter()
                                .map(|&vertex| (vertex, visited.clone(), false)),
                        );
                    }
                } else {
                    let mut visited = Vec::clone(&visited);
                    visited[vertex] = true;
                    let visited = Rc::new(visited);

                    stack.extend(
                        graph.edges[vertex]
                            .iter()
                            .map(|&vertex| (vertex, visited.clone(), charge)),
                    );
                }
            }
        }
    }

    paths
}

pub fn part1(input: &[(Vertex, Vertex)]) -> usize {
    let graph = Graph::from(input);

    let start = graph
        .vertices
        .iter()
        .position(|v| matches!(v, Vertex::Start))
        .unwrap();

    paths(&graph, start, false)
}

pub fn part2(input: &[(Vertex, Vertex)]) -> usize {
    let graph = Graph::from(input);

    let start = graph
        .vertices
        .iter()
        .position(|v| matches!(v, Vertex::Start))
        .unwrap();

    paths(&graph, start, true)
}
