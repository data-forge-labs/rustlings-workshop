# Project 27: Social Graph Analysis -- Retweet Network Influence Detection

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 16 tests pass**.

## Why Analyze a Retweet Graph Without Pandas?

**Python pain:** A typical social-graph pipeline loads CSV into pandas, converts to `networkx`, then runs analysis. Each conversion copies data and creates Python wrappers. For 100K tweets, that's 300K+ Python objects and ~8s of overhead:

```
Python pandas + networkx (100K tweets):  ~8 seconds
Rust HashMap + Vec (100K tweets):        ~0.1 seconds
```

**Rust fix:** Build the graph in a single pass into `HashMap<String, Vec<String>>` — no intermediate library, no copy, no GC pauses:

```rust
pub fn build_retweet_graph(tweets: &[Tweet]) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    for tweet in tweets {
        if let Some(ref original) = tweet.retweet_of {
            graph.entry(tweet.user.clone()).or_default().push(original.clone());
        }
        graph.entry(tweet.user.clone()).or_default();
    }
    graph
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Optional fields | `Option<String>` | `Optional[str]` | Model nullable retweet_of field |
| 2 | Retweet graph | `HashMap<String, Vec<String>>` | `defaultdict(list)` | Build directed graph from tweet data |
| 3 | Counting with HashMap | `entry().or_insert(0) += 1` | `collections.Counter` | Count tweets per user |
| 4 | Top-N sorting | `sort_by` desc, `truncate` | `Counter.most_common(n)` | Find most active users |
| 5 | BFS reachability | `VecDeque` + `HashSet` | `deque` + `set` | Check info flow A → B |
| 6 | Influence score | count incoming edges | list comprehension | Measure how many retweeted a user |
| 7 | Bridge detection | BFS component labeling | connected components | Find users connecting communities |

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Tweet Data Model](#3-concept-tweet-data-model)
4. [Concept: Retweet Graph Construction](#4-concept-retweet-graph-construction)
5. [Concept: User Activity Analysis](#5-concept-user-activity-analysis)
6. [Concept: BFS Reachability](#6-concept-bfs-reachability)
7. [Concept: Influence Score and Bridge Users](#7-concept-influence-score-and-bridge-users)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Summary](#9-summary)

## 1. Introduction

Social network analysis helps identify influential users, information flow patterns, and bridge users who connect otherwise disconnected communities. The Russian Troll Tweets dataset contains tweets from accounts associated with influence operations.

You will build a retweet graph from tweet data, then analyze it to find:
- Who tweets the most (activity ranking)
- Who gets retweeted the most (influence score)
- Bridge users who connect different parts of the network
- Whether information can flow from one user to another (reachability)

In Python, you would use `networkx` for graph analysis. In Rust, you build the graph from raw tweet data and implement BFS traversal manually.

## 2. Prerequisites

- Structs with `Option<String>` fields
- `HashMap`, `HashSet`, `VecDeque`
- BFS traversal pattern

## 3. Concept: Tweet Data Model

### Explanation

Each tweet has a user, content, and an optional `retweet_of` field indicating whose tweet was retweeted. In Python:

```python
from dataclasses import dataclass
from typing import Optional

@dataclass
class Tweet:
    user: str
    content: str
    retweet_of: Optional[str]
```

In Rust:

```rust
pub struct Tweet {
    pub user: String,
    pub content: String,
    pub retweet_of: Option<String>,
}
```

In Python, `Optional[str]` means the field can be a string or `None`. In Rust, `Option<String>` is either `Some(String)` or `None`. A `retweet_of: None` means this is an original tweet, not a retweet.

## 4. Concept: Retweet Graph Construction

### Explanation

A retweet graph is a directed graph where an edge `A -> B` means "A retweeted B". Nodes are users, and edges represent information flow.

In Python:

```python
def build_retweet_graph(tweets):
    graph = defaultdict(list)
    for t in tweets:
        if t.retweet_of is not None:
            graph[t.user].append(t.retweet_of)
            graph.setdefault(t.retweet_of, [])  # ensure node exists
        else:
            graph.setdefault(t.user, [])
    return graph
