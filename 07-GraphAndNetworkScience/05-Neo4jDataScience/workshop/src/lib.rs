use std::collections::{HashMap, VecDeque};

pub struct Graph {
    pub adjacency: HashMap<usize, Vec<usize>>,
}

pub fn build_graph(edges: &[(usize, usize)]) -> Graph {
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(a, b) in edges {
        adjacency.entry(a).or_default().push(b);
        if a != b {
            adjacency.entry(b).or_default().push(a);
        }
    }
    Graph { adjacency }
}

pub fn compute_degree_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let n = graph.adjacency.len();
    let denom = if n <= 1 { 1.0 } else { (n - 1) as f64 };
    graph
        .adjacency
        .iter()
        .map(|(&k, v)| (k, v.len() as f64 / denom))
        .collect()
}

pub fn compute_closeness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    use std::collections::VecDeque;
    let mut result = HashMap::new();
    for &start in graph.adjacency.keys() {
        let mut visited: HashMap<usize, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        visited.insert(start, 0);
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            let dist = visited[&node];
            if let Some(neighbors) = graph.adjacency.get(&node) {
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

pub fn compute_betweenness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let mut scores: HashMap<usize, f64> = graph.adjacency.keys().map(|&k| (k, 0.0)).collect();
    let nodes: Vec<usize> = graph.adjacency.keys().copied().collect();
    for &s in &nodes {
        let mut stack = Vec::new();
        let mut predecessors: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut dist: HashMap<usize, f64> = HashMap::new();
        let mut sigma: HashMap<usize, f64> = HashMap::new();
        dist.insert(s, 0.0);
        sigma.insert(s, 1.0);
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            if let Some(neighbors) = graph.adjacency.get(&v) {
                for &w in neighbors {
                    let new_dist = dist[&v] + 1.0;
                    if !dist.contains_key(&w) {
                        dist.insert(w, new_dist);
                        queue.push_back(w);
                    }
                    if dist[&w] == new_dist {
                        *sigma.entry(w).or_insert(0.0) += sigma[&v];
                        predecessors.entry(w).or_default().push(v);
                    }
                }
            }
        }
        let mut delta: HashMap<usize, f64> = HashMap::new();
        while let Some(w) = stack.pop() {
            if let Some(preds) = predecessors.get(&w) {
                for &v in preds {
                    let contrib = (sigma[&v] / sigma[&w]) * (1.0 + *delta.get(&w).unwrap_or(&0.0));
                    *delta.entry(v).or_insert(0.0) += contrib;
                }
            }
            if w != s {
                *scores.entry(w).or_insert(0.0) += *delta.get(&w).unwrap_or(&0.0);
            }
        }
    }
    let n = nodes.len() as f64;
    if n > 2.0 {
        for v in scores.values_mut() {
            *v /= (n - 1.0) * (n - 2.0);
        }
    }
    scores
}

pub fn top_n_central(scores: &HashMap<usize, f64>, n: usize) -> Vec<(usize, f64)> {
    let mut sorted: Vec<(usize, f64)> = scores.iter().map(|(&k, &v)| (k, v)).collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    sorted.truncate(n);
    sorted
}

