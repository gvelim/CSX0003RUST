# Topological Sort Algorithm
A topological sort is an ordering of the nodes of a directed graph such that if
there is a path from node a to node b, then node a appears before node b in the
ordering.

# Graph Recursion and Processing state
The idea is to go through the nodes of the graph and always begin a depth-first search at the current node if it has not been processed yet. During the searches,
the nodes have three possible states:
* state 0: the node has not been processed (white)
* state 1: the node is under processing (light gray)
* state 2: the node has been processed (dark gray)

Initially, the state of each node is 0. When a search reaches a node for the first time, its state becomes 1. This is our `pre-processing` step for the node

If the graph contains a cycle, we will find this out during the search, because sooner or later we will arrive at a node whose state is 1. In this case, it is not
possible to construct a topological sort. This is the `pre-processing` step for the edge.

If the graph does not contain a cycle, we can construct a topological sort by adding each node to a list when the state of the node becomes 2. This is our `post-processing` step for the node.
This list in reverse order is a topological sort

As a result we can implement the `DFSearch` trait in the following way in relation to the above pre-processing & post-processing steps

```rust,no_run,noplayground
{{#include ../../src/graphs/scc.rs:graphs_topological_sort_state}}
```
# Implementation
When the search has completed and has exhausted all paths the `path` member of the `Tracker` structure will now contain the order by which the nodes have been visited. As a result we only have to `reverse` such order and return it.
```rust,no_run,noplayground
{{#include ../../src/graphs/scc.rs:graphs_topological_sort}}
```