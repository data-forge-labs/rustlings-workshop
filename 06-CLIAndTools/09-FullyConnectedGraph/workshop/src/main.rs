use fully_connected_graph::{
    fully_connected_graph, generate_fully_connected_edges, generate_nodes,
};

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
