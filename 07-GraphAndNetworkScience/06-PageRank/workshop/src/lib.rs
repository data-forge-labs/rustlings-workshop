use std::collections::HashMap;

/// Compute PageRank scores iteratively
/// damping: probability of following a link (typically 0.85)
/// iterations: number of iterations
/// edges: list of (from, to) directed edges
pub fn page_rank(edges: &[(usize, usize)], node_count: usize, damping: f64, iterations: usize) -> HashMap<usize, f64> {
    let outlinks = build_outlinks(edges);
    let inlinks = build_inlinks(edges);

    let mut ranks: HashMap<usize, f64> = (0..node_count)
        .map(|i| (i, 1.0 / node_count as f64))
        .collect();

    for _ in 0..iterations {
        let mut new_ranks: HashMap<usize, f64> = HashMap::new();

        for node in 0..node_count {
            let mut rank_sum = 0.0;
            if let Some(incoming) = inlinks.get(&node) {
                for &src in incoming {
                    let out_degree = outlinks
                        .get(&src)
                        .map(|v| v.len())
                        .unwrap_or(0);
                    if out_degree > 0 {
                        rank_sum += ranks[&src] / out_degree as f64;
                    }
                }
            }
            let rank = (1.0 - damping) / node_count as f64 + damping * rank_sum;
            new_ranks.insert(node, rank);
        }

        ranks = new_ranks;
    }

    ranks
}

/// Get top N pages by PageRank
pub fn top_pages(scores: &HashMap<usize, f64>, n: usize) -> Vec<(usize, f64)> {
    let mut sorted: Vec<(usize, f64)> = scores.iter().map(|(&k, &v)| (k, v)).collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    sorted.truncate(n);
    sorted
}

/// Compute the sum of score changes between iterations (convergence check)
pub fn score_delta(prev: &HashMap<usize, f64>, curr: &HashMap<usize, f64>) -> f64 {
    let mut total = 0.0;
    for (key, prev_val) in prev {
        let curr_val = curr.get(key).copied().unwrap_or(0.0);
        total += (curr_val - prev_val).abs();
    }
    total
}

/// Build outlink map from edge list
pub fn build_outlinks(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut outlinks: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(from, to) in edges {
        outlinks.entry(from).or_default().push(to);
    }
    outlinks
}

/// Build inlink map from edge list
pub fn build_inlinks(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut inlinks: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(from, to) in edges {
        inlinks.entry(to).or_default().push(from);
    }
    inlinks
}

#[cfg(test)]
mod tests {
    mod step_01_graph_setup {
        use crate::{build_inlinks, build_outlinks};
        use std::collections::HashMap;

        #[test]
        fn test_build_outlinks_simple() {
            let edges = [(0, 1), (1, 2), (2, 0)];
            let outlinks = build_outlinks(&edges);
            assert_eq!(outlinks.get(&0), Some(&vec![1]));
            assert_eq!(outlinks.get(&1), Some(&vec![2]));
            assert_eq!(outlinks.get(&2), Some(&vec![0]));
        }

        #[test]
        fn test_build_outlinks_empty() {
            let edges: [(usize, usize); 0] = [];
            let outlinks = build_outlinks(&edges);
            assert!(outlinks.is_empty());
        }

        #[test]
        fn test_build_outlinks_dangling() {
            let edges = [(0, 1)];
            let outlinks = build_outlinks(&edges);
            assert_eq!(outlinks.get(&0), Some(&vec![1]));
            assert_eq!(outlinks.get(&1), None);
        }

        #[test]
        fn test_build_inlinks_simple() {
            let edges = [(0, 1), (1, 2), (2, 0)];
            let inlinks = build_inlinks(&edges);
            assert_eq!(inlinks.get(&1), Some(&vec![0]));
            assert_eq!(inlinks.get(&2), Some(&vec![1]));
            assert_eq!(inlinks.get(&0), Some(&vec![2]));
        }

        #[test]
        fn test_build_inlinks_empty() {
            let edges: [(usize, usize); 0] = [];
            let inlinks = build_inlinks(&edges);
            assert!(inlinks.is_empty());
        }

