//! # Reflection Questions:
//! 
//! # Detecting Strongly Connected Components in a Directed Graph Using Kosaraju's
//! Algorithm
//!
//! Kosaraju's algorithm is a classical approach to find strongly connected 
//! components (SCCs) in a directed graph. It is based on the idea that a DFS 
//! traversal reveals ordering information that can be used to identify SCCs.
//!
//! The algorithm proceeds as follows:
//!
//! 1. Perform a Depth-First Search (DFS) on the original graph. As each vertex
//!    finishes, push it onto a stack. This step essentially computes a 
//!    post-order of the vertices.
//! 2. Reverse all the edges in the graph to create the transpose graph.
//! 3. While the stack is not empty, pop the top vertex, perform a DFS on the 
//!    transpose graph starting with that vertex, and mark all reachable vertices.
//!    Each set of reachable vertices from a single starting point forms one 
//!    strongly connected component.
//!
//! The result of this process will be the decomposition of the graph into its 
//! strongly connected components.
//! 
//! # How are Twitter users and their interactions represented in the graph?
//!
//! In the codebase, Twitter users and their interactions might be represented in a
//! graph structure using the `petgraph` library. Users are likely represented as 
//! nodes within the graph, and interactions between users are represented as edges.
//!
//! The `TWITTER_USERNAMES` constant may contain a list of usernames, which are 
//! used to create nodes in the graph. The interactions, such as follows or 
//! mentions, could be directed edges connecting these nodes. The direction of an 
//! edge could indicate the action's origin and target (e.g., user A follows user 
//! B). The `petgraph` library's `DiGraph` type is suitable for representing such 
//! directed relationships.
//!
//! Without more context about how the edges are constructed and what types of 
//! interactions are tracked, this is a general description based on the typical 
//! usage of graphs to represent social networks and the `petgraph` crate's 
//! capabilities.
//! 
//! # What is the significance of the strongly connected components in the context of
//! social network analysis?
//!
//! Strongly connected components (SCCs) in social network analysis are significant 
//! because they indicate groups of users who are all directly or indirectly 
//! connected to each other through directed paths. In the context of Twitter, an 
//! SCC may represent a community or a circle of users who all follow each other, 
//! suggesting a tight-knit group with common interests or affiliations.
//!
//! Identifying SCCs helps in understanding the structure of the social network:
//!
//! - **Community Detection**: SCCs can be considered as communities within the 
//!   larger network. Understanding these communities can be valuable for targeted 
//!   marketing, information dissemination, or understanding group dynamics.
//! - **Influence Spread**: In an SCC, information can circulate among all members, 
//!   making these components critical for understanding how information, trends, or 
//!   opinions spread in a network.
//! - **Network Robustness**: SCCs can also give insights into the robustness of 
//!   the network. A network with many SCCs may be less robust to node removal 
//!   since disconnecting a node might fragment the component.
//!
//! Analyzing SCCs provides a deeper insight into the relational dynamics of social 
//! networks, which can be applied to various fields such as sociology, marketing, 
//! political science, and information technology.
//! 
//! # Challenges Questions:
//! 
//! # Can you think of other real-life applications for community detection algorithms?
//!
//! Community detection algorithms have a wide range of real-life applications 
//! beyond social network analysis. These algorithms can uncover structured groups 
//! within large sets of data, which is valuable in various domains:
//!
//! - **Biology**: Detecting communities of genes or proteins that interact with 
//!   each other can help in understanding biological processes and disease 
//!   mechanisms.
//! - **Market Segmentation**: Businesses can use community detection to identify 
//!   groups of consumers with similar buying habits or preferences for more 
//!   targeted advertising and product development.
//! - **Fraud Detection**: In financial networks, community detection can help 
//!   identify clusters of fraudulent activity, such as unusual patterns of 
//!   transactions that could indicate collusion or money laundering.
//! - **Infrastructure Analysis**: Identifying tightly-knit communities within 
//!   utility networks, like power grids or water supply networks, can aid in 
//!   optimizing performance and robustness.
//! - **Transportation**: Analyzing traffic flow to detect communities can reveal 
//!   areas with high connectivity that might benefit from infrastructure 
//!   improvements or inform urban planning.
//! - **Recommendation Systems**: Community detection can improve recommendation 
//!   engines by identifying groups of users with similar interests, leading to 
//!   more accurate and personalized content recommendations.
//!
//! These applications demonstrate the versatility of community detection 
//! algorithms in providing insights into complex systems across various fields.
//! 
use community_detection::TWITTER_USERNAMES;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;
use std::collections::HashMap;

fn main() {
    // Create a new directed Graph
    let mut graph = DiGraph::<&str, &str>::new();

    // Create a HashMap to store node indices by user name
    let mut nodes = HashMap::new();

    // Iterate over the data to populate the graph
    for window in TWITTER_USERNAMES.windows(2) {
        let user = window[0];
        let mention = window[1];

        // Add the nodes to the graph and to the HashMap
        let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));
        let mention_node = *nodes
            .entry(mention)
            .or_insert_with(|| graph.add_node(mention));

        // Add the edge to the graph
        graph.add_edge(user_node, mention_node, "retweets");
    }

    // Use the Kosaraju's algorithm to detect strongly connected components
    let scc = kosaraju_scc(&graph);
    for component in scc {
        println!("{} nodes in community discovered", component.len());
        let usernames: Vec<&str> = component
            .iter()
            .map(|&node_index| graph[node_index])
            .collect();
        println!("{:?}", usernames);
    }
}
