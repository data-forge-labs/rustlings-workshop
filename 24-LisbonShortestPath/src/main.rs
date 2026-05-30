//! # Reflection Questions:
//! 
//! # How Does Dijkstra's algorithm determine the shortest path between two nodes?
//!
//! Dijkstra's algorithm computes the shortest paths from a starting node to all
//! other nodes in a graph with non-negative edge weights. It operates under the
//! following steps:
//!
//! 1. **Initialization**: The algorithm initializes the distance to the start
//!    node as 0 and all other nodes as infinity. It also maintains a priority
//!    queue (often implemented as a binary heap) that sorts nodes by their
//!    tentative distance value.
//!
//! 2. **Relaxation**: At each step, the node with the smallest distance is
//!    removed from the priority queue. For this node, the algorithm considers
//!    all its unvisited neighbors and calculates their tentative distances
//!    through the current node, comparing it with their current distances.
//!    If a shorter path is found, the algorithm updates the distance for that
//!    neighbor and reinserts the neighbor with the new distance into the
//!    priority queue.
//!
//! 3. **Marking as Visited**: Once all neighbors have been considered, the
//!    current node is marked as visited. A visited node will not be checked
//!    again; its shortest distance from the start node has been finalized.
//!
//! 4. **Termination**: The algorithm repeats the relaxation step until the
//!    priority queue is empty (all nodes visited) or until the destination node
//!    is removed from the priority queue, indicating that the shortest path
//!    to that node has been determined.
//!
//! The result is the shortest path from the start node to all nodes in the
//! graph, or to the destination node if one was specified.
//!
//! # How is the graph created and what does each edge represent?
//!
//! The graph is created using the `Graph` data structure from the `petgraph`
//! library. In this case, the graph is undirected and instantiated with
//! string slices (`&str`) as node weights and unsigned 32-bit integers (`u32`)
//! as edge weights. The `new_undirected` method is used to create an undirected
//! graph.
//!
//! Here is an example of how the graph is created:
//!
//! ```no_run
//! use petgraph::prelude::*;
//!
//! fn main() {
//!     let mut graph = Graph::<&str, u32, Undirected>::new_undirected();
//!     // Nodes and edges would be added here.
//! }
//! ```
//!
//! Each edge in the graph represents a bidirectional relationship or connection
//! between two nodes. In an undirected graph, edges do not have a direction.
//! The edge weights (`u32`) can be used to represent various attributes such as
//! cost, distance, or capacity, depending on the context of the graph's use.
//!
//! # What would happen if there was no route between Belem Tower and Lisbon
//! Cathedral?
//!
//! If no route exists between Belem Tower and Lisbon Cathedral, the
//! `dijkstra` function, which computes the shortest path between nodes in a
//! graph, would not find a path and therefore would return `None` for the
//! distance. In the code, this scenario is handled with an `if let` statement
//! that checks for the existence of a path. If `None` is returned, the code
//! will print "No route found from Belem Tower to Lisbon Cathedral." as seen
//! in the snippet below:
//!
//! ```no_run
//! if let Some(distance) = node_map.get(&lisbon_cathedral) {
//!     println!(
//!         "The shortest distance from Belem Tower to Lisbon Cathedral is {} km",
//!         distance
//!     );
//! } else {
//!     println!("No route found from Belem Tower to Lisbon Cathedral.");
//! }
//! ```
//!
//! This output is produced by the `else` block associated with the `if let`
//! statement in the `main` function when the `dijkstra` algorithm does not
//! find any path from the source node (`belem_tower`) to the target node
//! (`lisbon_cathedral`).
//! 
//! # Challenge Questions:
//! 
//! # Can you think of other real-life applications for Dijkstra's algorithm?
//!
//! Dijkstra's algorithm is a versatile tool that has many real-life
//! applications beyond simple route finding in geographic maps. Here are some
//! examples:
//!
//! - **Network Routing**: In computer networks, Dijkstra's algorithm is used to
//!   determine the shortest path for data packets to travel across a network
//!   from the source to the destination.
//!
//! - **Telecommunications**: In telephone networks, it can be used to find the
//!   least expensive route for a call to travel from one point to another.
//!
//! - **Traffic Engineering**: To manage and predict traffic flow and congestion
//!   in urban areas, Dijkstra's algorithm can optimize the traffic light
//!   timings and suggest the fastest route to commuters.
//!
//! - **Robotics**: Robots use Dijkstra's algorithm to navigate through an
//!   environment with obstacles, finding the shortest path to reach a target
//!   location.
//!
//! - **Flight Scheduling**: Airlines can use it to determine the shortest
//!   path for layover flights to minimize travel time or cost for passengers.
//!
//! - **Gaming**: In video games, non-player characters (NPCs) use pathfinding
//!   algorithms like Dijkstra's to navigate the virtual world.
//!
//! - **Urban Planning**: It can be applied to design the layout of utilities
//!   such as electrical grids, water supply networks, and public transportation
//!   systems for optimal performance and cost efficiency.
//!
//! These applications demonstrate the broad utility of Dijkstra's algorithm in
//! solving various shortest path problems across multiple domains.
//! 
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn main() {
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    let belem_tower = graph.add_node("Belem Tower");
    let monastery = graph.add_node("Jerónimos Monastery");
    let lx_factory = graph.add_node("LX Factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3), // The distance from Belem Tower to LX Factory is 3 km
        (belem_tower, commerce_square, 7), // The distance from Belem Tower to Commerce Square is 7 km
        (monastery, lx_factory, 3), // The distance from Jerónimos Monastery to LX Factory is 3 km
        (monastery, commerce_square, 6), // The distance from Jerónimos Monastery to Commerce Square is 6 km
        (lx_factory, commerce_square, 5), // The distance from LX Factory to Commerce Square is 5 km
        (commerce_square, lisbon_cathedral, 1), // The distance from Commerce Square to Lisbon Cathedral is 1 km
    ]);

    let node_map = dijkstra(&graph, belem_tower, Some(lisbon_cathedral), |e| *e.weight());

    if let Some(distance) = node_map.get(&lisbon_cathedral) {
        println!(
            "The shortest distance from Belem Tower to Lisbon Cathedral is {} km",
            distance
        );
    } else {
        println!("No route found from Belem Tower to Lisbon Cathedral.");
    }
}