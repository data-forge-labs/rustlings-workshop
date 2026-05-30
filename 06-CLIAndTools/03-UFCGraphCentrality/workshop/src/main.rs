use graph_centrality_ufc::{build_adjacency, closeness_centrality, degree_centrality, format_centrality, most_central_node};

fn main() {
    let fighters = [
        "Dustin Poirier",
        "Khabib Nurmagomedov",
        "Jose Aldo",
        "Conor McGregor",
        "Nate Diaz",
    ];

    let edges = [(0, 1), (1, 3), (3, 0), (3, 2), (3, 4), (0, 4), (2, 4)];

    let adj = build_adjacency(&edges);

    println!("UFC Fighter Fight Network");
    println!("=========================\n");

    println!("Degree Centrality:");
    let degree = degree_centrality(&adj);
    for line in format_centrality(&degree) {
        println!("  {}", line);
    }
    if let Some(most) = most_central_node(&degree) {
        println!(
            "  => Most central: {} (degree centrality {:.4})",
            fighters[most], degree[&most]
        );
    }

    println!("\nCloseness Centrality:");
    let closeness = closeness_centrality(&adj);
    for line in format_centrality(&closeness) {
        println!("  {}", line);
    }
    if let Some(most) = most_central_node(&closeness) {
        println!(
            "  => Most central: {} (closeness centrality {:.4})",
            fighters[most], closeness[&most]
        );
    }
}
