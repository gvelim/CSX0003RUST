use std::collections::{BinaryHeap};
use super::*;
use crate::graphs::{
    path_search::{
        Step,
        Tracker,
        NodeState::{ Discovered, Processed, Undiscovered }
    }
};

struct DFS {
    tracker: Tracker,
    queue: BinaryHeap<Step>,
    time: Cost,
    path: Vec<Node>
}

impl DFS {
    #[inline]
    fn tick(&mut self) -> Cost {
        self.time += 1;
        self.time
    }
    fn new(g: &Graph) -> DFS {
        DFS {
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
        self.path.push(start);

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
        // println!("Exit: {start}:{:?}", self.tracker[start]);
        &self.path
    }
}

trait ConnectedComponents {
    fn strongly_connected(&self) -> Vec<Vec<Node>>;
}

impl ConnectedComponents for Graph {
    fn strongly_connected(&self) -> Vec<Vec<Node>> {

        // initiate the run state structure for calculating the scc of the graph
        // and in order to enable recursive searching in rust
        let mut scc = DFS::new(self);

        // 1st Pass : Find all paths and calculate entry and exit times per node
        self.nodes.iter()
            .for_each(|&start| {
                // println!("Start >> {start}");
                if !scc.tracker[start].is_discovered() {
                    let path = scc.path_search(self, start);
                    println!("Pass 1: Path {:?}",path);
                    scc.path.clear();
                }
            });

        // Extract node sequence ordered by highest exit times
        let v = scc.queue.iter().rev().copied().collect::<Vec<_>>();
        println!("Timings: {:?}",v);
        // reverse the graph edges
        let rev_g = self.reverse();
        // reset run state
        scc = DFS::new( &rev_g );

        let mut components = Vec::new();
        // Pass 2: Identify and store each strongly connected component identified
        v.into_iter()
            .for_each(|Step(node, _)| {
                if !scc.tracker[node].is_discovered() {
                    // reset path so to remove last found component
                    scc.path.clear();
                    // extract new component
                    let component = scc.path_search(&rev_g, node );
                    println!("Pass 2: Component [{}]{:?}", component.len(), component);
                    // store component found
                    components.push(component.clone() );
                }
            });
        components
    }
}

impl Graph {
    fn reverse(&self) -> Graph {
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

                let mut mc  = g.strongly_connected();
                mc.sort_by(|a,b| b.len().cmp(&a.len()));

                let mut scc = mc
                        .into_iter()
                        .take(5)
                        .enumerate()
                        .fold(vec![0;5], |mut out, (idx, set)| {
                            out[idx] = set.len();
                            out
                        });
                println!("Found: {:?}, Expected {:?}",scc,cuts);
                assert_eq!( scc, cuts );
                println!("--------------------");
            });
        assert!(true);
    }
}