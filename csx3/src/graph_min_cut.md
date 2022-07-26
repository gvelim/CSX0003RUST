# DRAFT: Karger's Minimum Cut Algorithm
Karger's algorithm is a randomized algorithm to compute a minimum cut of a connected graph. The algorithm will ***randomly contract*** the graph a number of times in order to identify the minimum number of edges that when removed will cause the graph to split into two disjoint subsets that is minimal in some metric.


## Minimum Cut Algorithm
The algorithm performs the following steps
* **STEP 1** : calculate number of iterations
* **STEP 2** : Perform N contractions of the graph and record minimum-cut per contraction 
* **STEP 3** : Return the smallest minimum-cut recorded

The below image shows 10 repetitions of the contraction procedure. The 5th repetition finds the minimum cut of size 3
![image](img/10_repetitions_of_Karger’s_contraction_procedure.svg.png)
The below function provides an implementation approach to minimum-cut algorithm  
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

A visual example is the below image which shows the successful run of the contraction algorithm on a 10-vertex graph. The minimum cut has size 3.
![image](img/Single_run_of_Karger’s_Mincut_algorithm.svg.png)
The below function provides an implementation approach to the contraction algorithm
```rust,no_run,noplayground
{{#include ../../src/graphs/min_cut.rs:graphs_contraction}}
```
## Finding Graph Edges between two sets of Nodes
The below function returns the edges of a graph given two sets of nodes
```rust,no_run,noplayground
{{#include ../../src/graphs/min_cut.rs:graphs_crossing}}
```
