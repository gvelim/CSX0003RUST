# Processing state of graph nodes
In most case, we have to maintain some form of processing state while we perform a graph search. The most common processing data that we need to calculate and store is
* A node's visiting `state`
* The `parent` node, that is, the node we are visiting from
* A unit in terms of `cost` or `distance`

The `Tracker` structure holds the processed graph information while provides the means to
* access a node as an index in the form of `tracker[node]`
* set and update the path cost to a specific node
* set and update the parent node for the path cost calculation
* extract the minimum cost path given a target node
  The below code implements the above functionality
```rust,no_run,noplayground
{{#include ../../src/graphs/path_search.rs:graphs_search_path_utils_NodeTrack}}
```
To initialise the `Tracker` we use the `Graph` structure
```rust,no_run,noplayground
{{#include ../../src/graphs/path_search.rs:graphs_search_path_utils_NodeTrack_graph}}
```
