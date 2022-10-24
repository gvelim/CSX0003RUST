# Abstracting Depth First Search
Depth First Search is applied on a number of algorithms with the same pattern

1. Do some work on the node `before` exploring any paths
2. For edge of the node
   1. Process the edge `before` performing search on it
   2. Perform search on the edge
   3. Process the edge `after` all paths from this edge, got explored 
3. Do some work on the node `after` all paths from this node, got explored

Different algorithms have different demands on how what the graph state should be and which of those steps are required and in what way

The above can be observed on how the `Graph State` realises the `DFSearch` trait for [Topological sort](graph_path_topological_sort.md) and [Strongly Connected Components](graph_connect_scc.md) implementation

## Implementation
As a result, we can define a `trait` for any `Graph State` structure, that provide the means of how the pre-processing / post-processing steps should be performed and in relation to the required state and behaviour.

It is important to note here the recursive nature of the search and hence the need for `self` to maintain the internal state while recursively searching the graph

```rust,no_run,noplayground
{{#include ../../src/graphs/scc.rs:graphs_abstract_dfs}}
```