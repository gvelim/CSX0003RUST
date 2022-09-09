use crate::graphs::*;
use std::collections::{HashMap, HashSet};
use rand::{Rng, thread_rng};
use hashbag::*;



trait MinimumCut {
    fn minimum_cut(&self) -> Option<Graph>;
    fn contract_graph(&self) -> Option<Graph>;
    fn get_crossing_edges(&self, src_set:&HashSet<Node>, dst_set:&HashSet<Node>) -> Graph;
}

impl MinimumCut for Graph {
    // ANCHOR: graphs_min_cut
    fn minimum_cut(&self) -> Option<Graph> {

        // calculate the number of iterations as N*log(N)
        let nodes = self.nodes.len();
        let mut iterations = nodes as u32 * nodes.ilog2();

        // initialise min-cut min value and output as Option
        let mut min_cut = 100;
        let mut result = None;

        // iterate N*log(N) time or exit if min-cut found has only 2 edges
        while iterations != 0 && min_cut > 4 {

            // contract the graph
            if let Some(graph) = self.contract_graph() {

                // extract the number of edges
                let edges = graph.export_edges();
                print!("Edges: {:?}", edges);
                // count the edges
                let edges = edges.len();

                // if number of edges returned is smaller than current
                // then store the min-cut returned from this iteration
                if edges < min_cut {
                    min_cut = edges;
                    result = Some(graph);
                    print!(" << Min Cut !!")
                }
                println!();
            }
            iterations -= 1;
        }
        result
    }
    // ANCHOR_END: graphs_min_cut

    // ANCHOR: graphs_contraction
    fn contract_graph(&self) -> Option<Graph> {

        if self.edges.is_empty() {
            return None;
        }

        // STEP 1: INITIALISE temporary super node and super edge structures
        let mut super_edges = self.export_edges().into_iter().collect::<HashBag<Edge>>();
        let mut super_nodes = self.nodes.iter()
            .fold( HashMap::<Node,HashSet<Node>>::new(), |mut super_nodes, node| {
                super_nodes
                    .entry(*node)
                    .or_insert( HashSet::new() )
                    .insert(*node);
                super_nodes
            });

        // println!("Super Nodes: {:?}",super_nodes);
        // println!("Super Edges: {:?}",super_edges);

        // STEP 2: CONTRACT the graph, until 2 super nodes are left
        while super_nodes.len() > 2 {

            // STEP A: select a random edge
            let idx = thread_rng().gen_range(0..super_edges.len()-1);
                // get a copy rather a reference so we don't upset the borrow checker
                // while we deconstruct the edge into src and dst nodes
            let Edge(src,dst) = super_edges.iter().nth(idx).copied().unwrap();
            // println!("Random Edge: ({src},{dst})");

            // STEP B : Contract the edge by merging the edge's nodes
                // remove both nodes that form the random edge and
                // hold onto the incoming/outgoing edges
            let super_src = super_nodes.remove(&src).unwrap();
            let super_dst = super_nodes.remove(&dst).unwrap();
                // combine the incoming/outgoing edges for attaching onto the new super-node
            let super_node = super_src.union(&super_dst).copied().collect::<HashSet<Node>>();
            // println!("Merged super node: {src}->{:?}", super_node);

                // re-insert the src node as the new super-node and attach the resulting union
            super_nodes.entry(src).or_insert(super_node);


            // STEP C : Collapse/Remove newly formed edge loops since src & dst is the new super node
                // Hint: repeat until all edge loops have been removed
            while super_edges.remove(&Edge(src,dst)) != 0 { };
            while super_edges.remove(&Edge(dst,src)) != 0 { };

            // STEP D : Identify all edges affected due to the collapsing of nodes
            let bad_edges = super_edges.iter()
                // remove the reference
                .copied()
                // filter out those not affected
                .filter(|e| if e.0 == dst || e.1 == dst { true } else { false } )
                // collect any remaining
                .collect::<HashSet<Edge>>();

            // STEP E : Repoint affected edges to the new super node
                // We have to remove, fix and reinsert 1..* all edges incl. any **duplicate** ones
            for mut e in bad_edges {
                // we have only bad edges here hence this code does not have to deal with good edges
                // count how many duplicates we are about to remove
                let mut edges = super_edges.contains(&e);
                // then remove the bad edge incl. any duplicates
                while super_edges.remove(&e) != 0 { }

                // fix the edge
                if e.0 == dst { e.0 = src }
                if e.1 == dst { e.1 = src }

                // insert back the fixed edge incl. any duplicates
                while edges != 0 {
                    super_edges.insert(e);
                    edges -= 1;
                }
            }

            // println!("Round done\n=======");
            // println!("Super Nodes: {:?}",super_nodes);
            // println!("Super Edges: {:?}",super_edges);
        }
        // println!("Graph: {:?}",self);

        // STEP 3 : find the edges between the two super node sets
        let (_, dst_set) = super_nodes.iter().last().unwrap();
        let (_, src_set) = super_nodes.iter().next().unwrap();

        Some( self.get_crossing_edges(src_set, dst_set) )
    }
    // ANCHOR_END: graphs_contraction

