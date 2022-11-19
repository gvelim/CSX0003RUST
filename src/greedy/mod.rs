mod mst;
mod cluster;

use std::{ cmp::Ordering, collections::BinaryHeap };
use crate::graphs::{ Edge, Graph, Cost, NodeType::{N, NC} };

impl Graph {
    // ANCHOR: graphs_mst_graph
    /// Sums up the cost of all weighted edges
    pub fn sum_edges(&self) -> Cost {
        self.edges
            .values()
            .fold(0, |cost, edges| {
                cost + edges.iter()
                    .map(|&dst| {
                        let NC(_,c) = dst else { panic!("get_mst_cost(): Edge destination node is not of type NodeType::NC") };
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
/// BinaryHeap Step structure containing `Edge(src,(dst,cost))` tuple
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
        println!("{:?}", g.sum_edges());
        assert!(true)
    }
}