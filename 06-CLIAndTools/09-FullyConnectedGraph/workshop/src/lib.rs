use std::collections::HashMap;

pub fn connected_nodes(node_a: i32, node_b: i32, edges: &[(i32, i32)]) -> bool {
    for (left, right) in edges {
        if (*left == node_a && *right == node_b) || (*left == node_b && *right == node_a) {
            return true;
        }
    }
    false
}

pub fn fully_connected_node(
    node_index: usize,
    nodes: &[i32],
    edges: &[(i32, i32)],
    memory: &mut HashMap<i32, i32>,
) -> bool {
    let center_node = nodes[node_index];
    for node in nodes {
        if *node == center_node {
            continue;
        }
        if memory.contains_key(node) {
            continue;
        }
        if !connected_nodes(center_node, *node, edges) {
            return false;
        }
        memory.insert(center_node, *node);
        memory.insert(*node, center_node);
    }
    true
}

pub fn fully_connected_graph(nodes: &[i32], edges: &[(i32, i32)]) -> bool {
    let mut memory = HashMap::new();
    for i in 0..nodes.len() {
        if !fully_connected_node(i, nodes, edges, &mut memory) {
            return false;
        }
    }
    true
}

pub fn generate_fully_connected_edges(nodes: &[i32]) -> Vec<(i32, i32)> {
    let mut edges = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push((nodes[i], nodes[j]));
        }
    }
    edges
}

pub fn generate_nodes(nodes_quantity: i32) -> Vec<i32> {
    let mut nodes = Vec::new();
    for i in 0..nodes_quantity {
        nodes.push(i);
    }
    nodes
}

#[cfg(test)]
mod tests {
    mod step_01_connected_nodes {
        use super::super::*;

        #[test]
        fn test_connected_nodes() {
            let result = connected_nodes(1, 2, &[(1, 2)]);
            assert!(result)
        }

        #[test]
        fn test_non_connected_nodes() {
            let result = connected_nodes(1, 2, &[(3, 2)]);
            assert!(!result)
        }
    }

    mod step_02_fully_connected_node {
        use super::super::*;

        #[test]
        fn test_fully_connected_node() {
            let mut memory = HashMap::new();
            let result =
                fully_connected_node(0, &[1, 2, 3, 4], &[(1, 2), (1, 3), (1, 4)], &mut memory);
            assert!(result)
        }

        #[test]
        fn test_non_fully_connected_node() {
            let mut memory = HashMap::new();
            let result = fully_connected_node(
                1,
                &[1, 2, 3, 4],
                &[(1, 2), (1, 3), (1, 4)],
                &mut memory,
            );
            assert!(!result)
        }
    }

    mod step_03_fully_connected_graph {
        use super::super::*;

        #[test]
        fn test_fully_connected_graph() {
            let result = fully_connected_graph(
                &[1, 2, 3],
                &[(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)],
            );
            assert!(result)
        }

        #[test]
        fn test_non_fully_connected_graph() {
            let result = fully_connected_graph(&[1, 2, 3], &[(1, 3), (2, 3), (3, 1), (3, 2)]);
            assert!(!result)
        }
    }

    mod step_04_generators {
        use super::super::*;

        #[test]
        fn test_generate_nodes() {
            let nodes = generate_nodes(5);
            assert_eq!(nodes.len(), 5);
            assert_eq!(nodes, vec![0, 1, 2, 3, 4])
        }

        #[test]
        fn test_generate_fully_connected_edges() {
            let nodes = [1, 2, 3];
            let edges = generate_fully_connected_edges(&nodes);
            assert_eq!(edges.len(), 3);
            assert!(edges.contains(&(1, 2)));
            assert!(edges.contains(&(1, 3)));
            assert!(edges.contains(&(2, 3)))
        }
    }
}
