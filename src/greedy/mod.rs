use std::cmp::{Ordering};
use std::collections::{BinaryHeap};
use crate::graphs::{Edge, Graph, Cost, NodeType::{N, NC}};

// ANCHOR: graphs_mst
/// Trait defining the capability calculate the minimum spanning tree of a graph
/// given an input algorithm function()
trait MinimumSpanningTree {
    type Output = Option<Graph>;
    type Algo = fn(&Graph)->Self::Output;
    fn min_spanning_tree(&self, _:Self::Algo) -> Self::Output;
}

/// Implementation of the Minimum Spanning Tree by the Graph struct
impl MinimumSpanningTree for Graph {
    /// Implements function with algorithm parameterization
    fn min_spanning_tree(&self, algo:Self::Algo) -> Self::Output {
        algo(self)
    }
}

/// Implement Helper Graph functions for minimum spanning tree algorithm
impl Graph {
    // ANCHOR: graphs_mst_graph_prim
    pub fn mst_prim(&self) -> Option<Graph> {

        // Great empty graph to add one edge at a time
        // we'll be using g.node as the X Component invariant,
        // that is, all vertices spawned in the mst
        let mut g = Graph::new();

        // Min-Ordered heap with all edges found crossing the X Component
        let mut heap = BinaryHeap::<Edge>::new();

        // seed with first vertex
        let &start = self.nodes.iter().next().unwrap();
        heap.push( Edge(start,NC(start,0)));

        // spawn a node at a time until we have spawned all graph nodes
        // While X != V
        while g.nodes != self.nodes {
            // spawn a new edge node from the queue with the smallest edge weight
            let src = match heap.pop() {
                // if the queue is empty, but still have nodes to spawn
                // then either (a) the graph is not connected or (b) is a directed graph
                None => return None,
                // spawn the destination node from edge
                Some(Edge(_, NC(dst,_))) => dst,
                Some(Edge(_, N(_))) => panic!("mst_prim(): Extracted edge using wrong NodeType::N"),
            };

            // Find all edge nodes that crossing Component X from this node
            // and have not yet been spawned, that is, they are NOT already part of Component X
            self.edges.get(&src)
                .unwrap_or_else(|| panic!("mst_prim(): Node ({src}) has not edges; Graph is not undirected or connected"))
                .iter()
                // remove any edge node already in the mst, part of Component X
                .filter(|&&dst| !g.nodes.contains(&dst.into()))
                // push edges crossing Component X, that is,
                // src IN Component X, dst NOT IN Component X
                .for_each(|&dst| {
                    heap.push(Edge(src,dst));
                });

            // find the small edge crossing current component X
            // don't remove from heap as we need the dst node for the next iteration
            while let Some(&edge) = heap.peek() {
                let Edge(src,dst) = edge;
                // Is edge a valid one, that is, crosses the Component X
                if g.nodes.contains(&src) && g.nodes.contains(&dst.into()) {
                    // Some times heap holds older edges that, after been added, had both nodes moved into Component X
                    heap.pop();
                } else {
                    // either src or dst edge node are outside the component X
                    // hence add the edge into the mst
                    g.push_edge(edge);
                    // exit the while loop since we found the edge with the min weight
                    break
                }
            }
        }
        Some(g)
    }
    // ANCHOR_END: graphs_mst_graph_prim

    // ANCHOR: graphs_mst_graph
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
            let Edge(src,NC(dst,cost)) = edge else { panic!("mst_kruska() - Cannot find NodeType::NC") };
            print!("({src:2}->{dst:2}):{cost:6} - ");

            // if src is not a super node then get its super node
            let src = snodes.find_supernode(&src);
            // if dst is not a super node then get its super node
            let dst = snodes.find_supernode(&dst);

