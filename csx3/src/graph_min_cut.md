# DRAFT: Karger's Minimum Cut Algorithm
Karger's algorithm is a randomized algorithm to compute a minimum cut of a connected graph. The algorithm will ***randomly contract*** the graph a number of times in order to identify the minimum number of edges that when removed will cause the graph to split into two disjoint subsets that is minimal in some metric.

## Minimum Cut Algorithm
```rust,no_run,noplayground
{{#include ../../src/graphs/min_cut.rs:graphs_min_cut}}
```

## Contraction Algorithm
```rust,no_run,noplayground
{{#include ../../src/graphs/min_cut.rs:graphs_contraction}}
```
