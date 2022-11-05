use std::cmp::{Ordering};
use std::collections::BinaryHeap;
use crate::graphs::{Edge, Graph, NodeType, Cost, Step};

trait MinimumSpanningTree {
    fn min_spanning_tree(&self) -> Graph;
    fn get_mst_cost(&self) -> Cost;
}

impl MinimumSpanningTree for Graph {
    fn min_spanning_tree(&self) -> Graph {
        let heap = self.get_edges_by_cost();
        let graph = Graph::new();

        graph
    }

    fn get_mst_cost(&self) -> Cost {
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
}

/// Implement Heap Step Structure for a MinHeap behaviour
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

/// Implement Graph functions for getting Edges ordered by lowest cost first
impl Graph {
    fn get_edges_by_cost(&self) -> BinaryHeap<Step<Edge>> {
        self.edges.iter()
            .fold(BinaryHeap::new(),|mut heap, (&src, edges)| {
                for d in edges {
                    let &NodeType::NC(dst, cost) = d else { panic!("get_edges_by_cost(): NodeType is not of the NC(node, cost) format") };
                    heap.push(Step(Edge(src,dst),cost));
                }
                heap
            })

    }
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

                let mut iter = str.split(' ').into_iter();
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
        println!("{:?}", g.get_mst_cost());
        assert!(true)
    }
    #[test]
    fn test_mst() {
        let test_data = vec![
            ("src/greedy/input_random_1_10.txt", -7430)
            ,("src/greedy/input_random_6_20.txt", -15557)
        ];
        for (filename, result) in test_data {
            let mut g = Graph::new();
            assert_eq!(
                result,
                g.load_file_mst(filename)
                    .min_spanning_tree()
                    .get_mst_cost()
            );
        }
    }
}