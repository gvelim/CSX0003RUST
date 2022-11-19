use crate::graphs::{Edge, Graph, NodeType::{N, NC}};
use std::collections::{BinaryHeap, HashSet};

/// Trait defining the capability calculate the minimum spanning tree of a graph
/// given an input algorithm function()
pub trait MinimumSpanningTree {
    type Output;
    type Algo;
    fn min_spanning_tree(&self, _:Self::Algo) -> Self::Output;
}

/// Implementation of the Minimum Spanning Tree by the Graph struct
impl MinimumSpanningTree for Graph {
    type Output = Option<Graph>;
    type Algo = fn(&Graph)->Self::Output;
    /// Implements function with algorithm parameterization
    fn min_spanning_tree(&self, algo:Self::Algo) -> Self::Output {
        algo(self)
    }
}

/// Implement Helper Graph functions for minimum spanning tree algorithm
impl Graph {
    // ANCHOR: graphs_mst_graph_prim
    /// MST using Prim's algorithm implementation
    pub fn mst_prim(&self) -> Option<Graph> {

        // Create an empty Graph/Tree to add one edge at a time
        // we'll be using g.node as the Tree's Component invariant,
        // that is, the Component that contains all vertices absorbed by the Tree
        let mut tree = Graph::new();

        // Min-Ordered heap with all edges found crossing the evolving tree
        let mut heap = BinaryHeap::<Edge>::new();

        // seed with first vertex
        let &start = self.nodes.iter().next().unwrap();
        heap.push(Edge(start, NC(start, 0)));

        // spawn a node at a time until we have spawned all graph nodes
        // while tree component isn't equal input component
        while tree.nodes != self.nodes {
            // spawn a new edge node from the queue with the smallest edge weight
            let src = match heap.pop() {
                // if the queue is empty, but still have nodes to spawn
                // then either (a) the graph is not connected or (b) is a directed graph
                None => return None,
                // spawn the destination node from edge
                Some(Edge(_, NC(dst, _))) => dst,
                Some(Edge(_, N(_))) => panic!("mst_prim(): Extracted edge using wrong NodeType::N"),
            };

            // Add all edges that are crossing the tree Component given the spawned node
            // and have not yet been spawned, that is, they are NOT already part of tree component
            heap.extend(self.edges.get(&src)
                .unwrap_or_else(|| panic!("mst_prim(): Node ({src}) has not edges; Graph is not undirected or connected"))
                .iter()
                // remove any edge node already in the mst, part of Component X
                .filter(|&&dst| !tree.nodes.contains(&dst.into()))
                // push edges crossing Component X, that is,
                // src IN Component X, dst NOT IN Component X
                .map(|&dst| Edge(src, dst))
            );

            // find the min-weigh edge that is crossing the current tree component
            // don't remove from heap as we need to spawn dst node for the next iteration
            while let Some(&edge) = heap.peek() {
                let Edge(src, dst) = edge;
                // Is this edge a stale or a valid one, that is, crosses the tree component
                if HashSet::from([src, dst.into()]).is_subset(&tree.nodes) {
                    // Some times heap holds older edges that, after few iterations they get stale,
                    // that is, both edges nodes have been moved into the tree component
                    heap.pop();
                } else {
                    // either src or dst edge nodes are outside the tree component
                    // hence add the edge into the tree
                    tree.push_edge(edge);
                    // exit the while loop since we've found the edge with the min weight
                    break
                }
            }
        }
        Some(tree)
    }
    // ANCHOR_END: graphs_mst_graph_prim
    // ANCHOR: graphs_mst_graph_kruska
    /// MST using Kruskal's algorithm implementation
    pub fn mst_kruska(&self) -> Option<Graph> {

        // Get the ordered heap by lowest cost Edge on top
        let mut heap = self.get_edges_by_cost();
        // Keeps the graph's components, that is, a super node is a graph component's lead node
        // The initial state is for each node to be a lead component node with a component of its own
        let mut snodes = self.get_super_nodes();
        // the output graph that will hold *only* the edges
        // that form the minimum spanning tree
        let mut graph = Graph::new();

        // As long as more than 2 components
        while snodes.len() > 1 {
            // get the edge with the lowest cost
            // otherwise if we've run out of edges while there are 2 or more components
            // then the graph IS NOT CONNECTED
            let Some(edge) = heap.pop() else { return None };
            let Edge(src, NC(dst, _)) = edge else { panic!("mst_kruska() - Cannot find NodeType::NC") };
            // print!("({src:2}->{dst:2}):{cost:6} - ");

            // if src is not a super node then get its super node
            let src = snodes.find_supernode(&src);
            // if dst is not a super node then get its super node
            let dst = snodes.find_supernode(&dst);

            // if src component differs from dst component then merge the two and save the edge connecting them
            if src != dst {
                snodes.merge_nodes(src, dst);
                graph.push_edge(edge);
                // println!("Store");
            } else {
                // println!("Skip");
            }
        }
        Some(graph)
    }
    // ANCHOR_END: graphs_mst_graph_kruska
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mst_kruska() {
        let test_data = vec![
            ("src/greedy/mst_cpb.txt", 20)
            ,("src/greedy/input_random_1_10.txt", -7430)
            ,("src/greedy/input_random_2_10.txt", -12829)
            ,("src/greedy/input_random_6_20.txt", -15557)
            ,("src/greedy/input_random_20_100.txt", -183953)
        ];
        for (filename, result) in test_data {
            let mut g = Graph::new();
            println!("{filename}");
            let mst = g.load_file_mst(filename).min_spanning_tree(Graph::mst_kruska);
            assert!(mst.is_some());
            let graph = mst.unwrap();
            let cost = graph.sum_edges();
            println!("Min Spanning Tree: ({cost}) {:?}",graph);
            assert_eq!(result, cost);
        }
    }
    #[test]
    fn test_mst_prim() {
        let test_data = vec![
            ("src/greedy/txt/mst_cpb.txt", 20)
            ,("src/greedy/txt/input_random_1_10.txt", -7430)
            ,("src/greedy/txt/input_random_2_10.txt", -12829)
            ,("src/greedy/txt/input_random_6_20.txt", -15557)
            ,("src/greedy/txt/input_random_20_100.txt", -183953)
        ];
        for (filename, result) in test_data {
            let mut g = Graph::new();
            println!("{filename}");
            let mst = g.load_file_mst(filename).min_spanning_tree(Graph::mst_prim);
            assert!(mst.is_some());
            let graph = mst.unwrap();
            let cost = graph.sum_edges();
            println!("Min Spanning Tree: ({cost}) {:?}",graph);
            assert_eq!(result, cost);
        }
    }
}