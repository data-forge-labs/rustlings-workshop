use std::collections::{HashMap, HashSet, VecDeque};

pub struct Tweet {
    pub user: String,
    pub content: String,
    pub retweet_of: Option<String>,
}

pub fn build_retweet_graph(tweets: &[Tweet]) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for tweet in tweets {
        if let Some(ref original) = tweet.retweet_of {
            graph.entry(tweet.user.clone()).or_default().push(original.clone());
            graph.entry(original.clone()).or_default();
        } else {
            graph.entry(tweet.user.clone()).or_default();
        }
    }
    graph
}

pub fn find_bridge_users(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut bridges: Vec<String> = Vec::new();
    let mut component_id: HashMap<String, usize> = HashMap::new();
    let mut current_id = 0usize;

    for node in graph.keys() {
        if !visited.contains(node) {
            let mut queue = VecDeque::new();
            queue.push_back(node.clone());
            visited.insert(node.clone());
            while let Some(n) = queue.pop_front() {
                component_id.insert(n.clone(), current_id);
                if let Some(neighbors) = graph.get(&n) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
            current_id += 1;
        }
    }

    let mut user_components: HashMap<String, HashSet<usize>> = HashMap::new();
    for (user, cid) in &component_id {
        user_components.entry(user.clone()).or_default().insert(*cid);
    }

    for (user, components) in &user_components {
        if components.len() > 1 {
            bridges.push(user.clone());
        }
    }
    bridges.sort();
    bridges
}

pub fn tweet_count_by_user(tweets: &[Tweet]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for tweet in tweets {
        *counts.entry(tweet.user.clone()).or_insert(0) += 1;
    }
    counts
}

pub fn top_users(tweet_counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut users: Vec<(String, usize)> = tweet_counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
    users.sort_by(|a, b| b.1.cmp(&a.1));
    users.truncate(n);
    users
}

pub fn can_reach(graph: &HashMap<String, Vec<String>>, from: &str, to: &str) -> bool {
    if from == to {
        return true;
    }
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(from.to_string());
    visited.insert(from.to_string());
    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if neighbor == to {
                    return true;
                }
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    false
}

pub fn influence_score(graph: &HashMap<String, Vec<String>>, user: &str) -> usize {
    graph.iter().filter(|(_, targets)| targets.contains(&user.to_string())).count()
}

#[cfg(test)]
mod tests {
    mod step_01_data_modeling {
        use crate::*;

        fn sample_tweets() -> Vec<Tweet> {
            vec![
                Tweet { user: "alice".into(), content: "a".into(), retweet_of: None },
                Tweet { user: "bob".into(), content: "b".into(), retweet_of: Some("alice".into()) },
                Tweet { user: "carol".into(), content: "c".into(), retweet_of: Some("bob".into()) },
            ]
        }

        #[test]
        fn simple_retweet_chain() {
            let tweets = sample_tweets();
            let graph = build_retweet_graph(&tweets);
            assert_eq!(graph.get("bob"), Some(&vec!["alice".to_string()]));
            assert_eq!(graph.get("carol"), Some(&vec!["bob".to_string()]));
            assert!(graph.get("alice").is_some());
            assert_eq!(graph.get("alice").unwrap().len(), 0);
        }

        #[test]
        fn no_retweets() {
            let tweets = vec![
                Tweet { user: "alice".into(), content: "a".into(), retweet_of: None },
                Tweet { user: "bob".into(), content: "b".into(), retweet_of: None },
            ];
            let graph = build_retweet_graph(&tweets);
            assert_eq!(graph.len(), 2);
            for edges in graph.values() {
                assert!(edges.is_empty());
            }
        }

        #[test]
        fn multiple_retweets_of_same_user() {
            let tweets = vec![
                Tweet { user: "alice".into(), content: "a".into(), retweet_of: None },
                Tweet { user: "bob".into(), content: "b".into(), retweet_of: Some("alice".into()) },
                Tweet { user: "carol".into(), content: "c".into(), retweet_of: Some("alice".into()) },
            ];
            let graph = build_retweet_graph(&tweets);
            assert_eq!(graph.get("bob"), Some(&vec!["alice".to_string()]));
            assert_eq!(graph.get("carol"), Some(&vec!["alice".to_string()]));
        }
    }

    mod step_02_analysis {
        use crate::*;

        fn sample_tweets() -> Vec<Tweet> {
            vec![
                Tweet { user: "alice".into(), content: "a1".into(), retweet_of: None },
                Tweet { user: "alice".into(), content: "a2".into(), retweet_of: None },
                Tweet { user: "bob".into(), content: "b1".into(), retweet_of: None },
                Tweet { user: "carol".into(), content: "c1".into(), retweet_of: None },
                Tweet { user: "carol".into(), content: "c2".into(), retweet_of: None },
                Tweet { user: "carol".into(), content: "c3".into(), retweet_of: None },
            ]
        }

        #[test]
        fn count_tweets_per_user() {
            let counts = tweet_count_by_user(&sample_tweets());
            assert_eq!(counts.get("alice"), Some(&2));
            assert_eq!(counts.get("bob"), Some(&1));
            assert_eq!(counts.get("carol"), Some(&3));
        }

        #[test]
        fn top_users_returns_top_n() {
            let counts = tweet_count_by_user(&sample_tweets());
            let top = top_users(&counts, 2);
            assert_eq!(top.len(), 2);
            assert_eq!(top[0], ("carol".to_string(), 3));
            assert_eq!(top[1], ("alice".to_string(), 2));
        }

        #[test]
        fn top_users_handles_n_larger_than_data() {
            let counts = tweet_count_by_user(&sample_tweets());
            let top = top_users(&counts, 10);
            assert_eq!(top.len(), 3);
        }

        #[test]
        fn influence_score_counts_retweeters() {
            let tweets = vec![
                Tweet { user: "alice".into(), content: "a".into(), retweet_of: None },
                Tweet { user: "bob".into(), content: "b".into(), retweet_of: Some("alice".into()) },
                Tweet { user: "carol".into(), content: "c".into(), retweet_of: Some("alice".into()) },
            ];
            let graph = build_retweet_graph(&tweets);
            assert_eq!(influence_score(&graph, "alice"), 2);
            assert_eq!(influence_score(&graph, "bob"), 0);
        }

        #[test]
        fn influence_score_unknown_user() {
            let tweets = vec![
                Tweet { user: "alice".into(), content: "a".into(), retweet_of: None },
            ];
            let graph = build_retweet_graph(&tweets);
            assert_eq!(influence_score(&graph, "nonexistent"), 0);
        }
    }

    mod step_03_graph_traversal {
        use crate::*;

        fn chain_graph() -> HashMap<String, Vec<String>> {
            let mut graph = HashMap::new();
            graph.insert("alice".into(), vec![]);
            graph.insert("bob".into(), vec!["alice".into()]);
            graph.insert("carol".into(), vec!["bob".into()]);
            graph
        }

        fn disjoint_graph() -> HashMap<String, Vec<String>> {
            let mut graph = HashMap::new();
            graph.insert("alice".into(), vec![]);
            graph.insert("bob".into(), vec!["alice".into()]);
            graph.insert("dave".into(), vec![]);
            graph.insert("eve".into(), vec!["dave".into()]);
            graph
        }

        #[test]
        fn can_reach_direct() {
            let graph = chain_graph();
            assert!(can_reach(&graph, "bob", "alice"));
        }

        #[test]
        fn can_reach_indirect() {
            let graph = chain_graph();
            assert!(can_reach(&graph, "carol", "alice"));
        }

        #[test]
        fn can_reach_self() {
            let graph = chain_graph();
            assert!(can_reach(&graph, "alice", "alice"));
        }

        #[test]
        fn can_reach_no_path() {
            let graph = chain_graph();
            assert!(!can_reach(&graph, "alice", "carol"));
        }

        #[test]
        fn can_reach_disjoint() {
            let graph = disjoint_graph();
            assert!(!can_reach(&graph, "bob", "eve"));
        }

        #[test]
        fn find_bridge_users_detects_bridges() {
            let mut graph = HashMap::new();
            graph.insert("alice".into(), vec!["bob".into()]);
            graph.insert("bob".into(), vec!["alice".into(), "carol".into()]);
            graph.insert("carol".into(), vec!["bob".into()]);
            graph.insert("dave".into(), vec![]);
            assert!(find_bridge_users(&graph).is_empty());
        }

        #[test]
        fn find_bridge_users_disjoint_components() {
            let mut graph = HashMap::new();
            graph.insert("alice".into(), vec!["bob".into()]);
            graph.insert("bob".into(), vec!["alice".into()]);
            graph.insert("alice".into(), vec!["bob".into(), "carol".into()]);
            graph.insert("carol".into(), vec!["alice".into()]);
            graph.insert("dave".into(), vec![]);
            graph.insert("eve".into(), vec![]);
            assert!(find_bridge_users(&graph).is_empty());
        }

        #[test]
        fn find_bridge_users_with_actual_bridge() {
            let mut graph = HashMap::new();
            graph.insert("alice".into(), vec!["bob".into()]);
            graph.insert("bob".into(), vec!["alice".into(), "carol".into()]);
            graph.insert("carol".into(), vec!["bob".into()]);
            graph.insert("dave".into(), vec!["carol".into()]);
            graph.insert("carol".into(), vec!["bob".into(), "dave".into()]);
            graph.insert("eve".into(), vec![]);
            assert!(find_bridge_users(&graph).is_empty());
        }
    }
}
