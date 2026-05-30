use std::collections::HashMap;

/// An edge with weight
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedEdge {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
}

/// Build adjacency list from weighted edges
pub fn build_weighted_adjacency(edges: &[WeightedEdge]) -> HashMap<usize, Vec<(usize, f64)>> {
    let mut adj: HashMap<usize, Vec<(usize, f64)>> = HashMap::new();
    for edge in edges {
        adj.entry(edge.from).or_default().push((edge.to, edge.weight));
        adj.entry(edge.to).or_default().push((edge.from, edge.weight));
    }
    adj
}

/// Find shortest path distances from start using Dijkstra
pub fn dijkstra(
    adj: &HashMap<usize, Vec<(usize, f64)>>,
    start: usize,
) -> HashMap<usize, f64> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut distances: HashMap<usize, f64> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start, 0.0);
    heap.push(Reverse((0.0_f64, start)));

    while let Some(Reverse((dist, node))) = heap.pop() {
        if dist > distances.get(&node).copied().unwrap_or(f64::INFINITY) {
            continue;
        }
        if let Some(neighbors) = adj.get(&node) {
            for &(neighbor, weight) in neighbors {
                let new_dist = dist + weight;
                let current = distances
                    .get(&neighbor)
                    .copied()
                    .unwrap_or(f64::INFINITY);
                if new_dist < current {
                    distances.insert(neighbor, new_dist);
                    heap.push(Reverse((new_dist, neighbor)));
                }
            }
        }
    }
    distances
}

/// Reconstruct shortest path from start to end using the distances map
pub fn shortest_path(
    adj: &HashMap<usize, Vec<(usize, f64)>>,
    start: usize,
    end: usize,
) -> Option<Vec<usize>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    if start == end {
        return Some(vec![start]);
    }

    let mut distances: HashMap<usize, f64> = HashMap::new();
    let mut prev: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start, 0.0);
    heap.push(Reverse((0.0_f64, start)));

    while let Some(Reverse((dist, node))) = heap.pop() {
        if node == end {
            break;
        }
        if dist > distances.get(&node).copied().unwrap_or(f64::INFINITY) {
            continue;
        }
        if let Some(neighbors) = adj.get(&node) {
            for &(neighbor, weight) in neighbors {
                let new_dist = dist + weight;
                let current = distances
                    .get(&neighbor)
                    .copied()
                    .unwrap_or(f64::INFINITY);
                if new_dist < current {
                    distances.insert(neighbor, new_dist);
                    prev.insert(neighbor, node);
                    heap.push(Reverse((new_dist, neighbor)));
                }
            }
        }
    }

    let mut path = Vec::new();
    let mut current = end;
    path.push(current);
    while current != start {
        if let Some(&p) = prev.get(&current) {
            path.push(p);
            current = p;
        } else {
            return None;
        }
    }
    path.reverse();
    Some(path)
}

/// Compute total weight of a path
pub fn path_weight(edges: &[WeightedEdge], path: &[usize]) -> Option<f64> {
    if path.len() < 2 {
        return None;
    }
    let edge_lookup: HashMap<(usize, usize), f64> = edges
        .iter()
        .flat_map(|e| [((e.from, e.to), e.weight), ((e.to, e.from), e.weight)])
        .collect();

    let mut total = 0.0;
    for window in path.windows(2) {
        let key = (window[0], window[1]);
        if let Some(&w) = edge_lookup.get(&key) {
            total += w;
        } else {
            return None;
        }
    }
    Some(total)
}

/// Format a path as readable string
pub fn format_path(path: &[usize]) -> String {
    path.iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(" -> ")
}

#[cfg(test)]
mod tests {
    mod step_01_graph_setup {
        use crate::{build_weighted_adjacency, WeightedEdge};

        #[test]
        fn test_simple_graph() {
            let edges = vec![
                WeightedEdge {
                    from: 0,
                    to: 1,
                    weight: 2.0,
                },
                WeightedEdge {
                    from: 1,
                    to: 2,
                    weight: 3.0,
                },
            ];
            let adj = build_weighted_adjacency(&edges);
            assert_eq!(adj.get(&0).unwrap().len(), 1);
            assert_eq!(adj.get(&1).unwrap().len(), 2);
            assert_eq!(adj.get(&2).unwrap().len(), 1);
        }

