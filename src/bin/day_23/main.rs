use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn get_k_plus_1<'a>(k_graphs: &HashSet<Vec<&'a str>>, links: &HashMap<&str, HashSet<&'a str>>) -> HashSet<Vec<&'a str>> {
    let mut k_plus_1s = HashSet::new();

    for graph in k_graphs {
        let mut common_neighbors = links.get(graph[0]).unwrap().clone();
        for node in graph[1..].iter() {
            common_neighbors = common_neighbors.intersection(links.get(node).unwrap()).copied().collect();
        }

        for node in common_neighbors {
            let mut k_plus_1 = graph.clone();
            k_plus_1.push(node);
            k_plus_1.sort();
            k_plus_1s.insert(k_plus_1);
        }
    }

    k_plus_1s
}

fn main() {
    let input = include_str!("input.txt");

    let mut k_graphs = HashSet::new();
    let mut links: HashMap<&str, HashSet<&str>> = HashMap::new();

    let connections = input.lines()
        .map(|line| { line.split_once('-').unwrap() });

    for (a, b) in connections {
        let edge = [a, b].into_iter().sorted().collect::<Vec<_>>();
        k_graphs.insert(edge);
        links.entry(a).or_default().insert(b);
        links.entry(b).or_default().insert(a);
    }

    k_graphs = get_k_plus_1(&k_graphs, &links);

    let k_three_starting_with_t = k_graphs.iter()
        .filter(|mesh| mesh.len() == 3)
        .filter(|mesh| mesh.iter().any(|&node| node.starts_with('t')))
        .count();

    println!("Part 1: {}", k_three_starting_with_t);

    loop {
        let new_graphs = get_k_plus_1(&k_graphs, &links);
        if new_graphs.is_empty() {
            break;
        }
        k_graphs = new_graphs;
    }

    let biggest_k_graph = k_graphs.iter().next().unwrap();

    println!("Part 2: {}", biggest_k_graph.join(","));
}