            // if src component differs from dst component then merge the two and save the edge connecting them
            if src != dst {
                snodes.merge_nodes(src, dst);
                graph.push_edge(edge);
                println!("Store");
            } else {
                println!("Skip");
            }
        }
        Some(graph)
    }
    // ANCHOR_END: graphs_mst

    /// Sums up the cost of all weighted edges
    pub fn get_edges_cost(&self) -> Cost {
        self.edges
            .values()
            .fold(0, |cost, edges| {
                cost + edges.iter()
                    .map(|&dst| {
                        let NC(_,c) = dst else { panic!("get_mst_cost(): Destination node is not of type NodeType::NC") };
                        c
                    })
                    .reduce(|acc,c| acc + c )
                    .unwrap()
            }) >> 1 // in an undirected graph we count twice the edge hence dividing by 2
    }
    /// Adds a new Edge to the graph
    pub fn push_edge(&mut self, edge: Edge) {
        let Edge(src, dst) = edge;
        self.nodes.insert(src);
        self.edges.entry(src)
            .or_default()
            .insert(dst);
        let NC(dst,cost) = dst else { panic!("") };
        self.nodes.insert(dst);
        self.edges.entry(dst)
            .or_default()
            .insert(NC(src,cost));
    }
    /// Returns Graph's edges in the form of a MinHeap, that is,
    /// the lowest cost edge at the top of the heap
    pub fn get_edges_by_cost(&self) -> BinaryHeap<Edge> {
        self.edges.iter()
            .fold(BinaryHeap::new(), |mut heap, (&src, edges)| {
                    heap.extend(
                        edges.iter().map(|&dst| Edge(src,dst))
                    );
                    heap
                })

    }
    // ANCHOR_END: graphs_mst_graph
    /// Private function that contracts a `Graph` struct given an MST test file
    fn load_file_mst(&mut self, filename: &str) -> &mut Graph {
        use std::fs::File;
        use std::io::{BufReader, BufRead};
        use std::str::FromStr;

        let hnd = File::open(filename).unwrap_or_else(|e| panic!("load_test_file(): Cannot open file `{filename}` = {e}"));
        let buf = BufReader::new(hnd);

        buf.lines()
            .skip(1)
            .into_iter()
            .for_each(| line| {
                let str = line.unwrap_or_else(|e| panic!("load_file_mst(): {e}"));

                let mut iter = str.split(' ');
                let src = usize::from_str(iter.next().unwrap()).unwrap_or_else(|e| panic!("load_file_mst(): {e}"));
                let dst = usize::from_str(iter.next().unwrap()).unwrap_or_else(|e| panic!("load_file_mst(): {e}"));
                let cost = i32::from_str(iter.next().unwrap()).unwrap_or_else(|e| panic!("load_file_mst(): {e}"));

                self.push_edge( Edge(src,NC(dst,cost)));
            });
        self
    }
}

// ANCHOR: graphs_mst_step
/// BinaryHeap Step structure containing (Edge, Cost) tuple
/// The `cost` is only used as the prioritisation key for the `Heap`
/// Implementing MinHeap through reverse comparison of Other against Self
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.1 {
            N(_) => other.partial_cmp(self),
            NC(_, cost) => {
                let Edge(_,NC(_,sc)) = self else { panic!("") };
                cost.partial_cmp(sc)
            }
        }
    }
}
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
// ANCHOR_END: graphs_mst_step

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_load_edge_by_cost() {
        let filename = "src/greedy/input_random_1_10.txt";
        let mut g = Graph::new();
        println!("{:?}", g.load_file_mst(filename).get_edges_by_cost() );
        println!("{:?}", g.get_edges_cost());
        assert!(true)
    }
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
            let cost = graph.get_edges_cost();
            println!("Min Spanning Tree: ({cost}) {:?}",graph);
            assert_eq!(result, cost);
        }
    }
    #[test]
    fn test_mst_prim() {
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
            let mst = g.load_file_mst(filename).min_spanning_tree(Graph::mst_prim);
            assert!(mst.is_some());
            let graph = mst.unwrap();
            let cost = graph.get_edges_cost();
            println!("Min Spanning Tree: ({cost}) {:?}",graph);
            assert_eq!(result, cost);
        }
    }
}