pub fn centrality_summary(scores: &HashMap<usize, f64>) -> (f64, f64, f64) {
    let values: Vec<f64> = scores.values().copied().collect();
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    (min, max, mean)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_graph_creation {
        use super::*;

        #[test]
        fn test_build_graph_simple() {
            let edges = vec![(0, 1), (1, 2), (2, 0)];
            let graph = build_graph(&edges);
            assert_eq!(graph.adjacency.len(), 3);
            assert!(graph.adjacency.get(&0).unwrap().contains(&1));
            assert!(graph.adjacency.get(&1).unwrap().contains(&2));
            assert!(graph.adjacency.get(&2).unwrap().contains(&0));
        }

        #[test]
        fn test_build_graph_empty() {
            let edges: Vec<(usize, usize)> = vec![];
            let graph = build_graph(&edges);
            assert!(graph.adjacency.is_empty());
        }

        #[test]
        fn test_build_graph_disconnected() {
            let edges = vec![(0, 1)];
            let graph = build_graph(&edges);
            assert_eq!(graph.adjacency.len(), 2);
            assert_eq!(graph.adjacency.get(&0).unwrap().len(), 1);
            assert_eq!(graph.adjacency.get(&1).unwrap().len(), 1);
        }
    }

    mod step_02_degree {
        use super::*;

        #[test]
        fn test_degree_fully_connected() {
            let edges = vec![(0, 1), (0, 2), (1, 2)];
            let graph = build_graph(&edges);
            let scores = compute_degree_centrality(&graph);
            // Each node is connected to 2 others out of 3 possible
            for v in scores.values() {
                assert!((*v - 1.0).abs() < 1e-6);
            }
        }

        #[test]
        fn test_degree_star() {
            let edges = vec![(0, 1), (0, 2), (0, 3)];
            let graph = build_graph(&edges);
            let scores = compute_degree_centrality(&graph);
            assert!((scores[&0] - 1.0).abs() < 1e-6);
            assert!((scores[&1] - 1.0 / 3.0).abs() < 1e-6);
        }

        #[test]
        fn test_degree_single_node() {
            let edges = vec![(0, 0)];
            let graph = build_graph(&edges);
            let scores = compute_degree_centrality(&graph);
            assert!((scores[&0] - 1.0).abs() < 1e-6);
        }
    }

    mod step_03_closeness {
        use super::*;

        #[test]
        fn test_closeness_line() {
            let edges = vec![(0, 1), (1, 2)];
            let graph = build_graph(&edges);
            let scores = compute_closeness_centrality(&graph);
            // Node 1 is in the middle -> highest closeness
            assert!(scores[&1] > scores[&0]);
            assert!(scores[&1] > scores[&2]);
        }

        #[test]
        fn test_closeness_isolated() {
            let edges = vec![(0, 1)];
            let graph = build_graph(&edges);
            let scores = compute_closeness_centrality(&graph);
            assert_eq!(scores[&0], 1.0);
            assert_eq!(scores[&1], 1.0);
        }
    }

    mod step_04_betweenness {
        use super::*;

        #[test]
        fn test_betweenness_line() {
            let edges = vec![(0, 1), (1, 2)];
            let graph = build_graph(&edges);
            let scores = compute_betweenness_centrality(&graph);
            // Middle node 1 lies on the only path between 0 and 2
            assert!(scores[&1] > scores[&0]);
            assert!(scores[&1] > scores[&2]);
        }

        #[test]
        fn test_betweenness_fully_connected() {
            let edges = vec![(0, 1), (0, 2), (1, 2)];
            let graph = build_graph(&edges);
            let scores = compute_betweenness_centrality(&graph);
            // In a triangle, no node lies on a shortest path between the other two
            for v in scores.values() {
                assert!((*v).abs() < 1e-6);
            }
        }
    }

    mod step_05_analysis {
        use super::*;

        #[test]
        fn test_top_n_central_basic() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.8);
            scores.insert(1, 0.5);
            scores.insert(2, 0.9);
            scores.insert(3, 0.1);
            let top = top_n_central(&scores, 2);
            assert_eq!(top.len(), 2);
            assert_eq!(top[0].0, 2);
            assert_eq!(top[1].0, 0);
        }

        #[test]
        fn test_top_n_central_larger_than_map() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.8);
            scores.insert(1, 0.5);
            let top = top_n_central(&scores, 5);
            assert_eq!(top.len(), 2);
        }

        #[test]
        fn test_centrality_summary_basic() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.1);
            scores.insert(1, 0.5);
            scores.insert(2, 0.9);
            let (min, max, mean) = centrality_summary(&scores);
            assert!((min - 0.1).abs() < 1e-6);
            assert!((max - 0.9).abs() < 1e-6);
            assert!((mean - 0.5).abs() < 1e-6);
        }

        #[test]
        fn test_centrality_summary_single() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.42);
            let (min, max, mean) = centrality_summary(&scores);
            assert!((min - 0.42).abs() < 1e-6);
            assert!((max - 0.42).abs() < 1e-6);
            assert!((mean - 0.42).abs() < 1e-6);
        }
    }
}
