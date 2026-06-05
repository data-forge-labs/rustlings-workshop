use lisbon_shortest_path::{build_weighted_adjacency, dijkstra, format_path, shortest_path, WeightedEdge};

fn main() {
    // Lisbon landmarks as node indices:
    // 0: Belem Tower, 1: Jeronimos Monastery, 2: LX Factory,
    // 3: Commerce Square, 4: Lisbon Cathedral
    let edges = vec![
        WeightedEdge { from: 0, to: 1, weight: 1.0 },
        WeightedEdge { from: 0, to: 2, weight: 3.0 },
        WeightedEdge { from: 0, to: 3, weight: 7.0 },
        WeightedEdge { from: 1, to: 2, weight: 3.0 },
        WeightedEdge { from: 1, to: 3, weight: 6.0 },
        WeightedEdge { from: 2, to: 3, weight: 5.0 },
        WeightedEdge { from: 3, to: 4, weight: 1.0 },
    ];

    let adj = build_weighted_adjacency(&edges);
    let distances = dijkstra(&adj, 0);

    if let Some(&dist) = distances.get(&4) {
        println!(
            "The shortest distance from Belem Tower to Lisbon Cathedral is {:.1} km",
            dist
        );
        if let Some(path) = shortest_path(&adj, 0, 4) {
            println!("Path: {}", format_path(&path));
        }
    } else {
        println!("No route found from Belem Tower to Lisbon Cathedral.");
    }
}
