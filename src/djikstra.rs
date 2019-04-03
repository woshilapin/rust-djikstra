use crate::node::Node;

use std::collections::HashMap;

trait Pick<V> {
    fn pick(&self) -> Option<&V>;
}

impl<K : std::cmp::Eq + std::hash::Hash, V> Pick<V> for HashMap<K, V> {
    fn pick(&self) -> Option<&V> {
        if let Some(key) = self.keys().next() {
            self.get(&key)
        } else {
            None
        }
    }
}

pub fn djikstra(nodes: Vec<Node>) {
    println!("Start Djikstra algorithm");
    let source: &Node = &nodes[0];
    let mut distances: HashMap<usize, u64> = HashMap::new();
    for n in nodes.iter() {
        distances.insert(n.id, std::u64::MAX);
    }
    distances.insert(source.id, 0);
    let mut q: HashMap<usize, &Node> = HashMap::new();
    for n in nodes.iter() {
        q.insert(n.id, n);
    }
    while !q.is_empty() {
        println!("Number of nodes in 'q' is {}", q.len());
        let mut minimum: u64 = std::u64::MAX;
        let mut minimum_node: &Node = source;
        for (k, n) in &mut q {
            if let Some(&d) = distances.get(k) {
                if d < minimum {
                    minimum = d;
                    minimum_node = n;
                }
            }
        }
        println!(
            "Node {} is closer from source (distance: {}), it will be removed",
            minimum_node.id, minimum
        );
        if let Some(removed_node) = q.remove(&minimum_node.id) {
            for c in &removed_node.connections {
                println!("Process connection from {} to {}", removed_node.id, c.to);
                let current_distance: u64 = *distances.get(&c.to).unwrap();
                println!("Distance from source to {} is {}", c.to, current_distance);
                let to_current: u64 = *distances.get(&removed_node.id).unwrap();
                println!(
                    "Distance from source to {} is {}",
                    removed_node.id, to_current
                );
                let from_current: u64 = c.weight;
                println!(
                    "Distance from {} to {} is {}",
                    removed_node.id, c.to, from_current
                );
                let new_distance: u64 = to_current + from_current;
                if new_distance < current_distance {
                    distances.insert(c.to, new_distance);
                }
            }
        }
    }
    for (key, value) in distances.iter() {
        println!("Node: {} -> {}", key, value);
    }
}
