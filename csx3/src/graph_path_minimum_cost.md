# Minimum Path Cost Algorithm
Here we cover problems involving finding the shortest path between vertices in a
graph with weights (lengths) on the edges. One obvious application is in finding the shortest
route from one address to another, however shortest paths have many other application[^note] 

## Dijkstra's Algorithm
Dijkstra’s is an important algorithm both because it is an efficient algorithm for an important problem, but
also because it is a very elegant example of an efficient greedy algorithm that generates optimal
solutions on a nontrivial task.

The below animated image demonstrated how the algorithm works

![image](img/Dijkstra_Animation.gif "Dijkstra_Animation")

The above depiction performs the below steps 
* push the starting node `a` in the priority queue with cost `0`
* Pop the node with the lowest cost in the queue; at fist this is `a`
  * if the 'node' matches our target node `b` 
    * extract path with minimum cost 
    * terminate
  * For each `edge node` attached to the `node`
    * calculate `cost distance`
    * if `edge node` has `cost` larger to the calculated `cost distance` then assign cost to `edge node`, otherwise do not update cost
    * push `(edge node, cost)` to the priority queue and repeat

### Processing path cost across the graph structure
As we traverse the graph, the `Tracker` structure holds the processed graph information in relation to nodes visited along with the smallest `cost` and associated `parent` node.
This helper structure, enables us to 
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

### Implementation
Dijkstra's differentiating approach is that we must always process next the node with the lowest cost in the queue. To achieve this we have to make use of the `BinaryHeap` collection structure. The use of such structure help us to maintain on ordered-queue by node-cost, hence keeping the node with lowest-cost at the top of the heap/queue.
```rust,no_run,noplayground
{{#include ../../src/graphs/path_search.rs:graphs_search_path_utils_Step}}
```
With the ordered-queue logic in place, the algorithm can now be realised through the following code
```rust,no_run,noplayground
{{#include ../../src/graphs/path_search.rs:graphs_search_path_min_cost}}
```

### References:
[^note]:[Shortest Path](https://www.cs.cmu.edu/afs/cs/academic/class/15210-s15/www/lectures/shortest-path.pdf)
