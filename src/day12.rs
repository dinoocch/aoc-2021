use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day12 {}

#[derive(Default, Debug, Clone, Copy)]
pub struct Node {
    is_small: bool,
}

pub struct Graph<'a> {
    edges: HashMap<&'a str, HashSet<&'a str>>,
    nodes: HashMap<&'a str, Node>,
}

#[derive(Debug, Clone)]
struct Visited<'a> {
    last_node: &'a str,
    seen: HashSet<&'a str>,
    can_revisit_small: bool,
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(|c| c.is_lowercase())
}

impl<'a> Graph<'a> {
    fn from_input(input: &'a str) -> Graph<'a> {
        let mut nodes = HashMap::new();
        let mut edges = HashMap::new();
        input
            .lines()
            .map(|line| line.split_once("-"))
            .flatten()
            .for_each(|(a, b)| {
                nodes.insert(
                    a,
                    Node {
                        is_small: is_small_cave(a),
                    },
                );
                nodes.insert(
                    b,
                    Node {
                        is_small: is_small_cave(b),
                    },
                );

                (*edges.entry(a).or_insert_with(HashSet::new)).insert(b);
                (*edges.entry(b).or_insert_with(HashSet::new)).insert(a);
            });
        Graph { edges, nodes }
    }

    fn paths(&self, can_revisit_small: bool) -> u64 {
        let mut seen = HashSet::new();
        seen.insert("start");

        let mut current = vec![Visited {
            last_node: "start",
            seen,
            can_revisit_small,
        }];

        let mut paths = 0;
        loop {
            if current.len() == 0 {
                break;
            }
            let mut next: Vec<Visited> = Vec::new();
            for current_path in current.iter() {
                if current_path.last_node == "end" {
                    paths += 1;
                    continue;
                }

                for neighbor in self.edges.get(current_path.last_node).unwrap().iter() {
                    if *neighbor == "start" {
                        continue;
                    }
                    let mut can_revisit = current_path.can_revisit_small;
                    if self.nodes.get(neighbor).unwrap().is_small
                        && current_path.seen.contains(neighbor)
                    {
                        if !can_revisit {
                            continue;
                        }
                        can_revisit = false;
                    }
                    // idk how to remove this clone :/
                    let mut next_seen = current_path.seen.clone();
                    next_seen.insert(neighbor);
                    next.push(Visited {
                        last_node: neighbor,
                        seen: next_seen,
                        can_revisit_small: can_revisit,
                    });
                }
            }
            current = next;
        }
        paths
    }
}

impl crate::aoc::AoCSolution for Day12 {
    type ConvertedType = String;
    type ReturnType = u64;

    const DAY: usize = 12;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input.to_owned()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        // The lifetime is annoying so eat the cost twice
        let graph = Graph::from_input(input);
        graph.paths(false)
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        // The lifetime is annoying so eat the cost twice
        let graph = Graph::from_input(input);
        graph.paths(true)
    }
}
