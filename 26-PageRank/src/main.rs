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

// Importing the fill function from the textwrap crate to wrap text at 78 characters per line.
use textwrap::fill;

// The PageRank struct holds the damping factor and the number of iterations to run the algorithm.
struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    // The new function creates a new instance of the PageRank struct.
    fn new(damping: f64, iterations: usize) -> Self {
        Self { damping, iterations }
    }

    // The rank function calculates and returns the PageRank for each node in the graph.
    fn rank(&self, graph: &Vec<Vec<usize>>) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.len();

        // The initial PageRank value for each node.
        let mut ranks = vec![1.0 / (n as f64); n];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank values.
            let mut new_ranks = vec![0.0; n];

            // Iterates over each node and its edges in the graph.
            for (node, edges) in graph.iter().enumerate() {
                // The amount of PageRank value this node contributes to its linked nodes.
                let contribution = ranks[node] / (edges.len() as f64);

                // Distributes the PageRank value to the linked nodes.
                for &edge in edges {
                    new_ranks[edge] += contribution;
                }
            }

            // Updates the PageRank values using the damping factor.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            // Replaces the old PageRank values with the new ones.
            ranks = new_ranks;
        }

        // Returns the final PageRank values.
        ranks
    }
}

fn main() {
    // The graph represents links between sports websites. Each index represents a website, 
    // and the values in the vectors are the indexes of the websites they link to.
    let graph = vec![
        vec![1, 2],  // ESPN links to NFL, NBA
        vec![0],     // NFL links to ESPN
        vec![0, 3],  // NBA links to ESPN, UFC
        vec![0],     // UFC links to ESPN
        vec![0, 1],  // MLB links to ESPN, NFL
    ];
    
    // The names corresponding to the indexes of the websites.
    let names = vec!["ESPN", "NFL", "NBA", "UFC", "MLB"];

    // Initializes the PageRank struct.
    let pagerank = PageRank::new(0.85, 100);

    // Calculates the PageRank values.
    let ranks = pagerank.rank(&graph);  

    // Prints the PageRank values.
    println!("The PageRank values are:");
    for (i, rank) in ranks.iter().enumerate() {
        println!("{}: {}", names[i], rank);
    }
    
    // Prints the PageRank values.
    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", names[i], rank);
    }

    // Explanation of how PageRank works.
    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";
    
    // Prints the explanation wrapped at 78 characters per line.
    println!("{}", fill(explanation, 78));
}