        #[test]
        fn test_build_inlinks_multiple() {
            let edges = [(0, 2), (1, 2), (2, 0)];
            let inlinks = build_inlinks(&edges);
            let mut expected: HashMap<usize, Vec<usize>> = HashMap::new();
            expected.insert(2, vec![0, 1]);
            expected.insert(0, vec![2]);
            assert_eq!(inlinks, expected);
        }
    }

    mod step_02_pagerank {
        use crate::page_rank;

        #[test]
        fn test_pagerank_basic_two_node() {
            let edges = [(0, 1), (1, 0)];
            let ranks = page_rank(&edges, 2, 0.85, 100);
            let sum: f64 = ranks.values().sum();
            assert!((sum - 1.0).abs() < 1e-10);
            assert!((ranks[&0] - ranks[&1]).abs() < 1e-10);
        }

        #[test]
        fn test_pagerank_single_node() {
            let edges: [(usize, usize); 0] = [];
            let ranks = page_rank(&edges, 1, 0.85, 10);
            assert!((ranks[&0] - 1.0).abs() < 1e-10);
        }

        #[test]
        fn test_pagerank_no_edges() {
            let edges: [(usize, usize); 0] = [];
            let ranks = page_rank(&edges, 3, 0.85, 10);
            for i in 0..3 {
                assert!((ranks[&i] - 1.0 / 3.0).abs() < 1e-10);
            }
        }

        #[test]
        fn test_pagerank_sum_to_one() {
            let edges = [(0, 1), (1, 2), (2, 0), (0, 2)];
            let ranks = page_rank(&edges, 3, 0.85, 50);
            let sum: f64 = ranks.values().sum();
            assert!((sum - 1.0).abs() < 1e-6);
        }

        #[test]
        fn test_pagerank_damping_one() {
            let edges = [(0, 1)];
            let ranks = page_rank(&edges, 2, 1.0, 100);
            assert!((ranks[&1] - 1.0).abs() < 1e-10);
            assert!((ranks[&0]).abs() < 1e-10);
        }

        #[test]
        fn test_pagerank_damping_zero() {
            let edges = [(0, 1)];
            let ranks = page_rank(&edges, 2, 0.0, 10);
            assert!((ranks[&0] - 0.5).abs() < 1e-10);
            assert!((ranks[&1] - 0.5).abs() < 1e-10);
        }
    }

    mod step_03_analysis {
        use crate::{score_delta, top_pages};
        use std::collections::HashMap;

        #[test]
        fn test_top_pages_basic() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.5);
            scores.insert(1, 0.3);
            scores.insert(2, 0.2);
            let top = top_pages(&scores, 2);
            assert_eq!(top.len(), 2);
            assert_eq!(top[0], (0, 0.5));
            assert_eq!(top[1], (1, 0.3));
        }

        #[test]
        fn test_top_pages_n_larger_than_available() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.8);
            scores.insert(1, 0.2);
            let top = top_pages(&scores, 10);
            assert_eq!(top.len(), 2);
        }

        #[test]
        fn test_top_pages_empty() {
            let scores: HashMap<usize, f64> = HashMap::new();
            let top = top_pages(&scores, 5);
            assert!(top.is_empty());
        }

        #[test]
        fn test_score_delta_identical() {
            let mut map = HashMap::new();
            map.insert(0, 0.5);
            map.insert(1, 0.5);
            let delta = score_delta(&map, &map);
            assert!((delta).abs() < 1e-10);
        }

        #[test]
        fn test_score_delta_different() {
            let mut prev = HashMap::new();
            prev.insert(0, 0.8);
            prev.insert(1, 0.2);
            let mut curr = HashMap::new();
            curr.insert(0, 0.6);
            curr.insert(1, 0.4);
            let delta = score_delta(&prev, &curr);
            assert!((delta - 0.4).abs() < 1e-10);
        }

        #[test]
        fn test_score_delta_partial_overlap() {
            let mut prev = HashMap::new();
            prev.insert(0, 1.0);
            let mut curr = HashMap::new();
            curr.insert(1, 1.0);
            let delta = score_delta(&prev, &curr);
            assert!((delta - 1.0).abs() < 1e-10);
        }

        #[test]
        fn test_top_pages_stable_order() {
            let mut scores = HashMap::new();
            scores.insert(0, 0.1);
            scores.insert(1, 0.1);
            scores.insert(2, 0.1);
            let top = top_pages(&scores, 3);
            assert_eq!(top.len(), 3);
        }
    }
}
