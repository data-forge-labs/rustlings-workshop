use std::collections::{HashMap, HashSet};

pub const TWITTER_USERNAMES: [&str; 140] = [
    "blackmattersus",
    "bleepthepolice",
    "jenn_abrams",
    "leroylovesusa",
    "missourinewsus",
    "rightnpr",
    "ten_gop",
    "traceyhappymom",
    "trayneshacole",
    "traceyhappymom",
    "ten_gop",
    "leroylovesusa",
    "leroylovesusa",
    "traceyhappymom",
    "traceyhappymom",
    "traceyhappymom",
    "ten_gop",
    "traceyhappymom",
    "jenn_abrams",
    "ten_gop",
    "rightnpr",
    "traceyhappymom",
    "leroylovesusa",
    "ten_gop",
    "ten_gop",
    "jenn_abrams",
    "leroylovesusa",
    "leroylovesusa",
    "ten_gop",
    "traceyhappymom",
    "ten_gop",
    "leroylovesusa",
    "ten_gop",
    "traceyhappymom",
    "jenn_abrams",
    "trayneshacole",
    "ten_gop",
    "ten_gop",
    "leroylovesusa",
    "leroylovesusa",
    "leroylovesusa",
    "leroylovesusa",
    "ten_gop",
    "ten_gop",
    "leroylovesusa",
    "ten_gop",
    "ten_gop",
    "traceyhappymom",
    "traceyhappymom",
    "ten_gop",
    "traceyhappymom",
    "ten_gop",
    "jenn_abrams",
    "ten_gop",
    "ten_gop",
    "leroylovesusa",
    "worldofhashtags",
    "traceyhappymom",
    "ten_gop",
    "leroylovesusa",
    "ten_gop",
    "traceyhappymom",
    "traceyhappymom",
    "ten_gop",
    "traceyhappymom",
    "traceyhappymom",
    "worldofhashtags",
    "ten_gop",
    "traceyhappymom",
    "ten_gop",
    "ten_gop",
    "ten_gop",
    "rightnpr",
    "ten_gop",
    "leroylovesusa",
    "traceyhappymom",
    "leroylovesusa",
    "leroylovesusa",
    "traceyhappymom",
    "traceyhappymom",
    "traceyhappymom",
    "ten_gop",
    "leroylovesusa",
    "traceyhappymom",
    "ten_gop",
    "blackmattersus",
    "ten_gop",
    "leroylovesusa",
    "ten_gop",
    "traceyhappymom",
    "jenn_abrams",
    "trayneshacole",
    "ten_gop",
    "ten_gop",
    "leroylovesusa",
    "leroylovesusa",
    "leroylovesusa",
    "leroylovesusa",
    "ten_gop",
    "ten_gop",
    "leroylovesusa",
    "ten_gop",
    "ten_gop",
    "traceyhappymom",
    "traceyhappymom",
    "worldofhashtags",
    "blackmattersus",
    "jenn_abrams",
    "traceyhappymom",
    "leroylovesusa",
    "jenn_abrams",
    "leroylovesusa",
    "traceyhappymom",
    "leroylovesusa",
    "jenn_abrams",
    "ten_gop",
    "leroylovesusa",
    "ten_gop",
    "ten_gop",
    "journalist1",
    "journalist2",
    "journalist3",
    "journalist1",
    "journalist2",
    "journalist1",
    "journalist3",
    "journalist2",
    "journalist1",
    "journalist3",
    "journalist2",
    "journalist3",
    "journalist1",
    "journalist2",
    "journalist1",
    "journalist3",
    "journalist2",
    "journalist1",
    "journalist3",
    "journalist2",
    "journalist3",
];

pub fn build_adjacency_list(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(from, to) in edges {
        graph.entry(from).or_default().push(to);
        graph.entry(to).or_default();
    }
    graph
}

pub fn dfs_order(graph: &HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    let mut stack = vec![start];
    while let Some(node) = stack.pop() {
        if visited.insert(node) {
            order.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors.iter().rev() {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
    order
}

pub fn reverse_graph(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, Vec<usize>> {
    let mut reversed: HashMap<usize, Vec<usize>> = HashMap::new();
    for (&node, neighbors) in graph {
        reversed.entry(node).or_default();
        for &neighbor in neighbors {
            reversed.entry(neighbor).or_default().push(node);
        }
    }
    reversed
}

pub fn kosaraju_scc(edges: &[(usize, usize)], node_count: usize) -> Vec<Vec<usize>> {
    let graph = build_adjacency_list(edges);
    let reversed = reverse_graph(&graph);

    let mut visited = vec![false; node_count];
    let mut stack = Vec::new();

    fn dfs_post(node: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
        visited[node] = true;
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    dfs_post(neighbor, graph, visited, stack);
                }
            }
        }
        stack.push(node);
    }

    for i in 0..node_count {
        if !visited[i] {
            dfs_post(i, &graph, &mut visited, &mut stack);
        }
    }

    let mut visited2 = vec![false; node_count];
    let mut sccs = Vec::new();

    fn dfs_collect(node: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut Vec<bool>, component: &mut Vec<usize>) {
        visited[node] = true;
        component.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    dfs_collect(neighbor, graph, visited, component);
                }
            }
        }
    }

    while let Some(node) = stack.pop() {
        if !visited2[node] {
            let mut component = Vec::new();
            dfs_collect(node, &reversed, &mut visited2, &mut component);
            component.sort();
            sccs.push(component);
        }
    }

    sccs
}

