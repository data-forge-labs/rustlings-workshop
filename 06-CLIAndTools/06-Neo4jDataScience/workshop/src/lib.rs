use std::collections::HashMap;

pub struct Graph {
    pub adjacency: HashMap<usize, Vec<usize>>,
}

pub fn build_graph(edges: &[(usize, usize)]) -> Graph {
    todo!()
}

pub fn compute_degree_centrality(graph: &Graph) -> HashMap<usize, f64> {
    todo!()
}

pub fn compute_closeness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    todo!()
}

pub fn compute_betweenness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    todo!()
}

pub fn top_n_central(scores: &HashMap<usize, f64>, n: usize) -> Vec<(usize, f64)> {
    todo!()
}

pub fn centrality_summary(scores: &HashMap<usize, f64>) -> (f64, f64, f64) {
    todo!()
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
