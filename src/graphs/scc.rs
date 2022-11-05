use std::collections::{BinaryHeap};
use super::{*, NodeState::{ Discovered, Processed, Undiscovered }};

// ANCHOR: graphs_abstract_dfs
/// Depth First Search abstraction, enabling a variety of implementations such as, strongly connected components, topological sort, etc
/// The `Path_Search()` default implementation uses the below functions while it leaves their behaviour to the trait implementer
/// - Node pre-processing step fn()
/// - Node post-processing step fn()
/// - Node pre-processing edge fn()
/// - abort recursion fn()
/// - Path return fn()
/// - node state fn()
trait DFSearch {
    type Output;

    /// work to be done before edges are explored, that is, discovered but not processed
    /// uses incl. measuring entry time, set node state, etc
    fn pre_process_node(&mut self, node: Node) -> &mut Self;

    /// work to be done after the edges have been explored; hence the node is now processed
    /// uses incl. measuring exit time, set node parent, save node in path, etc
    fn post_process_node(&mut self, node: Node) -> &mut Self;

    /// work to be done after the node pre-processed and before the edges is explored
    /// uses incl. check for loops, categorize edge into types, etc
    /// default implementation does nothing otherwise you have to override
    fn pre_process_edge(&mut self, _edge: Edge) -> &mut Self { self }

    /// Abort the recursion
    /// uses incl. detecting the graph is not Direct acyclic, etc
    fn abort(&self) -> bool { false }

    /// return the path at position and given the pre/post processing steps
    fn path(&self) -> &Self::Output;

    /// return whether the node has been seen before
    fn is_discovered(&self, node: Node) -> bool;

    /// Default implementation of depth first search
    fn path_search(&mut self, g: &Graph, start: Node) -> Option<&Self::Output> {
        // Entering the node at time tick()
        if self.pre_process_node(start).abort() { return None }

        // processing the edges
        // println!("Enter: {start}:{:?}", self.tracker[start]);
        if let Some(edges) = g.edges.get(&start) {
            for &dst in edges {
                let d = dst.into();

                if self.pre_process_edge(Edge(start,d)).abort() { return None };

                if !self.is_discovered(d) {
                    self.path_search(g, d);
                }
            }
        }
        // Exiting the node at time tick()
        if self.post_process_node(start).abort() { return None };
        // println!("Exit: {start}:{:?}", self.tracker[start]);
        Some(self.path())
    }
}
// ANCHOR_END: graphs_abstract_dfs
// ANCHOR: graphs_scc_state
/// GraphState struct enable us to maintain the processing state of the graph
/// and while we apply a recursive approach in searching the graph
struct GraphState {
    tracker: Tracker,
    queue: BinaryHeap<Step<Node>>,
    time: Cost,
    path: Vec<Node>
}

impl GraphState {
    /// Construct a new `GraphState` given a `Graph`
    fn new(g: &Graph) -> GraphState {
        GraphState {
            tracker: g.get_tracker(Undiscovered, 0, None),
            queue: BinaryHeap::new(),
            time: 0,
            path: Vec::new()
        }
    }

    /// Extract from `BinaryHeap` the exit times per ordered from max -> min
    fn get_timings(&self) -> Vec<(Node, Cost)> {
        self.queue.iter().rev().map(|&s| (s.0, s.1) ).collect::<Vec<_>>()
    }
}

/// Graph State implements DFSearch trait and particularly provides specific implementation for
/// the calculation of the strongly connected components, in terms of node post/pre processing fn(),
/// path return fn() and node state fn()
impl DFSearch for GraphState {
    type Output = Vec<Node>;

    /// capture time of entry and set node state to visited,
    /// given the node's edges have yet be visited
    fn pre_process_node(&mut self, node: Node) -> &mut Self {
        // Entering the node at time tick()
        self.time += 1;
        self.tracker[node].visited(Discovered).distance(self.time);
        self
    }

    /// capture time of exit and set node state to processed,
    /// given all edges have also been processed
    fn post_process_node(&mut self, node: Node) -> &mut Self {
        // Exiting the node at time tick()
        self.time += 1;
        self.tracker[node].visited(Processed).distance(self.time);
        self.queue.push(Step(node, self.time));
        self.path.push(node);
        self
    }

    /// Return the path as it was calculated by the post processing step
    fn path(&self) -> &Self::Output {
        &self.path
    }

