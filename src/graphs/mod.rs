

mod min_cut;
mod path_search;
mod scc;

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Error, Formatter};
use crate::graphs::NodeType::NC;

type Node = usize;
type Cost = usize;

#[derive(Debug,Clone,Copy,Hash, PartialEq, Eq)]
enum NodeType {
    N(Node),
    NC(Node, Cost)
}
impl From<NodeType> for Node {
    fn from(nt: NodeType) -> Self {
        match nt { NodeType::N(node)|NC(node, _) => node }
    }
}
impl Into<NodeType> for Node {
    fn into(self) -> NodeType {
        NodeType::N(self)
    }
}

#[derive(Clone,Copy,Hash,Eq, PartialEq)]
struct Edge(Node, Node);

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


#[derive(PartialEq)]
struct Graph {
    edges: HashMap<Node, HashSet<NodeType>>,
    nodes: HashSet<Node>
}

impl Graph {
    fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
            nodes: HashSet::new()
        }
    }
    fn export_edges(&self) -> HashSet<Edge> {
        use NodeType::*;

        self.edges.iter()
            .fold( HashSet::<Edge>::new(),|mut edges, (src_node, dst_nodes)| {
                dst_nodes.iter()
                    .for_each(|&dst_node| {
                        match dst_node {
                            N(node) | NC(node, _) => {
                                edges.insert(Edge(*src_node, node));
                                edges.insert(Edge(node, *src_node));
                            }
                        }
                    });
                edges
            })
    }
    fn import_edges( list: &[Vec<Node>] ) -> Result<Self, Error> {
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
    fn from_edge_list(edge_list: &[(Node, Node, Cost)]) -> Self {
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
    fn import_text_graph(file: &str, node_pat: char, edge_pat: char) -> Option<Graph> {
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
            .entries( self.edges.iter() )
            .finish()
    }
}

#[cfg(test)]
mod test {
    //use crate::graphs::Graph;

}