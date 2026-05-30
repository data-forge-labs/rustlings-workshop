use std::collections::HashMap;

fn main() {
    // Challenge(4): Implement a function that checks if a graph is fully connected
    let nodes = vec![1, 2, 3];
    let edges = vec![(1, 2), (1, 3), (2, 3)];
    let result = fully_connected_graph(&nodes, &edges);
    println!("Graph is fully connected: {:?}", edges);
    println!("Fully connected graph: {:?}", result);

    // Run with "big" graph
    let nodes = generate_nodes(10_000);
    let edges = generate_fully_connected_edges(&nodes);
    println!("Graph edges: {:?}", edges.len());
    let result = fully_connected_graph(&nodes, &edges);
    println!("Fully connected graph: {:?}", result);
}

fn connected_nodes(node_a: i32, node_b: i32, edges: &Vec<(i32, i32)>) -> bool {
    for (left, right) in edges {
        if (*left == node_a && *right == node_b) || (*left == node_b && *right == node_a) {
            return true;
        }
    }
    false
}

fn fully_connected_node(node_index: usize, nodes: &Vec<i32>, edges: &Vec<(i32, i32)>, memory: &mut HashMap<i32, i32>) -> bool {
    let center_node = nodes[node_index];
    for node in nodes {
        if *node == center_node {
            continue;
        }
        if memory.contains_key(node) {
            continue;
        }
        if !connected_nodes(center_node, *node, &edges) {
            return false;
        }
        memory.insert(center_node, *node);
        memory.insert(*node, center_node);
    }
    true
}

fn fully_connected_graph(nodes: &Vec<i32>, edges: &Vec<(i32, i32)>) -> bool {
    let mut memory = HashMap::new();
    for i in 0..nodes.len() {
        if !fully_connected_node(i, nodes, edges, &mut memory) {
            return false;
        }
    }
    true
}

fn generate_fully_connected_edges(nodes: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut edges = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push((nodes[i], nodes[j]));
        }
    }
    edges
}

fn generate_nodes(nodes_quantity: i32) -> Vec<i32> {
    let mut nodes = Vec::new();
    for i in 0..nodes_quantity {
        nodes.push(i);
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connected_nodes() {
        let result = connected_nodes(1, 2, &vec![(1, 2)]);
        assert!(result)
    }

    #[test]
    fn test_non_connected_nodes() {
        let result = connected_nodes(1, 2, &vec![(3, 2)]);
        assert!(!result)
    }

    #[test]
    fn test_fully_connected_node() {
        let mut memory = HashMap::new();
        let result = fully_connected_node(0, &vec![1, 2, 3, 4], &vec![(1, 2), (1, 3), (1, 4)], &mut memory);
        assert!(result)
    }

    #[test]
    fn test_non_fully_connected_node() {
        let mut memory = HashMap::new();
        let result = fully_connected_node(1, &vec![1, 2, 3, 4], &vec![(1, 2), (1, 3), (1, 4)], &mut memory); // 2 is only connected to 1
        assert!(!result)
    }

    #[test]
    fn test_fully_connected_graph() {
        let result = fully_connected_graph(
            &vec![1, 2, 3],
            &vec![(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)],
        );
        assert!(result)
    }

    #[test]
    fn test_non_fully_connected_graph() {
        let result = fully_connected_graph(&vec![1, 2, 3], &vec![(1, 3), (2, 3), (3, 1), (3, 2)]); // 1 and 2 are not connected
        assert!(!result)
    }
}
