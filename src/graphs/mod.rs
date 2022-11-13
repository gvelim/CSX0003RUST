mod min_cut;
mod path_search;
mod scc;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Error, Formatter};
use std::ops::{Index, IndexMut};
use crate::graphs::NodeType::NC;

pub type Node = usize;
pub type Cost = i32;

#[derive(Debug,Clone,Copy,Hash, PartialEq, Eq)]
pub enum NodeType {
    N(Node),
    NC(Node, Cost)
}
impl From<NodeType> for Node {
    fn from(nt: NodeType) -> Self {
        match nt { NodeType::N(node)|NC(node, _) => node }
    }
}

impl From<Node> for NodeType {
    fn from(value: Node) -> Self {
        NodeType::N(value)
    }
}

#[derive(Clone,Copy,Hash,Eq, PartialEq)]
pub struct Edge(pub Node, pub NodeType);

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("E")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl Edge {
    // fn has_node(&self, n:Node) -> bool {
    //     self.0 == n || self.1 == n
    // }
    // fn from(&self, e: &Self) -> bool {
    //     self.0 == e.1 // Edge(4,5).from( Edge(3,4) )
    // }
    // fn to(&self, e: &Self) -> bool {
    //     self.1 == e.0 // Edge(4,5).to( Edge(5,6) )
    // }
    // fn is_adjacent(&self, other:&Self) -> bool {
    //     self.from(other) || self.to(other)
    // }
    // fn is_loop(&self) -> bool {
    //     self.0 == self.1
    // }
    // fn reverse(&mut self) { swap( &mut self.1, &mut self.0); }
    // fn replace_node(&mut self, from:Node, to:Node) {
    //     if self.0 == from { self.0 = to } else if self.1 == from { self.1 = to }
    // }
}

// ANCHOR: graphs_search_path_utils_Step
#[derive(Debug,Copy, Clone)]
pub struct Step<T>(pub T, pub Cost);

impl Eq for Step<Node> {}
impl PartialEq<Self> for Step<Node> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl PartialOrd<Self> for Step<Node> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Step<Node> {
    fn cmp(&self, other: &Self) -> Ordering {
        // binary head is a max-heap implementation pushing to the top the biggest element self.cmp(other)
        // hence we need to reverse the comparison other.cmp(self)
        other.1.cmp(&self.1) //then_with(|| other.0.cmp(&self.0))
    }
}
// ANCHOR_END: graphs_search_path_utils_Step

// ANCHOR: graphs_search_path_utils_NodeTrack
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum NodeState {
    Undiscovered,
    Discovered,
    Processed
}
#[derive(Debug,Clone)]
pub struct NodeTrack {
    visited:NodeState,
    dist:Cost,
    parent:Option<Node>
}
impl NodeTrack {
    pub fn visited(&mut self, s:NodeState) -> &mut Self {
        self.visited = s; self
    }
    pub fn distance(&mut self, d:Cost) -> &mut Self {
        self.dist = d; self
    }
    pub fn parent(&mut self, n:Node) -> &mut Self {
        self.parent =Some(n); self
    }
    pub fn is_discovered(&self) -> bool {
        self.visited != NodeState::Undiscovered
    }
}
#[derive(Debug)]
pub struct Tracker {
    list: HashMap<Node, NodeTrack>
}
pub trait Tracking {
    fn extract(&self, start:Node) -> (Vec<Node>, Cost) {
        (self.extract_path(start), self.extract_cost(start))
    }
    fn extract_path(&self, start: Node) -> Vec<Node>;
    fn extract_cost(&self, start: Node) -> Cost;
}
impl Tracking for Tracker {
    fn extract_path(&self, start:Node) -> Vec<Node> {
        let mut path = VecDeque::new();
        // reconstruct the shortest path starting from the target node
        path.push_front(start);
        // set target as current node
        let mut cur_node= start;
        // backtrace all parents until you reach None, that is, the start node
        while let Some(parent) = self[cur_node].parent {
            path.push_front(parent);
            cur_node = parent;
        }
        path.into()
    }
    fn extract_cost(&self, start:Node) -> Cost {
        self[start].dist
    }
}
impl Index<Node> for Tracker {
    type Output = NodeTrack;

