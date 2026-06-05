//! Reflection Quesions:
//! 
//! # How Does the PageRank Algorithm Calculate the Importance of Each Node in
//! the Graph?
//!
//! The PageRank algorithm calculates the importance of each node in a graph
//! based on the structure of incoming links. It operates under the principle
//! that connections to a node are endorsements of that node's importance. The
//! algorithm follows these general steps:
//!
//! - **Initialization**: Each node is assigned an initial rank, usually `1/N`
//!   where `N` is the total number of nodes.
//!
//! - **Iterative Ranking**: For a number of iterations, the rank of each node
//!   is updated based on the ranks of nodes that link to it. The rank
//!   contributed by a linking node is proportional to its rank divided by the
//!   number of links it has.
//!
//! - **Damping Factor**: A damping factor (usually around 0.85) is applied to
//!   simulate the probability that a 'surfer' would continue clicking on links,
//!   with a small chance of jumping to a random node. This prevents ranks from
//!   inflating and deals with rank sinks.
//!
//! - **Normalization**: After the final iteration, ranks are typically
//!   normalized so their sum equals one.
//!
//! The algorithm's result is a vector of ranks that provides a relative measure
//! of importance for each node. Higher ranked nodes are considered more
//! important within the graph based on the structure of incoming links.
//!
//! # What Is the Effect of the Damping Factor on the PageRank Algorithm?
//!
//! The damping factor in the PageRank algorithm is a crucial parameter that
//! affects the ranking process. Representing the probability that a person
//! clicking on links will continue to do so, the damping factor has several
//! effects:
//!
//! - **Rank Distribution**: It influences how rank is distributed through
//!   the network. A higher damping factor places more emphasis on the
//!   structure of the graph, while a lower one means random jumps are more
//!   likely.
//!
//! - **Preventing Rank Sinks**: Without a damping factor, rank could get
//!   trapped in cycles or 'sinks' in the graph. The damping factor ensures
//!   some rank is distributed across the graph, even to nodes without incoming
//!   links.
//!
//! - **Convergence**: The damping factor helps the algorithm to converge by
//!   ensuring there is no rank inflation and that every node receives some
//!   portion of the rank, ultimately leading to a steady state.
//!
//! - **Stability**: It contributes to the stability of PageRank values by
//!   giving a small probability to the event of jumping to a random page, thus
//!   not allowing any single node or tightly-knit community to dominate the
//!   rank distribution.
//!
//! Overall, the damping factor balances the importance of the graph structure
//! with the randomness of user behavior, helping to create a more accurate
//! representation of page importance.
//! 
//! # Purpose of Running the PageRank Algorithm for a Certain Number of Iterations
//!
//! Running the PageRank algorithm for a predetermined number of iterations is
//! critical for several reasons:
//!
//! - **Convergence Assurance**: The iterative process allows the algorithm to
//!   converge to a steady state where subsequent iterations yield minimal
//!   changes in PageRank values.
//!
//! - **Distribution of Rank**: Multiple iterations ensure that the rank is
//!   properly distributed across the graph, allowing each node to influence
//!   others based on the graph's structure.
//!
//! - **Accuracy of Results**: With each iteration, the PageRank value of each
//!   node is adjusted, leading to more accurate results that reflect the true
//!   importance of nodes within the network.
//!
//! - **Damping Factor Implementation**: Iterations incorporate the damping
//!   factor, preventing rank sinks by redistributing rank across all nodes,
//!   even those with no direct links.
//!
//! The number of iterations is often a balance between computational efficiency
//! and the accuracy of the PageRank values. Too few iterations may not allow
//! the algorithm to stabilize, while too many may result in unnecessary
//! computation once convergence is achieved.

use pagerank::{build_inlinks, build_outlinks, page_rank, score_delta, top_pages};
use textwrap::fill;

fn main() {
    // Graph edges: (from, to) — sports website links
    // 0: ESPN, 1: NFL, 2: NBA, 3: UFC, 4: MLB
    let edges: [(usize, usize); 8] = [
        (0, 1), (0, 2),
        (1, 0),
        (2, 0), (2, 3),
        (3, 0),
        (4, 0), (4, 1),
    ];
    let node_count = 5;
    let names = ["ESPN", "NFL", "NBA", "UFC", "MLB"];

    let outlinks = build_outlinks(&edges);
    let inlinks = build_inlinks(&edges);

    println!("Outlinks:");
    for (node, links) in &outlinks {
        println!("  {} ({}): {:?}", names[*node], node, links);
    }
    println!("\nInlinks:");
    for (node, links) in &inlinks {
        println!("  {} ({}): {:?}", names[*node], node, links);
    }

    let ranks = page_rank(&edges, node_count, 0.85, 100);

    println!("\nPageRank scores:");
    for (node, score) in top_pages(&ranks, node_count) {
        println!("  {}: {:.6}", names[node], score);
    }

    let delta = score_delta(&ranks, &page_rank(&edges, node_count, 0.85, 101));
    println!("\nScore delta after one more iteration: {:.2e}", delta);

    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";
    println!("\n{}", fill(explanation, 78));
}
