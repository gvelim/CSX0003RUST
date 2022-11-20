# Single-linkage clustering
Single-linkage clustering is one of several methods of hierarchical clustering. It is based on grouping clusters in bottom-up fashion (agglomerative clustering), at each step combining two clusters that contain the closest pair of elements not yet belonging to the same cluster as each other. A drawback of this method is that it tends to produce long thin clusters in which nearby elements of the same cluster have small distances, but elements at opposite ends of a cluster may be much farther from each other than two elements of other clusters. This may lead to difficulties in defining classes that could usefully subdivide the data[^note]

## Approach
At the start all points form their own component. Then at each iteration, we fuse together those components that are connected by the shortest distance edge. We repeat until the number of components left is equal to the number of clusters required.

This is exactly how the [Kruska's algorithm](graph_mst_kruskal.md) works, with the only difference, the produced Minimum Spanning Tree can be seen as a single component / cluster, therefore we have to stop the process until `k` components/clusters are left.

However, if we stop the at `k` components our min spanning tree won't have the remaining edges connecting the clusters, hence we won't know the clusters' `spacing`, that is, the distance between the closest together pair of separated nodes.

## ClusterSet Structure
Therefore, we need to capture both (a) the min spanning tree and (b) the nodes forming the `k` clusters requested

The `ClusterSet` structure captures such information and further provides the means to query the **spacing of a clustering**, through the use of the following functions, 
* `crossing_edges()` returns those `mst` edges crossing the node `clusters`
* `spacing()` returns the smallest `mst` edge
```rust,no_run,noplayground
{{#include ../../src/greedy/cluster.rs:graphs_mst_cluster_def}}
```

## Implementation
With the `ClusterSet` data structure in place we implemented the `Graph` implementation of the Clustering trait looks as follows
```rust,no_run,noplayground
{{#include ../../src/greedy/cluster.rs:graphs_mst_cluster_impl}}
```

[^note]:[Wikipedia: Single-linkage clustering](https://en.wikipedia.org/wiki/Single-linkage_clustering)