pub fn count_distinct_users() -> usize {
    let set: HashSet<&str> = TWITTER_USERNAMES.iter().cloned().collect();
    set.len()
}

pub fn top_n_users(n: usize) -> Vec<(&'static str, usize)> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &name in TWITTER_USERNAMES.iter() {
        *counts.entry(name).or_insert(0) += 1;
    }
    let mut vec: Vec<(&str, usize)> = counts.into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    vec.truncate(n);
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_graph_ops {
        use super::*;

        #[test]
        fn test_build_adjacency_list_basic() {
            let edges = [(0, 1), (1, 2), (2, 0)];
            let graph = build_adjacency_list(&edges);
            assert_eq!(graph.get(&0), Some(&vec![1]));
            assert_eq!(graph.get(&1), Some(&vec![2]));
            assert_eq!(graph.get(&2), Some(&vec![0]));
        }

        #[test]
        fn test_build_adjacency_list_empty() {
            let edges: [(usize, usize); 0] = [];
            let graph = build_adjacency_list(&edges);
            assert!(graph.is_empty());
        }

        #[test]
        fn test_build_adjacency_list_multiple_edges() {
            let edges = [(0, 1), (0, 2), (1, 2)];
            let graph = build_adjacency_list(&edges);
            assert_eq!(graph.get(&0), Some(&vec![1, 2]));
            assert_eq!(graph.get(&1), Some(&vec![2]));
        }

        #[test]
        fn test_dfs_order_simple_path() {
            let edges = [(0, 1), (1, 2)];
            let graph = build_adjacency_list(&edges);
            let order = dfs_order(&graph, 0);
            assert_eq!(order, vec![0, 1, 2]);
        }

        #[test]
        fn test_dfs_order_with_branching() {
            let edges = [(0, 1), (0, 2), (1, 3)];
            let graph = build_adjacency_list(&edges);
            let order = dfs_order(&graph, 0);
            assert_eq!(order, vec![0, 1, 3, 2]);
        }

        #[test]
        fn test_dfs_order_start_not_in_graph() {
            let graph: HashMap<usize, Vec<usize>> = HashMap::new();
            let order = dfs_order(&graph, 0);
            assert!(order.is_empty());
        }

        #[test]
        fn test_reverse_graph_basic() {
            let edges = [(0, 1), (1, 2)];
            let graph = build_adjacency_list(&edges);
            let reversed = reverse_graph(&graph);
            assert_eq!(reversed.get(&0), Some(&vec![]));
            assert_eq!(reversed.get(&1), Some(&vec![0]));
            assert_eq!(reversed.get(&2), Some(&vec![1]));
        }

        #[test]
        fn test_reverse_graph_empty() {
            let graph: HashMap<usize, Vec<usize>> = HashMap::new();
            let reversed = reverse_graph(&graph);
            assert!(reversed.is_empty());
        }
    }

    mod step_02_kosaraju {
        use super::*;

        #[test]
        fn test_single_node() {
            let edges = [];
            let sccs = kosaraju_scc(&edges, 1);
            assert_eq!(sccs.len(), 1);
            assert_eq!(sccs[0], vec![0]);
        }

        #[test]
        fn test_simple_cycle() {
            let edges = [(0, 1), (1, 0)];
            let sccs = kosaraju_scc(&edges, 2);
            assert_eq!(sccs.len(), 1);
            assert_eq!(sccs[0], vec![0, 1]);
        }

        #[test]
        fn test_two_components() {
            let edges = [(0, 1)];
            let sccs = kosaraju_scc(&edges, 2);
            assert_eq!(sccs.len(), 2);
            assert!(sccs.contains(&vec![0]));
            assert!(sccs.contains(&vec![1]));
        }

        #[test]
        fn test_disconnected_graph() {
            let edges = [];
            let sccs = kosaraju_scc(&edges, 3);
            assert_eq!(sccs.len(), 3);
        }

        #[test]
        fn test_complex_graph() {
            let edges = [(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 3)];
            let sccs = kosaraju_scc(&edges, 5);
            assert_eq!(sccs.len(), 2);
            assert!(sccs.contains(&vec![0, 1, 2]));
            assert!(sccs.contains(&vec![3, 4]));
        }
    }

    mod step_03_twitter_data {
        use super::*;

        #[test]
        fn test_count_distinct_users() {
            assert_eq!(count_distinct_users(), 13);
        }

        #[test]
        fn test_top_n_users_top_1() {
            let top = top_n_users(1);
            assert_eq!(top.len(), 1);
            assert_eq!(top[0].0, "ten_gop");
            assert_eq!(top[0].1, 40);
        }

        #[test]
        fn test_top_n_users_top_3() {
            let top = top_n_users(3);
            assert_eq!(top.len(), 3);
            assert_eq!(top[0].0, "ten_gop");
            assert_eq!(top[0].1, 40);
            assert_eq!(top[1].0, "leroylovesusa");
            assert_eq!(top[1].1, 28);
            assert_eq!(top[2].0, "traceyhappymom");
            assert_eq!(top[2].1, 28);
        }

        #[test]
        fn test_top_n_users_more_than_available() {
            let top = top_n_users(100);
            assert_eq!(top.len(), 13);
        }
    }
}
