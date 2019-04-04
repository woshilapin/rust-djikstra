use crate::node::Node;

use std::collections::HashMap;

trait Pick<V> {
    fn pick(&self) -> Option<&V>;
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Pick<V> for HashMap<K, V> {
    fn pick(&self) -> Option<&V> {
        if let Some(key) = self.keys().next() {
            self.get(&key)
        } else {
            None
        }
    }
}

fn pick_closest(q: &HashMap<usize, &Node>, distances: &HashMap<usize, u64>) -> Option<usize> {
    let mut minimum: u64 = std::u64::MAX;
    let mut minimum_node: &Node = match q.pick() {
        Some(n) => n,
        None => return None,
    };
    for (k, n) in q.iter() {
        if let Some(&d) = distances.get(&k) {
            if d < minimum {
                minimum = d;
                minimum_node = n;
            }
        }
    }
    Some(minimum_node.id)
}

pub fn djikstra(nodes: &[Node], source_id: usize) -> HashMap<usize, u64> {
    let mut distances: HashMap<usize, u64> = HashMap::new();
    for n in nodes.iter() {
        distances.insert(n.id, std::u64::MAX);
    }
    distances.insert(source_id, 0);
    let mut q: HashMap<usize, &Node> = HashMap::new();
    for n in nodes.iter() {
        q.insert(n.id, n);
    }
    while !q.is_empty() {
        if let Some(id_to_remove) = pick_closest(&q, &distances) {
            if let Some(removed_node) = q.remove(&id_to_remove) {
                for c in &removed_node.connections {
                    if let Some(current_distance) = distances.get(&c.to) {
                        if let Some(to_current) = distances.get(&removed_node.id) {
                            let from_current: u64 = c.weight;
                            let new_distance: u64 = *to_current + from_current;
                            if new_distance < *current_distance {
                                distances.insert(c.to, new_distance);
                            }
                        } else {
                            panic!("Cannot find the current distance to {}", removed_node.id);
                        }
                    } else {
                        panic!("Cannot find the current distance to {}", c.to);
                    }
                }
            } else {
                panic!(
                    "Cannot remove node {} from the list of nodes.",
                    id_to_remove
                );
            }
        } else {
            panic!("Cannot find the next closest node.");
        }
    }
    distances
}
