use std::collections::{HashMap, VecDeque};

/// Build adjacency list for UFC fighters
pub fn build_adjacency(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(a, b) in edges {
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }
    adj
}

/// Compute degree centrality for each node (normalized by N-1)
pub fn degree_centrality(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64> {
    let n = adj.len();
    let denom = if n <= 1 { 1.0 } else { (n - 1) as f64 };
    adj.iter().map(|(&k, v)| (k, v.len() as f64 / denom)).collect()
}

/// Compute closeness centrality for each node (uses BFS shortest paths)
pub fn closeness_centrality(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64> {
    let mut result = HashMap::new();
    for &start in adj.keys() {
        let mut visited: HashMap<usize, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        visited.insert(start, 0);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            let dist = visited[&node];
            if let Some(neighbors) = adj.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains_key(&neighbor) {
                        visited.insert(neighbor, dist + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        let mut sum_dist = 0usize;
        let mut reachable = 0usize;
        for (&node, &d) in &visited {
            if node != start {
                sum_dist += d;
                reachable += 1;
            }
        }

        let closeness = if reachable > 0 && sum_dist > 0 {
            reachable as f64 / sum_dist as f64
        } else {
            0.0
        };
        result.insert(start, closeness);
    }
    result
}

/// Return the node with highest degree centrality
pub fn most_central_node(centrality: &HashMap<usize, f64>) -> Option<usize> {
    centrality
        .iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(&k, _)| k)
}

/// Format centrality scores as "node: score" strings (sorted by node)
pub fn format_centrality(scores: &HashMap<usize, f64>) -> Vec<String> {
    let mut keys: Vec<&usize> = scores.keys().collect();
    keys.sort();
    keys.iter()
        .map(|&&k| format!("{}: {:.4}", k, scores[&k]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_graph_building {
        use super::*;

        #[test]
        fn simple_graph() {
            let edges = [(0, 1), (1, 2)];
            let adj = build_adjacency(&edges);
            assert_eq!(adj.len(), 3);
            assert_eq!(adj[&0], vec![1]);
            assert_eq!(adj[&1], vec![0, 2]);
            assert_eq!(adj[&2], vec![1]);
        }

        #[test]
        fn empty_edges() {
            let adj = build_adjacency(&[]);
            assert!(adj.is_empty());
        }

        #[test]
        fn node_with_no_edges() {
            let edges = [(0, 1)];
            let adj = build_adjacency(&edges);
            assert!(adj.contains_key(&0));
            assert!(adj.contains_key(&1));
            assert!(!adj.contains_key(&2));
        }
    }

    mod step_02_degree_centrality {
        use super::*;

        #[test]
        fn fully_connected_three_nodes() {
            let edges = [(0, 1), (0, 2), (1, 2)];
            let adj = build_adjacency(&edges);
            let cent = degree_centrality(&adj);
            // Each node has degree 2, N=3 -> N-1 = 2, so 2/2 = 1.0
            assert!((cent[&0] - 1.0).abs() < 1e-6);
            assert!((cent[&1] - 1.0).abs() < 1e-6);
            assert!((cent[&2] - 1.0).abs() < 1e-6);
        }

        #[test]
        fn single_node() {
            let mut adj = HashMap::new();
            adj.insert(0, vec![]);
            let cent = degree_centrality(&adj);
            // N=1, denom caps to 1, degree=0 -> 0.0
            assert!((cent[&0] - 0.0).abs() < 1e-6);
        }

        #[test]
        fn disconnected_nodes() {
            let mut adj = HashMap::new();
            adj.insert(0, vec![]);
            adj.insert(1, vec![]);
            let cent = degree_centrality(&adj);
            // N=2, N-1=1, degree=0 -> 0.0
            assert!((cent[&0] - 0.0).abs() < 1e-6);
            assert!((cent[&1] - 0.0).abs() < 1e-6);
        }
    }

    mod step_03_closeness_centrality {
        use super::*;

        #[test]
        fn fully_connected_three_nodes() {
            let edges = [(0, 1), (0, 2), (1, 2)];
            let adj = build_adjacency(&edges);
            let cent = closeness_centrality(&adj);
            // Each node reaches 2 others at distance 1 => sum=2, reachable=2 => 2/2 = 1.0
            assert!((cent[&0] - 1.0).abs() < 1e-6);
            assert!((cent[&1] - 1.0).abs() < 1e-6);
            assert!((cent[&2] - 1.0).abs() < 1e-6);
        }

        #[test]
        fn star_graph() {
            // Center 0 connected to leaves 1,2,3
            let edges = [(0, 1), (0, 2), (0, 3)];
            let adj = build_adjacency(&edges);
            let cent = closeness_centrality(&adj);
            // Center: reaches 3 at distance 1 each => sum=3, reachable=3 => 3/3 = 1.0
            assert!((cent[&0] - 1.0).abs() < 1e-6);
            // Leaf 1: reaches center at 1, leaves 2,3 via center at 2 each => sum=1+2+2=5, reachable=3 => 3/5 = 0.6
            assert!((cent[&1] - 0.6).abs() < 1e-6);
            assert!((cent[&2] - 0.6).abs() < 1e-6);
            assert!((cent[&3] - 0.6).abs() < 1e-6);
        }

        #[test]
        fn single_node() {
            let mut adj = HashMap::new();
            adj.insert(0, vec![]);
            let cent = closeness_centrality(&adj);
            // reachable=0, sum_dist=0 -> 0.0
            assert!((cent[&0] - 0.0).abs() < 1e-6);
        }
    }

    mod step_04_analysis {
        use super::*;

        #[test]
        fn most_central_returns_highest() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.5);
            scores.insert(1, 0.8);
            scores.insert(2, 0.3);
            assert_eq!(most_central_node(&scores), Some(1));
        }

        #[test]
        fn most_central_empty() {
            let scores: HashMap<usize, f64> = HashMap::new();
            assert_eq!(most_central_node(&scores), None);
        }

        #[test]
        fn format_sorted_by_node() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.5);
            scores.insert(2, 0.3);
            scores.insert(1, 0.8);
            let formatted = format_centrality(&scores);
            assert_eq!(formatted.len(), 3);
            assert!(formatted[0].starts_with("0:"));
            assert!(formatted[1].starts_with("1:"));
            assert!(formatted[2].starts_with("2:"));
        }

        #[test]
        fn format_empty() {
            let scores: HashMap<usize, f64> = HashMap::new();
            let formatted = format_centrality(&scores);
            assert!(formatted.is_empty());
        }
    }
}
