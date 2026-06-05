use community_detection::{count_distinct_users, kosaraju_scc, top_n_users, TWITTER_USERNAMES};
use std::collections::HashMap;

fn main() {
    let mut name_to_id = HashMap::new();
    let mut edges = Vec::new();

    for pair in TWITTER_USERNAMES.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let next_a = name_to_id.len();
        let id_a = *name_to_id.entry(a).or_insert(next_a);
        let next_b = name_to_id.len();
        let id_b = *name_to_id.entry(b).or_insert(next_b);
        edges.push((id_a, id_b));
    }

    let node_count = name_to_id.len();
    let sccs = kosaraju_scc(&edges, node_count);

    let id_to_name: HashMap<usize, &str> = name_to_id.into_iter().map(|(k, v)| (v, k)).collect();

    println!("Community Detection via Kosaraju's SCC Algorithm");
    println!("================================================");
    println!("Total nodes (distinct users): {}", node_count);
    println!("Total edges (retweet pairs):  {}", edges.len());
    println!("Distinct users in dataset:    {}", count_distinct_users());
    println!();
    let top3 = top_n_users(3);
    println!("Top 3 most frequent users:");
    for (name, count) in &top3 {
        println!("  {:<20} {}", name, count);
    }
    println!();
    println!("Strongly Connected Components found: {}\n", sccs.len());

    for (i, comp) in sccs.iter().enumerate() {
        let names: Vec<&str> = comp.iter().map(|id| id_to_name[id]).collect();
        println!("SCC #{} (size {}): {:?}", i + 1, comp.len(), names);
    }
}
