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
    fn strongly_connected_components(&self) -> Option<HashMap<Node, HashSet<Node>>>;
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
    fn strongly_connected_components(&self) -> Option<HashMap<Node, HashSet<Node>>> {

        struct DFS { tracker: Tracker, queue: BinaryHeap<Step>, count: Cost }
        impl DFS {
            fn new(g: &Graph) -> DFS {
                DFS {
                    tracker: g.get_tracker(Undiscovered, 0, None),
                    queue: BinaryHeap::new(),
                    count: 0
                }
            }
            fn dfs(&mut self, g: &Graph, start: Node) {
                // Entering the node
                self.count += 1;
                self.tracker[start].visited = Discovered;
                self.tracker[start].dist = self.count;

                // processing the edges
                println!("Enter: {start}:{:?}", self.tracker[start]);
                if let Some(edges) = g.edges.get(&start) {
                    for &dst in edges {
                        let d = dst.into();
                        if self.tracker[d].visited == Undiscovered {
                            self.tracker[d].parent = Some(start);
                            self.dfs(g, d);
                        }
                    }
                }
                // Exiting the node
                self.count += 1;
                self.tracker[start].visited = Processed;
                self.tracker[start].dist = self.count;
                self.queue.push(Step(start, self.count));
                println!("Exit: {start}:{:?}", self.tracker[start]);
            }
        }

        // initiate heap that prioritises by processed order
        let mut state = DFS::new(self);

        self.nodes.iter()
            .for_each(|&start| {
                println!("Start >> {start}");
                if state.tracker[start].visited == Undiscovered {
                    state.dfs(self, start)
                }
            });

        let v = state.queue.iter().rev().copied().collect::<Vec<_>>();
        let rev_g = &self.reverse();
        let mut state = DFS::new(self);

        println!("{:?}",v);
        v.into_iter()
            .for_each(|Step(node, _)| {
                if state.tracker[node].visited == Undiscovered {
                    state.dfs(rev_g,node);
                }
            });


        Some(HashMap::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scc_small() {
        let test_data = vec![
            ("src/graphs/txt/scc_simple.txt", &[3,2,1,1,0]),
            ("src/graphs/txt/scc_input_mostlyCycles_1_8.txt", &[4,2,2,0,0])
            ,("src/graphs/txt/scc_input_mostlyCycles_8_16.txt", &[13,1,1,1,0])
            ,("src/graphs/txt/scc_input_mostlyCycles_12_32.txt", &[29,3,0,0,0])
        ];

        test_data.into_iter()
            .for_each(|(fname, cuts)| {
                let g = Graph::import_text_graph(fname, ' ', '\0').unwrap_or_else(|| panic!("Cannot open file: {}",fname));
                let mc = g.strongly_connected_components();
                assert!(mc.is_some());
                println!(">> Scc: {:?}",mc);
                let scc =
                    mc.unwrap()
                        .into_iter()
                        .fold(vec![], |mut out, (_,set)| {
                            out.push(set.len());
                            out
                        });
                assert_eq!( scc, cuts );
                println!("--------------------");
            });
        assert!(true);
    }
}