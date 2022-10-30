# Abstracting Breadth First Search
Breadth First Search is applied on a number of algorithms with the same pattern, that is:

1. Initiate processing `state` with starting node
2. Do some work on the node `before` exploring any paths
3. For each edge off this node
    1. Process the edge `before` performing search on it
    2. Push edge node on the queue for further processing
4. Do some work on the node `after` all edges has been discovered

Different path search algorithms have different demands in terms of 
* Queue type, that is, FIFO, LIFO, etc 
* Initiation states for starting node
* Pre-processing & post-processing logic required for nodes and edges

The above can be observed on how the `Graph State` realises the `BFSearch` trait for [Minimum Path Cost](./graph_path_minimum_cost.md) and [Shortest Distance](./graph_path_shortest_distance.md) implementation

## Implementation
As a result, we can define a `trait` for any `Graph State` structure, that provides the means of how the `queue` processing, `node` & `edge` pre-processing / post-processing steps should be performed and in relation to the required state and behaviour.

```rust,no_run,noplayground
{{#include ../../src/graphs/path_search.rs:graphs_search_bfs_abstraction}}
```