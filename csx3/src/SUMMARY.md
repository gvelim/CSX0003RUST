
# Summary

[Introduction](intro.md)

# Topics

- [Merging](./merge.md)
  - [In-place merge with O(n+m) swaps](./merge_in_place.md)
  - [Sequential access across multiple slices](./merge_sequencial_access.md)
  - [Lazy merge and deferred slice mutability](./merge_lazy.md)
  - [Pattern matching: De-normalising control flow](./merge_denormalise.md)

- [Sorting](./sort.md)
  - [Merge sort](./sort_mergesort.md)
  - [Quick sort](./sort_quicksort.md)
  - [Count sort](./sort_count.md)

- [Selecting](./selection.md)

- [Graphs](./graph.md)
  - [Krager's Minimum Cut](./graph_min_cut.md)
    - [Contraction Algorithm](./graph_contraction.md)
  - [Graph Search](./graph_search.md)
    - [Node Processing State](graph_search_process_state.md)
    - [Abstracting Breadth First Search](graph_path_bfs_abstract.md)
      - [Shortest Distance](./graph_path_shortest_distance.md)
      - [Dijktra's Min Path Cost](./graph_path_minimum_cost.md)
      - [Bellman–Ford algorithm]()
    - [Abstracting Depth First Search](graph_path_dfs_abstract.md)
      - [Topological Sort](./graph_path_topological_sort.md)
      - [Strong Connectivity](./graph_connect.md)
        - [Kosaraju’s algorithm](./graph_connect_scc.md)
  - [Minimum Spanning Trees](./graph_mst.md)
    - [Kruskal's Algorithm](./graph_mst_kruskal.md)
    - [Prim's Algorithm](./graph_mst_prim.md)
    - [Single-linkage clustering](./graph_mst_cluster.md)
