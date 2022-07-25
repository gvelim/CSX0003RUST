# DRAFT: Karger's Minimum Cut Algorithm
Karger's algorithm is a randomized algorithm to compute a minimum cut of a connected graph. The algorithm will ***randomly contract*** the graph a number of times in order to identify the minimum number of edges that when removed will cause the graph to split into two disjoint subsets that is minimal in some metric.

## Minimum Cut Algorithm
The algorithm performs the following steps
* **STEP 1** : calculate number of iterations
* **STEP 2** : Contract the graph N times 
* **STEP 3** : Return the minimum cut found
```rust,no_run,noplayground
{{#include ../../src/graphs/min_cut.rs:graphs_min_cut}}
```

## Contraction Algorithm
The algorithm performs the following steps
* **STEP 1**: INITIALISE temporary super node and super edge structures
* **STEP 2**: CONTRACT the graph, until 2 super nodes are left
    * **STEP A**: select a random edge
    * **STEP B** : Contract the edge by merging the edge's nodes
    * **STEP C** : Collapse/Remove newly formed edge loops since src & dst is the new super node
    * **STEP D** : Identify all edges affected due to the collapsing of nodes
    * **STEP E** : Repoint affected edges to the new super node
* **STEP 3** : find the edges between the two super node sets
```rust,no_run,noplayground
{{#include ../../src/graphs/min_cut.rs:graphs_contraction}}
```
## Finding Graph Edges between two sets of Nodes
The below function returns the edges of a graph given two sets of nodes
```rust,no_run,noplayground
{{#include ../../src/graphs/min_cut.rs:graphs_crossing}}
```
