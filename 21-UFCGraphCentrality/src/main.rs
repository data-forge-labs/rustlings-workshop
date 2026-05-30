//! # Challenge Questions
//!
//! ## What is the 'closeness centrality' in the context of this program, and how is
//! it calculated?
//!
//! In this program, 'closeness centrality' is defined inversely to the number of
//! outgoing fights a fighter has. It is calculated as the reciprocal of the degree
//! of the node representing the fighter in the graph. The degree is the count of
//! outgoing edges from the node, which correspond to the fights the fighter has
//! had.
//!
//! ```rust
//! let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
//! let closeness = 1.0 / degree;
//! ```
//!
//! A higher degree (more fights) results in a lower closeness centrality score,
//! while a lower degree (fewer fights) results in a higher closeness centrality
//! score. This interpretation of centrality is specific to this program and may
//! differ from the traditional graph-theoretic definition, which normally
//! calculates closeness centrality based on the shortest paths to all other nodes
//! in the graph.
//! 
//! ## How does the add_edge function work, and why do you need to pass in an
//! array of NodeIndex?
//!
//! The `add_edge` function is used to add an edge between two nodes in the graph.
//! It requires references to NodeIndex for both nodes involved in the edge. Here's
//! how the function works:
//!
//! ```rust
//! fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex],
//!             a: usize, b: usize) {
//!     graph.add_edge(nodes[a], nodes[b], 1.0);
//! }
//! ```
//!
//! In this function, `graph` is a mutable reference to an undirected graph and
//! `nodes` is a slice of NodeIndex, which are indices that point to nodes within
//! the graph. The parameters `a` and `b` are the positions in the `nodes` array
//! of the two nodes you want to connect.
//!
//! The `add_edge` method of the `UnGraph` struct then takes the two `NodeIndex`
//! values from the `nodes` array and creates an edge between them with a weight of
//! 1.0.
//!
//! Passing an array of `NodeIndex` is necessary because the `NodeIndex` is a way
//! to reference nodes in a graph without using direct references, which aligns
//! with Rust's ownership and borrowing principles. This approach allows for safe
//! and efficient manipulation of the graph's structure.
//! 
//! ### Deep Explanation of "Passing an array of `NodeIndex`"
//!
//! In graph-related algorithms, a `NodeIndex` is a value that uniquely identifies
//! a node within a graph data structure. It acts as an abstraction over the
//! actual memory location of the node, providing a stable way to refer to nodes
//! even as the graph changes.
//!
//! When working with graphs in Rust, especially with libraries like `petgraph`,
//! you often manipulate the graph structure through these indices rather than
//! direct references to the nodes themselves. This design is particularly
//! advantageous in Rust due to its strict ownership and lifetime rules, as it
//! avoids issues related to borrowing and lifetimes that would arise with
//! references.
//!
//! The concept of passing an array of `NodeIndex` is common in scenarios where
//! bulk operations on the graph are required. Here's why:
//!
//! **Safety and Concurrency**
//! Rust's safety guarantees prevent data races and ensure that references to
//! graph nodes do not outlive the graph itself. By using `NodeIndex` values,
//! which are typically just integers under the hood, you can safely pass around
//! identifiers for nodes without worrying about these issues.
//!
//! **Efficiency**
//! Operations that might require re-indexing or reorganizing the graph don't have
//! to alter the actual references or move large amounts of data around; they can
//! simply remap the indices, which are lightweight.
//!
//! **Flexibility**
//! An array of `NodeIndex` values provides the ability to work with subgraphs or
//! specific node sets efficiently. For example, if you want to add edges in bulk
//! between nodes that form a **clique** or a **path**, having their indices in an
//! array makes iteration and edge creation straightforward.
//!
//! **API Design**
//! A graph library API can provide functions that take slices of `NodeIndex`
//! values (`&[NodeIndex]`) as parameters. This allows users of the library to
//! perform operations on dynamically sized sets of nodes. It also enables
//! functions to be more generic and work with any number of nodes, as opposed to
//! overloading functions for different numbers of parameters.
//!
//! When you pass an array of `NodeIndex` to a function like `add_edge`, the
//! function can use these indices to refer to the nodes and add an edge between
//! them without needing to access the nodes directly. This is particularly useful
//! when the nodes you are dealing with are not contiguous or have been selected
//! as a result of some computation or filtering criteria.
//!
//! In summary, passing an array of `NodeIndex` is a pattern that leverages Rust's
//! strengths in terms of safety and efficiency, while providing a flexible and
//! powerful interface for graph manipulation.
//! 
//! ## Why do we calculate the degree of a node by counting its outgoing edges?
//!
//! The degree of a node in a graph indicates how many edges are associated with
//! that node. For undirected graphs, this includes all edges connected to the
//! node. However, in directed graphs, edges have a direction, and thus the
//! concept of degree is split into two:
//!
//! - **Out-degree**: This is calculated by counting the number of outgoing
//! edges from the node. It represents the number of direct paths from this node
//! to others.
//!
//! - **In-degree**: This is calculated by counting the number of incoming
//! edges to the node. It represents the number of direct paths from other nodes
//! to this one.
//!
//! Calculating the out-degree gives us information about the potential
//! influence or spread a node has within the network, as it indicates how many
//! nodes it can directly reach.
//! 
//! # Challenge Questions:
//! 
//! ## What is betweenness centrality? What would it mean in the context of this
//! program?
//!
//! Betweenness centrality is a measure of centrality in a graph based on shortest
//! paths. For every pair of vertices in a connected graph, there are at least one
//! shortest path between the vertices such that either the number of edges that
//! the path passes through (for unweighted graphs) or the sum of the weights of
//! the edges (for weighted graphs) is minimized. The betweenness centrality for
//! each vertex is the number of these shortest paths that pass through the vertex.
//!
//! In the context of this program, which likely involves a graph representing
//! fighters and their relationships (e.g., who has fought whom), betweenness
//! centrality would measure a fighter's influence in the network. A fighter with
//! high betweenness centrality would be one who connects many other fighters
//! together, acting as a central figure in the network. This might indicate a
//! fighter who has fought with many other central fighters or who is critical in
//! the network structure of fights and rivalries.
//! 
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        // Explanation
        match name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                name, closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has the highest centrality of {:.2} as they have fought with the least number of fighters.",
                name, closeness
            ),
            _ => {}
        }
        println!("-----------------");
    }
    
}
