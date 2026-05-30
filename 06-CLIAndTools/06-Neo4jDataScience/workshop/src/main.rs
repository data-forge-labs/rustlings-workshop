use neo4j_data_science_lib::*;

fn main() {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)];
    let graph = build_graph(&edges);

    println!("Graph with {} nodes", graph.adjacency.len());

    let degree = compute_degree_centrality(&graph);
    println!("Degree centrality: {:?}", top_n_central(&degree, 3));

    let closeness = compute_closeness_centrality(&graph);
    println!("Closeness centrality: {:?}", top_n_central(&closeness, 3));

    let betweenness = compute_betweenness_centrality(&graph);
    println!("Betweenness centrality: {:?}", top_n_central(&betweenness, 3));

    println!("Degree summary: {:?}", centrality_summary(&degree));
}
