

mod min_cut;
mod bfs;

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Error, Formatter};
use std::fs::File;
use std::path::Path;
use crate::graphs::NodeType::NC;

type Node = usize;
type Cost = usize;

#[derive(Debug,Clone,Copy,Hash, PartialEq, Eq)]
enum NodeType {
    N(Node),
    NC(Node, Cost)
}

#[derive(Clone,Copy,Hash)]
struct Edge(Node, Node);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl Eq for Edge {}
impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("E")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
/*
impl Edge {
    fn starts_at(&self, e: &Self) -> bool {
        self.0 == e.1
    }
    fn ends_at(&self, e: &Self) -> bool {
        self.1 == e.0
    }
    fn is_adjacent(&self, other:&Self) -> bool {
        (self.0 == other.0 || self.0 == other.1) || (self.1 == other.0 || self.1 == other.1)
    }
    fn is_loop(&self) -> bool {
        self.0 == self.1
    }
    fn start(&mut self, e: &Self) { self.0 = e.1; }
    fn end(&mut self, e: &Self) { self.1 = e.0; }
    fn reverse(&self) -> Edge { Edge(self.1, self.0) }
}
 */

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
            .fold( HashSet::<Edge>::new(), |mut edges, (src_node, dst_nodes)| {
                dst_nodes.iter()
                    .for_each(|dst_node| {
                        match dst_node {
                            &N(node) |
                            &NC(node, _) => {
                                edges.insert(Edge(*src_node, node));
                                edges.insert(Edge(node, *src_node));
                            }
                        }
                    });
                edges
            })
    }
    fn import_edges( list: &[Vec<Node>] ) -> Result<Self, Error> {
        use NodeType::*;

        let mut graph = Graph::new();

        list.into_iter().
            map(|edges| {
                (&edges[0],&edges[1..])
            })
            .for_each(|(src, dst)| {
                graph.nodes.insert(*src);
                dst.into_iter()
                    .for_each(|dst| {
                        graph.edges.entry(*src)
                            .or_insert(HashSet::new())
                            .insert(N(*dst));
                    })
            });

        Ok(graph)
    }
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, HashSet<NodeType>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(|| HashSet::new());

            destinations.insert(NC(destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
    fn import_text_graph(file: &str) -> Option<Graph> {
        use std::{io::{BufRead, BufReader}, str::FromStr};

        let mut g = Graph::new();
        let f = File::open(Path::new(file)).expect("Could not open file");
        let buf = BufReader::new(f);

        let mut reader = buf.lines().into_iter();
        while let Some(Ok(line)) = reader.next() {

            let mut part = line.split('\t').into_iter();
            let node = Node::from_str(part.next().unwrap()).expect("Cannot convert text to Node");
            g.nodes.insert(node);

            while let Some(txt) = part.next() {

                if let Some((e_str, c_str)) = txt.split_once(',') {
                    let edge = Node::from_str(e_str).expect("Cannot convert text to Node");
                    let cost = Cost::from_str(c_str).expect("Cannot convert text to Cost");
                    g.edges.entry(node)
                        .or_insert(HashSet::new())
                        .insert(NC( edge, cost ));
                } else {
                    panic!("Cannot convert txt into (node, cost)")
                }
            }
            // println!("{} -> {:?}",node, g.edges[&node])
        }
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