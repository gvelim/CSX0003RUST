# Graphs
Collection of common graph algorithms for calculating 
* path search
* minimum-cuts
* strongly connected components

For more details visit here:
https://gvelim.github.io/CSX0003RUST/graph.html

### Examples
Minimum Cut - Karger Algorithm / Randomised
```rust,no_run,noplayground
use crate::graphs::*;

let adj_list: Vec<(Vec<Vec<Node>>, Vec<Vec<Node>>)> = vec![
            (
                vec![
                    vec![1, 2, 4, 3],
                    vec![2, 3, 1, 4, 5],
                    vec![3, 4, 2, 8, 1],
                    vec![4, 1, 3, 2],
                    vec![5, 6, 8, 7, 2],
                    vec![6, 7, 5, 8],
                    vec![7, 8, 6, 5],
                    vec![8, 5, 3, 7, 6]
                ],
                vec![
                    vec![3, 8],
                    vec![2, 5],
                    vec![8, 3],
                    vec![5, 2]
                ]
            ),
        ];
        

for (input, output) in &adj_list {
    let g = Graph::import_edges( input ).expect("Error: Couldn't load input edges");// Graph: {
                                                                                    //      8: {3, 5, 6, 7},
                                                                                    //      5: {2, 8, 6, 7},
                                                                                    //      4: {1, 3, 2},
                                                                                    //      1: {2, 4, 3},
                                                                                    //      2: {1, 4, 5, 3},
                                                                                    //      6: {5, 7, 8},
                                                                                    //      7: {6, 5, 8},
                                                                                    //      3: {2, 1, 8, 4}
                                                                                    // }
    let o = Graph::import_edges(output).expect("Error: Couldn't load output edges");// Graph: {
                                                                                    //      8: {3},
                                                                                    //      5: {2},
                                                                                    //      2: {5},
                                                                                    //      3: {8}
                                                                                    // }
    assert_eq!( g.minimum_cut(), Some(o) ); // Iterate through N graph contractions and pick the min-cut
                                            // 1: Edges: {E(8, 3), E(1, 3), E(3, 8), E(3, 1), E(3, 4), E(4, 3), E(2, 3), E(3, 2)} << Min Cut !!
                                            // 2: Edges: {E(8, 7), E(7, 5), E(6, 5), E(8, 6), E(6, 8), E(5, 6), E(7, 8), E(5, 7)}
                                            // 3: Edges: {E(1, 3), E(4, 1), E(2, 1), E(1, 2), E(3, 1), E(1, 4)} << Min Cut !!
                                            // 4: Edges: {E(2, 1), E(3, 2), E(3, 4), E(4, 1), E(2, 5), E(2, 3), E(1, 4), E(4, 3), E(1, 2), E(5, 2)}
                                            // 5: Edges: {E(4, 2), E(4, 3), E(3, 4), E(1, 2), E(1, 3), E(3, 1), E(2, 1)}
                                            // 6: Edges: {E(6, 5), E(2, 5), E(8, 5), E(7, 5), E(5, 7), E(5, 2), E(5, 6), E(5, 8)}
                                            // 7: Edges: {E(5, 6), E(5, 7), E(7, 8), E(6, 5), E(7, 5), E(6, 8), E(8, 7), E(8, 6)}
                                            // 8: Edges: {E(2, 5), E(8, 3), E(5, 2), E(3, 8)} << Min Cut !!
}

```
