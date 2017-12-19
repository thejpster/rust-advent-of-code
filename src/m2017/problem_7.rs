use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    weight: u32,
    children: Vec<String>,
}

pub fn run(contents: &[Vec<String>]) {
    let mut children_set: HashSet<String> = HashSet::new();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in &contents[0] {
        if line.contains("->") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let children: Vec<String> = parts[3..].iter().map(|x| x.replace(",", "")).collect();
            for c in &children {
                children_set.insert(c.clone());
            }
            let n = Node {
                weight: parts[1].replace("(", "").replace(")", "").parse().unwrap(),
                children: children,
            };
            nodes.insert(parts[0].into(), n);
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let n = Node {
                weight: parts[1].replace("(", "").replace(")", "").parse().unwrap(),
                children: Vec::new(),
            };
            nodes.insert(parts[0].into(), n);
        }
    }

    let mut root = None;
    for name in nodes.keys() {
        if !children_set.contains(name) {
            root = Some(name.clone());
        }
    }

    let root = root.unwrap();
    println!("Root: {}", root);

    walk_node(&root, &nodes);
}

fn weigh_node(name: &str, nodes: &HashMap<String, Node>) -> u32 {
    let node = nodes.get(name).unwrap();
    let mut weight = node.weight;
    for c in &node.children {
        weight += weigh_node(c, nodes);
    }
    weight
}

fn walk_node(name: &str, nodes: &HashMap<String, Node>) {
    let node = nodes.get(name).unwrap();
    let weights: Vec<u32> = node.children
        .iter()
        .map(|x| weigh_node(x, nodes))
        .collect();
    for w in &weights {
        if *w != weights[0] {
            println!("Mismatch {} != {}", w, weights[0]);
            break;
        }
    }
    if !node.children.is_empty() {
        println!(
            "{} ({}) -> {:?}",
            name,
            node.weight,
            node.children.iter().zip(weights).collect::<Vec<_>>()
        );
    } else {
        println!("{} ({})", name, node.weight);
    }
    for c in &node.children {
        walk_node(c, nodes);
    }
}
