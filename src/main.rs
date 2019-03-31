mod djikstra;
mod node;

use crate::node::{Link, Node};
use crate::djikstra::djikstra;

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
    println!("Graph initialized");
    djikstra(nodes);
}
