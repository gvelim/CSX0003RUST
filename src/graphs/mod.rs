mod min_cut;

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Error, Formatter};

type Node = usize;

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

struct Graph {
    edges: HashMap<Node, HashSet<Node>>,
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
        self.edges.iter()
            .fold( HashSet::<Edge>::new(), |mut edges, (src_node, dst_nodes)| {
                dst_nodes.iter()
                    .for_each(|dst_node| {
                        edges.insert(Edge(*src_node,*dst_node));
                    });
                edges
            })
    }
    fn import_edges( list: &[Vec<Node>] ) -> Result<Self, Error> {
        let mut graph = Graph::new();

        list.into_iter().
            map(|edges| {
                (&edges[0],&edges[1..])
            })
            .for_each(|(src, dst)| {
                graph.nodes.insert(*src);
                dst.into_iter()
                    .for_each(|dst| {
                        let nodes = graph.edges.entry(*src)
                            .or_insert(HashSet::new());
                        nodes.insert(*dst);
                    })
            });

        Ok(graph)
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