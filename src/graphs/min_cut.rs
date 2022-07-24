use super::*;
use std::collections::{HashMap, HashSet};
use rand::{Rng, thread_rng};
use hashbag::*;


trait MinimumCut {
    fn min_cuts(&self) -> Option<HashSet<Edge>>;
    fn edges_across_nodes(&self, src_set:&HashSet<Node>, dst_set:&HashSet<Node>) -> Graph;
}

impl MinimumCut for Graph {
    fn min_cuts(&self) -> Option<HashSet<Edge>> {

        if self.edges.is_empty() {
            return None;
        }

        // define super node and super edge structures
        let mut super_edges = self.export_edges().into_iter().collect::<HashBag<Edge>>();
        let mut super_nodes = self.nodes.iter()
            .fold( HashMap::<Node,HashSet<Node>>::new(), |mut super_nodes, node| {
                super_nodes
                    .entry(*node)
                    .or_insert( HashSet::new() )
                    .insert(*node);
                super_nodes
            });

        println!("Super Nodes: {:?}",super_nodes);
        println!("Super Edges: {:?}",super_edges);

        // run the min-cut algorithm, until 2 super nodes are left
        while super_nodes.len() > 2 {
            // select a random edge
            let len = super_edges.len();
            let idx = thread_rng().gen_range(0..len-1);
            // get a copy rather a reference so we don't upset the borrow checker
            let Edge(src,dst) = super_edges.iter().nth(idx).copied().unwrap();
            println!("Random Edge: ({src},{dst})");

            // remove both nodes that form the random edge and
            // hold onto the incoming/outgoing edges
            let super_src = super_nodes.remove(&src).unwrap();
            let super_dst = super_nodes.remove(&dst).unwrap();
            // combine the incoming/outgoing edges for attaching onto the new super-node
            let super_node = super_src.union(&super_dst).copied().collect::<HashSet<Node>>();
            println!("Merged super node: {src}->{:?}", super_node);
            // re-insert the src node as the new super-node and attach the resulting union
            super_nodes.entry(src).or_insert(super_node);

            // collapse / remove the obvious edge loops
            while super_edges.remove(&Edge(src,dst)) != 0 { };
            while super_edges.remove(&Edge(dst,src)) != 0 { };

            // find all bad/invalid edges; the ones affected
            let bad_edges = super_edges.iter()
                // remove the reference
                .copied()
                // filter out those not affected
                .filter(|e| if e.0 == dst || e.1 == dst { true } else { false } )
                // collect any remaining
                .collect::<HashSet<Edge>>();

            // now just remove, fix and reinsert edges
            for mut e in bad_edges {
                // we have only bad edges here hence this code does not have to deal with good edges
                // hence go and remove the bad edge
                print!("Remove:");
                let mut count = 0;
                while super_edges.remove(&e) != 0 { count += 1; print!("{:?}:{count},",e); }
                // fix the edge
                if e.0 == dst { e.0 = src }
                if e.1 == dst { e.1 = src }
                // insert back the fixed edge
                print!(" >> Insert:");
                while count != 0 { print!("{:?}:{count},",e); super_edges.insert(e); count -= 1; }
                println!("");
            }

            println!("Round done\n=======");
            println!("Super Nodes: {:?}",super_nodes);
            println!("Super Edges: {:?}",super_edges);
        }
        println!("Graph: {:?}",self);

        // find the edges between the two super node sets
        let (_, dst_set) = super_nodes.iter().last().unwrap();
        let (_, src_set) = super_nodes.iter().next().unwrap();

        Some( self.edges_across_nodes(src_set, dst_set).export_edges() )
    }

    fn edges_across_nodes(&self, src_set: &HashSet<Node>, dst_set: &HashSet<Node>) -> Graph {
        src_set.into_iter()
            .fold(Graph::new(), | mut out,node| {
                // get src_node's edges from the original graph
                let set = self.edges.get(node).unwrap();
                // Keep only the edges nodes found in the dst_set (intersection)
                // we need to clone the reference before we push them
                // into the output graph structure
                let edges = set.intersection(dst_set).copied().collect::<HashSet<Node>>();
                println!("Node: {node} -> {:?}",edges);
                // add only edges connecting src & dst super node sets
                if !edges.is_empty() {
                    out.nodes.insert(*node);
                    out.edges.insert(*node,edges);
                }
                out
            })
    }
}

#[cfg(test)]
mod test {
    use crate::graphs::Graph;
    use super::*;

    #[test]
    fn test_min_cut() {
/*
            expected result: 2
            cuts are [(1,7), (4,5)]
*/
        let adj_list: [Vec<Node>;8] = [
            vec![1, 2, 3, 4, 7],
            vec![2, 1, 3, 4],
            vec![3, 1, 2, 4],
            vec![4, 1, 2, 3, 5],
            vec![5, 4, 6, 7, 8],
            vec![6, 5, 7, 8],
            vec![7, 1, 5, 6, 8],
            vec![8, 5, 6, 7]
        ];
/*
        let adj_list: [Vec<Node>;8] = [
            vec![1, 2, 4, 3],
            vec![2, 3, 1, 4, 5],
            vec![3, 4, 2, 8, 1],
            vec![4, 1, 3, 2],
            vec![5, 6, 8, 7, 2],
            vec![6, 7, 5, 8],
            vec![7, 8, 6, 5],
            vec![8, 5, 3, 7, 6]
        ];

        let adj_list: [Vec<Node>;4] = [
            vec![1, 2, 4],
            vec![2, 3, 1, 4],
            vec![3, 4, 2],
            vec![4, 1, 3, 2]
        ];
*/
        let g = Graph::import_edges( &adj_list ).expect("Error: Couldn't load edges");
        println!("{:?}",g.min_cuts());
    }
}