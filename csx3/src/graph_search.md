# Graph Search Algorithms
The term graph search or graph traversal refers to a class of algorithms based on systematically
visiting the vertices of a graph that can be used to compute various properties of graphs

To motivate graph search, let’s first consider the kinds of properties of a graph that we
might be interested in computing. We might want to determine 
* if one vertex is reachable from another. Recall that a vertex `u` is reachable from `v` if there is a (directed) path from `v` to `u`
* if an undirected graph is connected, 
* if a directed graph is strongly connected—i.e. there is a path from every vertex to every other vertex
* the shortest path from a vertex to another vertex. 

These properties all involve paths, so it makes sense to think about algorithms that follow paths. This is effectively the goal of graph-search