```

In Rust, we use `HashMap<String, Vec<String>>`:

```rust
pub fn build_retweet_graph(tweets: &[Tweet]) -> HashMap<String, Vec<String>>
```

For each tweet:
- If it's a retweet (`retweet_of = Some(original)`): add edge from current user to original, and ensure original has an entry
- If it's original (`retweet_of = None`): just ensure the user has an entry

### Applying to Our Project

A chain `alice -> bob -> carol` (bob retweets alice, carol retweets bob) creates edges `bob -> alice` and `carol -> bob`.

## 5. Concept: User Activity Analysis

### Explanation

Counting tweets per user and finding the top N most active users. In Python:

```python
from collections import Counter

def tweet_count_by_user(tweets):
    return Counter(t.user for t in tweets)

def top_users(counts, n):
    return counts.most_common(n)
```

In Rust:

```rust
pub fn tweet_count_by_user(tweets: &[Tweet]) -> HashMap<String, usize>
pub fn top_users(tweet_counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)>
```

`tweet_count_by_user` increments a `HashMap` counter for each tweet. `top_users` collects into a `Vec`, sorts by count descending, and truncates.

## 6. Concept: BFS Reachability

### Explanation

Can information flow from user X to user Y? This is a BFS reachability query on the retweet graph. In Python:

```python
from collections import deque

def can_reach(graph, start, target):
    if start == target:
        return True
    visited = {start}
    queue = deque([start])
    while queue:
        current = queue.popleft()
        for neighbor in graph.get(current, []):
            if neighbor == target:
                return True
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)
    return False
```

In Rust with `VecDeque`:

```rust
pub fn can_reach(graph: &HashMap<String, Vec<String>>, from: &str, to: &str) -> bool
```

- If `from == to`, return `true` immediately
- Use `HashSet<String>` for visited and `VecDeque<String>` for BFS queue
- Return `true` as soon as `to` is found

### Applying to Our Project

In a chain `carol -> bob -> alice`, `can_reach(carol, alice)` returns true (carol retweeted bob, who retweeted alice). But `can_reach(alice, carol)` returns false because edges go in the retweet direction only.

## 7. Concept: Influence Score and Bridge Users

### Explanation

**Influence score**: how many users directly retweeted a given user. This is simply the number of incoming edges:

```rust
pub fn influence_score(graph: &HashMap<String, Vec<String>>, user: &str) -> usize
```

In Python: `sum(1 for targets in graph.values() if user in targets)`

**Bridge users**: users who appear in multiple connected components of the graph. The `find_bridge_users` function:

1. Runs BFS to identify connected components, assigning each node a component ID
2. Tracks which components each user belongs to (based on their neighbors)
3. A user whose neighbors span multiple components is a bridge

```rust
pub fn find_bridge_users(graph: &HashMap<String, Vec<String>>) -> Vec<String>
```

### Applying to Our Project

A bridge user might be the only person retweeting both political accounts and journalist accounts, bridging two otherwise disconnected communities.

## 8. Putting It All Together

Open `workshop/src/lib.rs` and implement:

1. **`build_retweet_graph`** -- iterate tweets, add edges for retweets, ensure all users are keys
2. **`tweet_count_by_user`** -- `HashMap<String, usize>` with `entry().or_insert(0) += 1`
3. **`top_users`** -- collect into `Vec`, sort by count descending, truncate
4. **`can_reach`** -- BFS with `VecDeque` and `HashSet`, return `true` when target found
5. **`influence_score`** -- count how many neighbor lists contain the user
6. **`find_bridge_users`** -- assign component IDs via BFS, find users whose neighbors span multiple components

Run `cd workshop && cargo test` after each step. Groups: `step_01_data_modeling` (3 tests), `step_02_analysis` (5 tests), `step_03_graph_traversal` (8 tests).

## 9. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| Optional field | `Option<String>` | `Optional[str]` | `Tweet.retweet_of` |
| Retweet graph | `HashMap<String, Vec<String>>` | `defaultdict(list)` | `build_retweet_graph` |
| Counting | `HashMap` with `entry().or_insert(0) += 1` | `collections.Counter` | `tweet_count_by_user` |
| Top-N sorting | `sort_by(|a,b| b.1.cmp(&a.1))`, truncate | `Counter.most_common(n)` | `top_users` |
| BFS reachability | `VecDeque` + `HashSet` | `deque` + `set` | `can_reach` |
| Influence score | `iter().filter(|(_, t)| t.contains(user)).count()` | List comprehension | `influence_score` |
| Bridge detection | BFS component labeling, multi-component check | Connected components | `find_bridge_users` |