    /// return the state of the node
    fn is_discovered(&self, node: Node) -> bool {
        self.tracker[node].is_discovered()
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
            .fold(Vec::new(),|mut components, (node, _)| {
                if !gs.is_discovered(node) {
                    // extract new component
                    let component = gs.path_search(&tg, node ).unwrap();
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

// ANCHOR: graphs_topological_sort_state
/// Graph state that we need to maintain
/// for the topological sort algorithm
struct TState {
    tracker: Tracker,
    path: Vec<Node>,
    abort: bool
}

impl TState {
    /// Construct a new `GraphState` given a `Graph`
    fn new(g: &Graph) -> TState {
        TState {
            tracker: g.get_tracker(Undiscovered, 0, None),
            path: Vec::new(),
            abort: false
        }
    }
}

/// Topological sort implementation of the TState
/// There is no need for exit/entry time or tracking parent node.
/// Here we only need to save the `node` in the `tracker.path` following its full processing
impl DFSearch for TState {
    type Output = Vec<Node>;

    /// mark node as visited but not processed
    fn pre_process_node(&mut self, node: Node) -> &mut Self {
        self.tracker[node].visited(Discovered);
        self
    }

    /// Important we store the node in the path following node processing complete
    fn post_process_node(&mut self, node: Node) -> &mut Self {
        self.tracker[node].visited(Processed);
        self.path.push(node);
        self
    }

    /// before we jump into the edge for further exploration
    /// we check if the edge is actually a node already `Discovered` but not `Processed`
    /// if that is the case, we set the abort flag to `True`
    fn pre_process_edge(&mut self, edge: Edge) -> &mut Self {
        let Edge(_,dst) = edge;
        if self.tracker[dst].visited == Discovered {
            self.abort = true;
        }
        self
    }

    /// Implement the abort fn() so we can stop the path search recursion
    fn abort(&self) -> bool {
        self.abort
    }

    /// extract the aggregate path stored
    fn path(&self) -> &Self::Output {
        &self.path
    }

    /// return true if node is either `Discovered` or `Processed`
    fn is_discovered(&self, node: Node) -> bool {
        self.tracker[node].is_discovered()
    }
}
// ANCHOR_END: graphs_topological_sort_state
// ANCHOR: graphs_topological_sort
/// Topological Sort trait
trait TopologicalSort {
    fn topological_sort(&self) -> Option<Vec<Node>>;
}
/// Graph implementation of Topological Sort
impl TopologicalSort for Graph {
    /// Implementation of topological sort for Graph
    fn topological_sort(&self) -> Option<Vec<Node>> {
        // initiate the run state structure for calculating the topological sort of the graph
        let mut ts = TState::new(self);

        // Construct a path aggregate, that is, each path found is joined up together
        // achieved by appending the path of each iteration onto tracker.path
        // see post_processing() of TState implementation of DFSearch
        for &node in &self.nodes {
            // if node is not yet visited && search hasn't thrown a NONE, that is, we've found a circle
            if !ts.is_discovered(node)
                && ts.path_search(self, node).is_none() {
                return None
            }
        }

        // Extract & reverse path from tracker so we extract the topological sort
        ts.path.reverse();
        Some(ts.path)
    }
}
// ANCHOR_END: graphs_topological_sort

#[cfg(test)]
mod test {
    use std::cmp::Reverse;
    use super::*;

    #[test]
    fn test_scc_small() {
        let test_data = vec![
            ("src/graphs/txt/scc_simple.txt", vec![3,2,1,1,0]),
            ("src/graphs/txt/scc_da_small.txt", vec![5, 4, 2, 1, 1]),
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
                scc.sort_by_key(|a| Reverse(a.len()));

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
    }
    #[test]
    fn test_topological_sort() {
        let test_data = vec![
            ("src/graphs/txt/ts_simple.txt", vec![Some(vec![4, 5, 1, 2, 3, 6]),Some(vec![4, 1, 5, 2, 3, 6])])
            ,("src/graphs/txt/ts_da_small.txt", vec![Some(vec![1,2,3,4]),Some(vec![1,3,2,4])])
            ,("src/graphs/txt/scc_simple.txt", vec![None])
        ];

        test_data.into_iter()
            .for_each(|(filename, out)| {
                println!("> {filename}");
                let g = Graph::import_text_graph(filename, ' ', '\0').unwrap_or_else(|| panic!("Cannot open file: {}", filename));
                let ts = g.topological_sort();
                println!("Found: {:?}, Expected {:?}",ts,out);
                assert!( out.contains(&ts) );
                println!("--------------------");
            });
    }
}