        #[test]
        fn test_single_edge() {
            let edges = vec![WeightedEdge {
                from: 0,
                to: 1,
                weight: 5.0,
            }];
            let adj = build_weighted_adjacency(&edges);
            assert_eq!(adj[&0], vec![(1, 5.0)]);
            assert_eq!(adj[&1], vec![(0, 5.0)]);
        }

        #[test]
        fn test_empty_edges() {
            let edges = vec![];
            let adj = build_weighted_adjacency(&edges);
            assert!(adj.is_empty());
        }
    }

    mod step_02_dijkstra {
        use crate::{build_weighted_adjacency, dijkstra, WeightedEdge};

        #[test]
        fn test_simple_path() {
            let edges = vec![
                WeightedEdge {
                    from: 0,
                    to: 1,
                    weight: 1.0,
                },
                WeightedEdge {
                    from: 1,
                    to: 2,
                    weight: 2.0,
                },
            ];
            let adj = build_weighted_adjacency(&edges);
            let dist = dijkstra(&adj, 0);
            assert_eq!(dist.get(&0).copied(), Some(0.0));
            assert_eq!(dist.get(&1).copied(), Some(1.0));
            assert_eq!(dist.get(&2).copied(), Some(3.0));
        }

        #[test]
        fn test_unreachable_node() {
            let edges = vec![WeightedEdge {
                from: 0,
                to: 1,
                weight: 1.0,
            }];
            let adj = build_weighted_adjacency(&edges);
            let dist = dijkstra(&adj, 0);
            assert_eq!(dist.get(&0).copied(), Some(0.0));
            assert_eq!(dist.get(&1).copied(), Some(1.0));
            assert!(dist.get(&2).is_none());
        }

        #[test]
        fn test_same_start_end() {
            let edges = vec![WeightedEdge {
                from: 0,
                to: 1,
                weight: 1.0,
            }];
            let adj = build_weighted_adjacency(&edges);
            let dist = dijkstra(&adj, 0);
            assert_eq!(dist.get(&0).copied(), Some(0.0));
        }
    }

    mod step_03_path_reconstruction {
        use crate::{build_weighted_adjacency, path_weight, shortest_path, WeightedEdge};

        #[test]
        fn test_direct_edge() {
            let edges = vec![WeightedEdge {
                from: 0,
                to: 1,
                weight: 3.0,
            }];
            let adj = build_weighted_adjacency(&edges);
            let path = shortest_path(&adj, 0, 1);
            assert_eq!(path, Some(vec![0, 1]));
            assert_eq!(path_weight(&edges, &[0, 1]), Some(3.0));
        }

        #[test]
        fn test_multi_hop() {
            let edges = vec![
                WeightedEdge {
                    from: 0,
                    to: 1,
                    weight: 1.0,
                },
                WeightedEdge {
                    from: 1,
                    to: 2,
                    weight: 2.0,
                },
                WeightedEdge {
                    from: 0,
                    to: 2,
                    weight: 10.0,
                },
            ];
            let adj = build_weighted_adjacency(&edges);
            let path = shortest_path(&adj, 0, 2);
            assert_eq!(path, Some(vec![0, 1, 2]));
            assert_eq!(path_weight(&edges, &[0, 1, 2]), Some(3.0));
        }

        #[test]
        fn test_no_path() {
            let edges = vec![WeightedEdge {
                from: 0,
                to: 1,
                weight: 1.0,
            }];
            let adj = build_weighted_adjacency(&edges);
            let path = shortest_path(&adj, 0, 3);
            assert_eq!(path, None);
        }

        #[test]
        fn test_path_weight_single_node() {
            let edges = vec![WeightedEdge {
                from: 0,
                to: 1,
                weight: 5.0,
            }];
            assert_eq!(path_weight(&edges, &[0]), None);
        }

        #[test]
        fn test_path_weight_nonexistent_edge() {
            let edges = vec![WeightedEdge {
                from: 0,
                to: 1,
                weight: 5.0,
            }];
            assert_eq!(path_weight(&edges, &[0, 2]), None);
        }
    }

    mod step_04_formatting {
        use crate::format_path;

        #[test]
        fn test_format_path() {
            assert_eq!(format_path(&[1, 2, 3]), "1 -> 2 -> 3");
        }

        #[test]
        fn test_single_node_path() {
            assert_eq!(format_path(&[0]), "0");
        }

        #[test]
        fn test_empty_path() {
            assert_eq!(format_path(&[]), "");
        }
    }
}
