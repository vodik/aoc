use std::collections::HashSet;

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
    verticies: Vec<Vertex>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn from(graph: &[(Vertex, Vertex)]) -> Self {
        let mut verticies = Vec::new();
        for (a, b) in graph {
            if !verticies.contains(a) {
                verticies.push(a.clone());
            }
            if !verticies.contains(b) {
                verticies.push(b.clone());
            }
        }

        let mut edges = vec![Vec::new(); verticies.len()];
        for (a, b) in graph {
            let a_idx = verticies.iter().position(|v| v == a).unwrap();
            let b_idx = verticies.iter().position(|v| v == b).unwrap();

            edges[a_idx].push(b_idx);
            edges[b_idx].push(a_idx);
        }

        Self { verticies, edges }
    }
}

fn paths(graph: &Graph, vertex: usize, mut visited: HashSet<usize>, charge: bool) -> usize {
    match &graph.verticies[vertex] {
        Vertex::Start => 0,
        Vertex::End => 1,
        Vertex::BigCave(_) => graph.edges[vertex]
            .iter()
            .map(|&v| paths(graph, v, visited.clone(), charge))
            .sum(),
        Vertex::SmallCave(_) => {
            if !visited.insert(vertex) {
                if charge {
                    graph.edges[vertex]
                        .iter()
                        .map(|&v| paths(graph, v, visited.clone(), false))
                        .sum()
                } else {
                    0
                }
            } else {
                graph.edges[vertex]
                    .iter()
                    .map(|&v| paths(graph, v, visited.clone(), charge))
                    .sum()
            }
        }
    }
}

pub fn part1(input: &[(Vertex, Vertex)]) -> usize {
    let graph = Graph::from(input);

    let start = graph
        .verticies
        .iter()
        .position(|v| matches!(v, Vertex::Start))
        .unwrap();

    graph.edges[start]
        .iter()
        .map(|&v| paths(&graph, v, HashSet::new(), false))
        .sum()
}

pub fn part2(input: &[(Vertex, Vertex)]) -> usize {
    let graph = Graph::from(input);

    let start = graph
        .verticies
        .iter()
        .position(|v| matches!(v, Vertex::Start))
        .unwrap();

    graph.edges[start]
        .iter()
        .map(|&v| paths(&graph, v, HashSet::new(), true))
        .sum()
}
