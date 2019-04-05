mod djikstra;
mod node;

use crate::djikstra::djikstra;
use crate::node::{Link, Node};
use std::collections::HashMap;

fn sort_by_id<'a, T>(entry: &'a (&usize, &T)) -> usize {
    *entry.0
}

fn main() {
    let node1 = Node {
        id: 1,
        connections: vec![
            Link { weight: 7, to: 2 },
            Link { weight: 9, to: 3 },
            Link { weight: 14, to: 6 },
        ],
    };
    let node2 = Node {
        id: 2,
        connections: vec![
            Link { weight: 7, to: 1 },
            Link { weight: 10, to: 3 },
            Link { weight: 15, to: 4 },
        ],
    };
    let node3 = Node {
        id: 3,
        connections: vec![
            Link { weight: 9, to: 1 },
            Link { weight: 10, to: 2 },
            Link { weight: 11, to: 4 },
            Link { weight: 2, to: 6 },
        ],
    };
    let node4 = Node {
        id: 4,
        connections: vec![
            Link { weight: 15, to: 2 },
            Link { weight: 11, to: 3 },
            Link { weight: 6, to: 5 },
        ],
    };
    let node5 = Node {
        id: 5,
        connections: vec![Link { weight: 6, to: 4 }, Link { weight: 9, to: 6 }],
    };
    let node6 = Node {
        id: 6,
        connections: vec![
            Link { weight: 14, to: 1 },
            Link { weight: 2, to: 3 },
            Link { weight: 9, to: 5 },
        ],
    };
    let nodes = vec![node1, node2, node3, node4, node5, node6];
    let mut distances = HashMap::new();
    for id in 1..7 {
        distances.insert(id, djikstra(&nodes, id));
    }
    println!("   {:^4} {:^4} {:^4} {:^4} {:^4} {:^4}", 1, 2, 3, 4, 5, 6);
    println!("   -----------------------------");
    let mut sorted_map: Vec<(&usize, &HashMap<_, _>)> = distances.iter().collect();
    sorted_map.sort_by_key(sort_by_id);
    for (source_id, distances_from_source) in sorted_map {
        print!("{} |", source_id);
        let mut sorted_distance: Vec<(&usize, &u64)> = distances_from_source.iter().collect();
        sorted_distance.sort_by_key(sort_by_id);
        for (_, distance) in sorted_distance {
            print!("{:^4}|", distance);
        }
        println!();
    }
}
