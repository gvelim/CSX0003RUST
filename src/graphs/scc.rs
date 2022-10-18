use std::collections::{BinaryHeap};
use super::*;
use crate::graphs::{
    path_search::{
        Step,
        Tracker,
        NodeState::{ Discovered, Processed, Undiscovered }
    }
};

// ANCHOR: graphs_scc_state
struct GraphState {
    tracker: Tracker,
    queue: BinaryHeap<Step>,
    time: Cost,
    path: Vec<Node>
}

impl GraphState {
    #[inline]
    fn tick(&mut self) -> Cost {
        self.time += 1;
        self.time
    }
    fn get_timings(&self) -> Vec<Step> {
        self.queue.iter().rev().copied().collect::<Vec<_>>()
    }
    fn new(g: &Graph) -> GraphState {
        GraphState {
            tracker: g.get_tracker(Undiscovered, 0, None),
            queue: BinaryHeap::new(),
            time: 0,
            path: Vec::new()
        }
    }
    fn path_search(&mut self, g: &Graph, start: Node) -> &Vec<Node>{
        // Entering the node at time tick()
        self.tick();
        self.tracker[start].visited(Discovered).distance(self.time);

        // processing the edges
        // println!("Enter: {start}:{:?}", self.tracker[start]);
        if let Some(edges) = g.edges.get(&start) {
            for &dst in edges {
                let d = dst.into();
                if !self.tracker[d].is_discovered() {
                    self.path_search(g, d);
                }
            }
        }
        // Exiting the node at time tick()
        self.tick();
        self.tracker[start].visited(Processed).distance(self.time);
        self.queue.push(Step(start, self.time));
        self.path.push(start);
        // println!("Exit: {start}:{:?}", self.tracker[start]);
        &self.path
    }
}
// ANCHOR_END: graphs_scc_state

// ANCHOR: graphs_scc
trait ConnectedComponents {
    fn strongly_connected(&self) -> Vec<Vec<Node>>;
}

impl ConnectedComponents for Graph {
    fn strongly_connected(&self) -> Vec<Vec<Node>> {

        // initiate the run state structure for calculating the scc of the graph
        // and in order to enable recursive searching in rust
        let mut gs = GraphState::new(self);

        // Pass 1: Find all paths and calculate entry and exit times per node
        self.nodes.iter()
            .for_each(|&start| {
                // println!("Start >> {start}");
                if !gs.tracker[start].is_discovered() {
                    let path = gs.path_search(self, start);
                    println!("Pass 1: Path {:?}",path);
                    gs.path.clear();
                }
            });

        // Extract node sequence ordered by highest exit times
        let v = gs.get_timings();
        println!("Timings: {:?}",v);
        // reverse the graph edges
        let tg = self.transpose();
        // reset run state
        gs = GraphState::new( &tg);

        // Pass 2: Identify and store each strongly connected component identified
        v.into_iter()
            .fold(Vec::new(),|mut components, Step(node, _)| {
                if !gs.tracker[node].is_discovered() {
                    // extract new component
                    let component = gs.path_search(&tg, node );
                    println!("Pass 2: Component [{}]{:?}", component.len(), component);
                    // store component found
                    components.push(component.clone() );
                    // reset path so to remove last found component
                    gs.path.clear();
                }
                components
            })
    }
}
// ANCHOR_END: graphs_scc
// ANCHOR: graphs_scc_traversal
impl Graph {
    fn transpose(&self) -> Graph {
        self.nodes.iter()
            .fold(Graph::new(), |mut g, &node| {
                g.nodes.insert(node);
                // reverse the edges for this node, if any
                if let Some(edges) = self.edges.get(&node) {
                    edges.iter()
                        .for_each(|&e|{
                            g.nodes.insert(e.into());
                            g.edges.entry(e.into()).or_default().insert(node.into());
                        });
                }
                g
            })
    }
}
// ANCHOR_END: graphs_scc_traversal

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scc_small() {
        let test_data = vec![
            ("src/graphs/txt/scc_simple.txt", vec![3,2,1,1,0]),
            ("src/graphs/txt/scc_input_mostlyCycles_1_8.txt", vec![4,2,2,0,0]),
            ("src/graphs/txt/scc_input_mostlyCycles_8_16.txt", vec![13,1,1,1,0]),
            ("src/graphs/txt/scc_input_mostlyCycles_9_32.txt", vec![14,9,6,2,1]),
            ("src/graphs/txt/scc_input_mostlyCycles_12_32.txt", vec![29,3,0,0,0]),
            ("src/graphs/txt/scc_input_mostlyCycles_30_800.txt", vec![437,256,51,44,10]),
            ("src/graphs/txt/scc_input_mostlyCycles_50_20000.txt", vec![12634,6703,253,139,113])
        ];

        test_data.into_iter()
            .for_each(|(fname, cuts)| {
                println!("> {fname}");
                let g = Graph::import_text_graph(fname, ' ', '\0').unwrap_or_else(|| panic!("Cannot open file: {}",fname));

                let mut scc = g.strongly_connected();
                scc.sort_by(|a, b| b.len().cmp(&a.len()));

                let vec = scc
                    .into_iter()
                    .map(|v| v.len() )
                    .take(5)
                    .enumerate()
                    .fold(vec![0;5], |mut out, (idx, val)| { out[idx] = val; out });
                println!("Found: {:?}, Expected {:?}",vec,cuts);
                assert_eq!( vec, cuts );
                println!("--------------------");
            });
        assert!(true);
    }
}