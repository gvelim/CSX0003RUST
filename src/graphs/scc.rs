use std::collections::{BinaryHeap};
use super::*;
use crate::graphs::{
    path_search::{
        Step,
        Tracker,
        NodeState::{ Discovered, Processed, Undiscovered }
    }
};

trait ConnectedComponents {
    fn strongly_connected_components(&self) -> Option<Vec<Vec<Node>>>;
}

impl Graph {
    fn reverse(&self) -> Graph {
        self.edges.iter()
            .fold(Graph::new(), |mut g, (&node, edges)| {
                edges.iter()
                    .for_each(|&e|{
                        g.nodes.insert(e.into());
                        g.edges.entry(e.into()).or_default().insert(node.into());
                    });
                g
            })
    }
}
impl ConnectedComponents for Graph {
    fn strongly_connected_components(&self) -> Option<Vec<Vec<Node>>> {

        struct DFS { tracker: Tracker, queue: BinaryHeap<Step>, time: Cost, path: Vec<Node> }
        impl DFS {
            fn new(g: &Graph) -> DFS {
                DFS {
                    tracker: g.get_tracker(Undiscovered, 0, None),
                    queue: BinaryHeap::new(),
                    time: 0,
                    path: Vec::new()
                }
            }
            fn dfs(&mut self, g: &Graph, start: Node) {
                // Entering the node
                self.time += 1;
                self.tracker[start].visited(Discovered).distance(self.time);

                // processing the edges
                // println!("Enter: {start}:{:?}", self.tracker[start]);
                if let Some(edges) = g.edges.get(&start) {
                    for &dst in edges {
                        let d = dst.into();
                        if !self.tracker[d].is_discovered() {
                            self.tracker[d].parent(start);
                            self.dfs(g, d);
                        }
                    }
                }
                // Exiting the node
                self.time += 1;
                self.tracker[start].visited(Processed).distance(self.time);
                self.queue.push(Step(start, self.time));
                self.path.push(start);
                // println!("Exit: {start}:{:?}", self.tracker[start]);
            }
        }

        // initiate heap that prioritises by processed order
        let mut scc = DFS::new(self);

        self.nodes.iter()
            .for_each(|&start| {
                println!("Start >> {start}");
                if !scc.tracker[start].is_discovered() {
                    scc.dfs(self, start)
                }
            });

        let v = scc.queue.iter().rev().copied().collect::<Vec<_>>();
        let rev_g = self.reverse();
        scc = DFS::new( &rev_g );

        println!("{:?}",v);
        let mut out = vec![vec![]];
        v.into_iter()
            .for_each(|Step(node, _)| {
                if !scc.tracker[node].is_discovered() {
                    scc.path.clear();
                    scc.dfs( &rev_g,node );
                    println!("path: {:?}",scc.path);
                    out.push(scc.path.clone() );
                }
            });

        Some(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scc_small() {
        let test_data = vec![
            ("src/graphs/txt/scc_simple.txt", vec![3,2,1,1,0]),
            ("src/graphs/txt/scc_input_mostlyCycles_1_8.txt", vec![4,2,2,0,0]),
            ("src/graphs/txt/scc_input_mostlyCycles_8_16.txt", vec![13,1,1,1,0]),
            ("src/graphs/txt/scc_input_mostlyCycles_12_32.txt", vec![29,3,0,0,0])
        ];

        test_data.into_iter()
            .for_each(|(fname, cuts)| {
                let g = Graph::import_text_graph(fname, ' ', '\0').unwrap_or_else(|| panic!("Cannot open file: {}",fname));
                let mc = g.strongly_connected_components();
                assert!(mc.is_some());
                println!(">> Scc: {:?}",mc);
                let mut scc =
                    mc.unwrap()
                        .into_iter()
                        .take(5)
                        .enumerate()
                        .fold(vec![0;5], |mut out, (idx, set)| {
                            out[idx] = set.len();
                            out
                        });
                scc.sort_by(|a, b| b.cmp(a));
                assert_eq!( scc, cuts );
                println!("--------------------");
            });
        assert!(true);
    }
}