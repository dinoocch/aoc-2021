use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day12 {}

impl crate::aoc::AoCSolution for Day12 {
    type ConvertedType = Vec<(String, String)>;
    type ReturnType = u64;

    const DAY: usize = 12;

    fn convert(&self, input: &str) -> Self::ConvertedType {
        input
            .lines()
            .map(|line| line.split_once("-"))
            .flatten()
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect()
    }

    fn part1(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();
        for (a, b) in input.iter() {
            {
                let node_a = nodes.entry(a.to_string()).or_insert(HashSet::new());
                (*node_a).insert(b.to_string());
            }
            let node_b = nodes.entry(b.to_string()).or_insert(HashSet::new());
            (*node_b).insert(a.to_string());
        }

        let mut paths = 0;
        let mut current = vec![vec!["start".to_string()]];
        loop {
            // Generate all the paths for each value in current
            if current.len() == 0 {
                break;
            }
            let mut next: Vec<Vec<String>> = Vec::new();
            for value in current.iter_mut() {
                let last_node = &value[value.len() - 1];
                if last_node == "end" {
                    paths += 1;
                    continue;
                }
                let mut visited_small: HashSet<String> = HashSet::new();
                for visited in value.iter() {
                    if small_cave(visited) {
                        visited_small.insert(visited.to_string());
                    }
                }
                for neighbor in nodes.get(last_node).unwrap().iter() {
                    if small_cave(neighbor) && visited_small.contains(neighbor) {
                        continue;
                    }
                    let mut new_vec: Vec<String> =
                        Vec::from_iter(value.iter().map(|x| x.to_string()));
                    new_vec.push(neighbor.to_string());
                    next.push(new_vec);
                }
            }
            current = next;
        }
        paths
    }

    fn part2(&self, input: &Self::ConvertedType) -> Self::ReturnType {
        let mut nodes: HashMap<String, HashSet<String>> = HashMap::new();
        for (a, b) in input.iter() {
            {
                let node_a = nodes.entry(a.to_string()).or_insert(HashSet::new());
                (*node_a).insert(b.to_string());
            }
            let node_b = nodes.entry(b.to_string()).or_insert(HashSet::new());
            (*node_b).insert(a.to_string());
        }

        let mut paths = 0;
        let mut current = vec![(vec!["start".to_string()], true)];
        loop {
            // Generate all the paths for each value in current
            if current.len() == 0 {
                break;
            }
            let mut next: Vec<(Vec<String>, bool)> = Vec::new();
            for value in current.iter_mut() {
                let last_node = &value.0[value.0.len() - 1];
                if last_node == "end" {
                    paths += 1;
                    continue;
                }
                let mut visited_small: HashSet<String> = HashSet::new();
                for visited in value.0.iter() {
                    if small_cave(visited) {
                        visited_small.insert(visited.to_string());
                    }
                }
                for neighbor in nodes.get(last_node).unwrap().iter() {
                    if neighbor == "start" {
                        continue;
                    }
                    let mut can_repeat = value.1;
                    if small_cave(neighbor) && visited_small.contains(neighbor) {
                        if !value.1 {
                            continue;
                        }
                        can_repeat = false;
                    }
                    let mut new_vec: Vec<String> =
                        Vec::from_iter(value.0.iter().map(|x| x.to_string()));
                    new_vec.push(neighbor.to_string());
                    next.push((new_vec, can_repeat));
                }
            }
            current = next;
        }
        paths
    }
}

fn small_cave(name: &str) -> bool {
    name.chars().all(|c| c.is_lowercase())
}
