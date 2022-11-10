use std::cmp::{Ordering};
use std::collections::BinaryHeap;
use crate::graphs::{Edge, Graph, NodeType, Cost, Step, Node};

// ANCHOR: graphs_mst
/// Trait defining the capability calculate the minimum spanning tree of a graph
trait MinimumSpanningTree {
    fn min_spanning_tree(&self) -> Option<Graph>;
}

/// Implementation of the Minimum Spanning Tree by the Graph struct
impl MinimumSpanningTree for Graph {
    /// MST using Kruskal's algorithm implementation
    fn min_spanning_tree(&self) -> Option<Graph> {

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
            let Some(Step(edge,cost)) = heap.pop() else { return None };
            let Edge(src,dst) = edge;
            print!("({src:2}->{:2?}):{cost:6} - ",<NodeType as Into<Node>>::into(dst));

            // if src is not a super node then get its super node
            let src = snodes.find_supernode(&src);
            // if dst is not a super node then get its super node
            let dst = snodes.find_supernode(&dst.into());

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
}
// ANCHOR_END: graphs_mst
// ANCHOR: graphs_mst_step
/// BinaryHeap Step structure containing (Edge, Cost) tuple
/// The `cost` is only used as the prioritisation key for the `Heap`
/// Implementing MinHeap through reverse comparison of Other against Self
impl Eq for Step<Edge> {}
impl PartialEq<Self> for Step<Edge> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
}
impl PartialOrd<Self> for Step<Edge> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       Some(other.1.cmp(&self.1))
    }
}
impl Ord for Step<Edge> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}
// ANCHOR_END: graphs_mst_step
/// Implement Helper Graph functions for minimum spanning tree algorithm
impl Graph {
    // ANCHOR: graphs_mst_graph
    /// Sums up the cost of all weighted edges
    pub fn get_edges_cost(&self) -> Cost {
        self.edges
            .values()
            .fold(0, |mut cost, edges| {
                for dst in edges {
                    let NodeType::NC(_,c) = dst else { panic!("get_mst_cost(): NodeType is not of the NC(node, cost) format") };
                    cost += c;
                }
                cost
            })
    }
    /// Adds a new Edge to the graph
    pub fn push_edge(&mut self, edge: Edge) {
        let Edge(src, dst) = edge;
        self.nodes.insert(src);
        self.nodes.insert(dst.into());
        self.edges.entry(src)
            .or_default()
            .insert(dst);
    }
    /// Returns Graph's edges in the form of a MinHeap, that is,
    /// the lowest cost edge at the top of the heap
    pub fn get_edges_by_cost(&self) -> BinaryHeap<Step<Edge>> {
        self.edges.iter()
            .fold(BinaryHeap::new(),|mut heap, (&src, edges)| {
                for &dst in edges {
                    let NodeType::NC(_,cost) = dst else { panic!("get_edges_by_cost(): Ops!") };
                    heap.push(Step(Edge(src,dst),cost));
                }
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

                self.nodes.insert(src);
                self.nodes.insert(dst);
                self.edges.entry(src).or_default().insert(NodeType::NC(dst, cost));
            });
        self
    }
}

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
    fn test_mst() {
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
            let mst = g.load_file_mst(filename).min_spanning_tree();
            assert!(mst.is_some());
            let graph = mst.unwrap();
            let cost = graph.get_edges_cost();
            println!("Min Spanning Tree: ({cost}) {:?}",graph);
            assert_eq!(result, cost);
        }
    }
}