    // ANCHOR: graphs_crossing
    fn get_crossing_edges(&self, src_set: &HashSet<Node>, dst_set: &HashSet<Node>) -> Graph {
        let output = src_set.into_iter()
            .fold(Graph::new(), |mut out, src| {
                // get src_node's edges from the original graph
                let set = self.edges.get(src).unwrap();

                // Keep only the edges nodes found in the dst_set (intersection)
                // we need to clone the reference before we push them
                // into the output graph structure
                let edges = set.intersection(dst_set).copied().collect::<HashSet<Node>>();
                // println!("Node: {node} -> {:?}",edges);
                
                // add only edges connecting src & dst super node sets
                if !edges.is_empty() {
                    // add edges: direction dst -> src
                    edges.iter()
                        .for_each(|dst| {
                            out.nodes.insert(*dst);
                            out.edges.entry(*dst)
                                .or_insert(HashSet::new())
                                .insert(*src);
                        });
                    // add edges: direction src -> dst
                    out.nodes.insert(*src);
                    out.edges.insert(*src, edges);
                }
                out
            });
        // println!("Crossing graph: {:?}", output);
        output
    }
    // ANCHOR_END: graphs_crossing
}

#[cfg(test)]
mod test {
    use crate::graphs::Graph;
    use super::*;

    #[test]
    fn test_min_cut() {

        // test dataset: Array[ (input_graph, output_graph) ]
        let adj_list: Vec<(Vec<Vec<Node>>, Vec<Vec<Node>>)> = vec![
            (
                vec![
                    vec![1, 2, 4, 3],
                    vec![2, 3, 1, 4, 5],
                    vec![3, 4, 2, 8, 1],
                    vec![4, 1, 3, 2],
                    vec![5, 6, 8, 7, 2],
                    vec![6, 7, 5, 8],
                    vec![7, 8, 6, 5],
                    vec![8, 5, 3, 7, 6]
                ],
                vec![
                    vec![3, 8],
                    vec![2, 5],
                    vec![8, 3],
                    vec![5, 2]
                ]
            ),
            (
                vec![
                    vec![1, 2, 3, 4, 7],
                    vec![2, 1, 3, 4],
                    vec![3, 1, 2, 4],
                    vec![4, 1, 2, 3, 5],
                    vec![5, 4, 6, 7, 8],
                    vec![6, 5, 7, 8],
                    vec![7, 1, 5, 6, 8],
                    vec![8, 5, 6, 7]
                ],
                vec![
                    vec![1, 7],
                    vec![4, 5],
                    vec![7, 1],
                    vec![5, 4]
                ]
            ),
            (
                vec![
                    vec![1, 2, 4],
                    vec![2, 3, 1, 4],
                    vec![3, 4, 2],
                    vec![4, 1, 3, 2]
                ],
                vec![
                    vec![1, 2, 4],
                    vec![2, 1],
                    vec![4, 1]
                ]
            )
        ];

        for (input, output) in &adj_list {
            let g = Graph::import_edges( input ).expect("Error: Couldn't load input edges");
            let o = Graph::import_edges(output ).expect("Error: Couldn't load output edges");
            assert_eq!( g.minimum_cut(), Some(o) );
        }
    }
}