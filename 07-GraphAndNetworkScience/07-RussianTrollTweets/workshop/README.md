# Workshop: Russian Troll Tweets

**Goal**: Study the implemented `src/lib.rs` and pass all 15 tests.

## Functions to Study

### `build_retweet_graph`
- **Signature**: `pub fn build_retweet_graph(tweets: &[Tweet]) -> HashMap<String, Vec<String>>`
- **Task**: Build a directed retweet graph from a list of tweets. Already implemented.

### `tweet_count_by_user`
- **Signature**: `pub fn tweet_count_by_user(tweets: &[Tweet]) -> HashMap<String, usize>`
- **Task**: Count tweets per user. Already implemented.

### `top_users`
- **Signature**: `pub fn top_users(tweet_counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)>`
- **Task**: Return the top `n` most active users. Already implemented.

### `can_reach`
- **Signature**: `pub fn can_reach(graph: &HashMap<String, Vec<String>>, from: &str, to: &str) -> bool`
- **Task**: Check if `to` is reachable from `from` in the graph using BFS. Already implemented.

### `influence_score`
- **Signature**: `pub fn influence_score(graph: &HashMap<String, Vec<String>>, user: &str) -> usize`
- **Task**: Count how many users retweeted the given user. Already implemented.

### `find_bridge_users`
- **Signature**: `pub fn find_bridge_users(graph: &HashMap<String, Vec<String>>) -> Vec<String>`
- **Task**: Find users that bridge multiple connected components. Already implemented.

## Structs

### `Tweet`
- Fields: `user: String`, `content: String`, `retweet_of: Option<String>`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_data_modeling | 3 | Retweet graph construction |
| step_02_analysis | 5 | Tweet counts, top users, influence scores |
| step_03_graph_traversal | 7 | Reachability, bridge users in retweet graph |

## How to Run Tests
```bash
cargo test
```
