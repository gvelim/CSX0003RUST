# Graphs
Collection of common graph algorithms

For more details visit here:
https://gvelim.github.io/CSX0003RUST/graph.html

### Examples
Minimum Cut - Karger Algorithm / Randomised
```rust,no_run,noplayground
use crate::graphs::*;

let adj_list: [Vec<Node>;8] = [
    vec![1, 2, 3, 4, 7],
    vec![2, 1, 3, 4],
    vec![3, 1, 2, 4],
    vec![4, 1, 2, 3, 5],
    vec![5, 4, 6, 7, 8],
    vec![6, 5, 7, 8],
    vec![7, 1, 5, 6, 8],
    vec![8, 5, 6, 7]
];

let g = import_edges( &adj_list )
            .expect("Error: Couldn't load edges");  // Graph: {
                                                    //      8: {3, 5, 6, 7},
                                                    //      5: {2, 8, 6, 7},
                                                    //      4: {1, 3, 2},
                                                    //      1: {2, 4, 3},
                                                    //      2: {1, 4, 5, 3},
                                                    //      6: {5, 7, 8},
                                                    //      7: {6, 5, 8},
                                                    //      3: {2, 1, 8, 4}
                                                    // }
let mut output = HashSet::<Edge>::new();
output.insert( Edge(3, 8));
output.insert( Edge(2, 5));
                                                    // Iterations
assert_eq!( g.minimum_cuts(), Some(output) );       // Edges: {E(2, 4), E(8, 3), E(2, 1), E(2, 3)} << Min Cut !!
                                                    // Edges: {E(8, 6), E(7, 6), E(5, 6)} << Min Cut !!
                                                    // Edges: {E(1, 2), E(1, 3), E(1, 4)}
                                                    // Edges: {E(3, 1), E(2, 1), E(4, 1)}
                                                    // Edges: {E(1, 3), E(1, 4), E(1, 2)}
                                                    // Edges: {E(7, 6), E(7, 8), E(7, 5)}
                                                    // Edges: {E(2, 1), E(8, 3), E(2, 4), E(2, 3)}
                                                    // Edges: {E(2, 5), E(3, 8)} << Min Cut !!
```