    fn index(&self, index: Node) -> &Self::Output {
        self.list.get(&index).unwrap_or_else(|| panic!("Error: cannot find {index} in tracker {:?}", &self))
    }
}
impl IndexMut<Node> for Tracker {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        self.list.get_mut(&index).unwrap_or_else(|| panic!("Error: cannot find {index} in tracker"))
    }
}
// ANCHOR_END: graphs_search_path_utils_NodeTrack
// ANCHOR: graphs_search_path_utils_NodeTrack_graph
impl Graph {

    pub fn get_tracker(&self, visited: NodeState, dist: Cost, parent: Option<Node>) -> Tracker {
        Tracker {
            list: self.nodes.iter()
                .fold(HashMap::new(), |mut out, &node| {
                    out.entry(node)
                        .or_insert(NodeTrack { visited, dist, parent });
                    out
                })
        }
    }
}
// ANCHOR_END: graphs_search_path_utils_NodeTrack_graph

#[derive(PartialEq, Default)]
pub struct Graph {
    pub edges: HashMap<Node, HashSet<NodeType>>,
    pub nodes: HashSet<Node>
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }
    pub fn export_edges(&self) -> HashSet<Edge> {
        use NodeType::*;

        self.edges.iter()
            .fold( HashSet::<Edge>::new(),|mut edges, (&src, dst_nodes)| {
                dst_nodes.iter()
                    .for_each(|&dst_node| {
                        edges.insert(Edge(src, dst_node));
                        match dst_node {
                            N(dst) => edges.insert(Edge(dst, N(src))),
                            NC(dst,cost) => edges.insert(Edge(dst, NC(src, cost)))
                        };
                    });
                edges
            })
    }
    pub fn import_edges( list: &[Vec<Node>] ) -> Result<Self, Error> {
        let mut graph = Graph::new();

        list.iter().
            map(|edges| {
                (&edges[0],&edges[1..])
            })
            .for_each(|(src, dst)| {
                graph.nodes.insert(*src);
                dst.iter()
                    .for_each(|dst| {
                        graph.edges.entry(*src)
                            .or_default()
                            .insert((*dst).into());
                    })
            });
        Ok(graph)
    }
    pub fn from_edge_list(edge_list: &[(Node, Node, Cost)]) -> Self {
        let mut adjacency_list: HashMap<Node, HashSet<NodeType>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(HashSet::new);

            destinations.insert(NC(destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
    pub fn import_text_graph(file: &str, node_pat: char, edge_pat: char) -> Option<Graph> {
        use std::{fs::File, path::Path, io::{BufRead, BufReader}, str::FromStr};

        let mut g = Graph::new();
        let f = File::open(Path::new(file)).unwrap_or_else(|e| panic!("Could not open {file}: {e}"));
        let buf = BufReader::new(f);

        buf.lines().into_iter()
            .enumerate()
            .map(|(num,line)| (num, line.unwrap_or_else(|e| panic!("Cannot read line:{num} from file: {e}") )))
            .for_each(|(num,line)| {
                let mut part = line.split(node_pat);
                let node = Node::from_str(part.next().unwrap()).unwrap_or_else(|e| panic!("Line {num}: Cannot extract Node from line {e}"));
                g.nodes.insert(node);

                for txt in part {
                    let edge = match edge_pat {
                        '\0' => NodeType::N( Node::from_str(txt).unwrap_or_else(|e| panic!("Line {num}: Cannot convert {txt} to Edge {e}")) ),
                        ',' =>
                            if let Some((e_str, c_str)) = txt.split_once(edge_pat) {
                                NC(
                                    Node::from_str(e_str).unwrap_or_else(|e| panic!("Line {num}: Cannot convert {e_str} to Edge {e}")),
                                    Cost::from_str(c_str).unwrap_or_else(|e| panic!("Line {num}: Cannot convert {c_str} to Cost {e}"))
                                )
                            } else {
                                panic!("Cannot convert {txt} into (edge, cost): line {num} ends with a tab ??")
                            },
                        pat => panic!("Unknown delimiter:({}) within txt:({txt}",pat)
                    };
                    g.edges.entry(node)
                        .or_default()
                        .insert(edge);
                }
                // println!("{} -> {:?}",node, g.edges[&node])
            });
        Some(g)
    }
}


impl Clone for Graph {
    fn clone(&self) -> Self {
        Graph {
            edges: self.edges.clone(),
            nodes: self.nodes.clone()
        }
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.edges.iter())
            .finish()
    }
}

#[cfg(test)]
mod test {
    //use crate::graphs::Graph;

}