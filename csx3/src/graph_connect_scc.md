# Kosaraju’s algorithm
Kosaraju’s algorithm is an efficient method for finding the strongly connected components of a directed graph.
## Approach
The algorithm performs two depth-first searches
1. the first search constructs an **ordered node list** of nodes according to the structure of the graph.  
2. the second search applies the **ordered node list** against the **reversed edges** of the graph in order to find the strongly connected components.
## Graph Recursion and Processing State
In the **first** Depth First Search we need to calculate per node
* the exit `time`, that is, the time in which the node has been `Processed`, that is, there is nothing left to be found.
* the node state in relation to any of the states, `Undiscovered`, `Discovered` or `Processed`

Recursion is a key implementation approach that will enable us to perform
1. Node pre-processing, e.g. capture/log the `entry` time and before search any deeper
2. Node post-processing, e.g. capture/log the `exit` time after there is no path remaining to be found from this node

As a result to measure time across recursions and without the use of a `global` variable, we resort to the `GraphState` struct that
* implements the [`DFSearch` trait](graph_path_dfs_abstract.md) that provides the recursive function
* holds the recursion state for `time`, `path` at node, node `state` & `ordered list`

In addition, `GraphState` provide us with the `Tracker` structure that simplifies handling of the [node processing state](graph_search_process_state.md) while we are search the graph.

```rust,no_run,noplayground
{{#include ../../src/graphs/scc.rs:graphs_scc_state}}
```
## Transpose the graph
The `GraphState` will help us capture the node order by which we will run search on the second pass. However, the second pass must run against the **transposed graph**, that is, the graph with all edges reversed.
```rust,no_run,noplayground
{{#include ../../src/graphs/scc.rs:graphs_scc_traversal}}
```
## Final implementation
With all of the above elements in place, The below function provides an implementation approach to the algorithm 
```rust,no_run,noplayground
{{#include ../../src/graphs/scc.rs:graphs_scc}}
```
