use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Error, Formatter};

mod min_cut;

type Node = usize;

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