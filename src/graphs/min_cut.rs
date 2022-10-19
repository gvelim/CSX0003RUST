use super::*;
use std::{collections::HashMap, ops::Div};
use rand::{Rng, thread_rng};
use hashbag::HashBag;

// ANCHOR: graphs_min_cut_super_edges
#[derive(Debug)]
struct SuperEdges {
    list: HashMap<Node, HashBag<Node>>
}
impl SuperEdges {
    fn get_random_edge(&self) -> Edge {
        let mut idx = thread_rng().gen_range(0..self.list.len());
        let (&node, edges) =
            self.list.iter().nth(idx)
                .unwrap_or_else(|| panic!("get_random_edge(): Couldn't find (Node,Edges) at position({idx})"));
        // print!("get_random_edge(): ({node},{:?}<- pos:({idx},",edges);
        idx = thread_rng().gen_range(0..edges.len());
        // println!("{idx})");
        Edge(
            node,
            self.list[&node].iter().nth(idx).cloned()
                .unwrap_or_else(|| panic!("get_random_edge(): cannot get dst node at position({idx})"))
        )
    }
    fn remove_edge(&mut self, src: Node, dst: Node) {
        // print!("remove_edge(): {:?} IN:{:?} -> Out:", edge, self);
        while self.list.get_mut(&src)
            .unwrap_or_else(|| panic!("remove_edge(): Node({src}) cannot be found within SuperEdges"))
            .remove(&dst) != 0 { };
        // println!("{:?}",self);
    }
    fn move_edges(&mut self, old: Node, new: Node) {
        // Fix direction OLD -> *
        let old_edges = self.list
            .remove(&old)
            .unwrap_or_else(|| panic!("move_edges(): cannot remove old node({old})"));
        // print!("move_edges(): {old}:{:?}, {new}:{:?}", old_edges,self.list[&new]);
        self.list.get_mut(&new)
            .unwrap_or_else(|| panic!("move_edges(): failed to extend({new}) with {:?} from({new})", old_edges))
            .extend( old_edges.into_iter());

        // Fix Direction * -> OLD
        self.list.values_mut()
            .filter_map( |e| {
                let count = e.contains(&old);
                if  count > 0  { Some((count, e)) } else { None }
            })
            .for_each(|(mut count, edges)| {
                while edges.remove(&old) != 0 {};
                while count != 0 { edges.insert(new); count -= 1; }
            });
        // println!(" -> {:?}",self.list[&new]);
    }
}
// ANCHOR_END: graphs_min_cut_super_edges
// ANCHOR: graphs_min_cut_super_edges_graph
impl Graph {
    fn get_super_edges(&self) -> SuperEdges {
        let list = self.edges.iter()
            .map(|(n,e)| (*n, e.iter().map(|&nt| nt.into()).collect())
            ).collect();
        // println!("get_super_edges(): {:?}",list);
        SuperEdges { list }
    }
    fn get_super_nodes(&self) -> HashMap<Node,HashSet<Node>> {
        self.nodes.iter()
            .map(|&node| (node, HashSet::<Node>::new()))
            .map(|(node, mut map)| { map.insert(node); (node,map) })
            .collect::<HashMap<Node,HashSet<Node>>>()
    }
}
// ANCHOR_END: graphs_min_cut_super_edges_graph

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
        println!("Run Iterations: {iterations}");

        // initialise min-cut min value and output as Option
        let mut min_cut = usize::MAX;
        let mut result = None;
        let repetitions = iterations as f32;

        // iterate N*log(N) time or exit if min-cut found has only 2 edges
        let mut f = f32::MAX;
        while iterations != 0 && f > 0.089 {

            // contract the graph
            if let Some(graph) = self.contract_graph() {

                // extract the number of edges
                let edges = graph.export_edges();
                // count the edges
                let edges = edges.len();

                // if number of edges returned is smaller than current
                // then store the min-cut returned from this iteration
                if edges < min_cut {
                    min_cut = edges;
                    result = Some(graph);
                    f = (min_cut as f32).div(repetitions);
                    println!("({iterations})({f:.3}) Min Cut !! => {:?}", edges);
                }
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
        let mut super_edges = self.get_super_edges();
        let mut super_nodes= self.get_super_nodes();

        // STEP 2: CONTRACT the graph, until 2 super nodes are left
        while super_nodes.len() > 2 {

            // STEP A: select a random edge
                // get a copy rather a reference so we don't upset the borrow checker
                // while we deconstruct the edge into src and dst nodes
            let Edge(src,dst) = super_edges.get_random_edge();
            // println!("While: E({src},{dst}):{:?}",super_edges.list);

            // STEP B : Contract the edge by merging the edge's nodes
                // remove both nodes that form the random edge and
                // hold onto the incoming/outgoing edges
            let super_src = super_nodes.remove(&src).unwrap();
            let super_dst = super_nodes.remove(&dst).unwrap();
                // combine the incoming/outgoing edges for attaching onto the new super-node
            let super_node = super_src.union(&super_dst).copied().collect::<HashSet<Node>>();

                // re-insert the src node as the new super-node and attach the resulting union
            super_nodes.entry(src).or_insert(super_node);

            // STEP C : Collapse/Remove newly formed edge loops since src & dst is the new super node
            super_edges.remove_edge( src, dst);
            super_edges.remove_edge( dst, src);

            // STEP D : Identify all edges that still point to the dst removed as part of collapsing src and dst nodes
            // STEP E : Repoint all affected edges to the new super node src
            super_edges.move_edges(dst, src);
        }

        // STEP 3 : find the edges between the two super node sets
        let mut snode_iter = super_nodes.iter();
        Some(
            self.get_crossing_edges(
                snode_iter.next().expect("There is no src super node").1,
                snode_iter.next().expect("There is no dst super node").1
            )
        )
    }
    // ANCHOR_END: graphs_contraction

    // ANCHOR: graphs_crossing
    fn get_crossing_edges(&self, src_set: &HashSet<Node>, dst_set: &HashSet<Node>) -> Graph {
         src_set.iter()
            .map(|src|
                ( src,
                  // get src_node's edges from the original graph
                  self.edges.get(src)
                      .unwrap_or_else(|| panic!("get_crossing_edges(): cannot extract edges for node({src}"))
                      .iter()
                      .map(|&ntype| ntype.into() )
                      .collect::<HashSet<Node>>()
                )
            )
            .map(|(src, set)|
                // Keep only the edges nodes found in the dst_set (intersection)
                // we need to clone the reference before we push them
                // into the output graph structure
                (src, set.intersection(dst_set).copied().collect::<HashSet<Node>>())
            )
            .filter(|(_, edges)| !edges.is_empty() )
            .fold(Graph::new(), |mut out, (&src, edges)| {
                // println!("Node: {node} -> {:?}",edges);
                // add edges: direction dst -> src
                edges.iter()
                    .for_each(|&dst| {
                        out.nodes.insert(dst);
                        out.edges.entry(dst)
                            .or_default()
                            .insert(src.into() );
                    });
                // add edges: direction src -> dst
                out.nodes.insert(src);
                out.edges.insert(
                    src,
                    edges.into_iter().map(|edge| edge.into()).collect()
                );
                out
            })
        // println!("Crossing graph: {:?}", output);
    }
    // ANCHOR_END: graphs_crossing
}

#[cfg(test)]
mod test {
    use crate::graphs::Graph;
    use super::*;

    #[test]
    fn test_min_cut() {

        // test dataset: Array[ (input_graph, minimum expected edges) ]
        let adj_list: Vec<(Vec<Vec<Node>>, usize)> = vec![
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
                4
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
                4
            ),
            (
                vec![
                    vec![1, 2, 4],
                    vec![2, 3, 1, 4],
                    vec![3, 4, 2],
                    vec![4, 1, 3, 2]
                ],
                4
            )
        ];

        for (input, output) in adj_list {
            let g = Graph::import_edges( &input ).expect("Error: Couldn't load input edges");
            let mc = g.minimum_cut();
            assert!(mc.is_some());
            let edges = mc.unwrap().export_edges();
            assert_eq!( edges.len(), output );
            println!("------------");
        }
    }
    #[test]
    fn test_min_cut_txt_graph() {

        let test_data = vec![
            ("src/graphs/txt/mc_input_random_1_6.txt", 4)
            ,("src/graphs/txt/mc_input_random_10_25.txt", 12)
            ,("src/graphs/txt/mc_input_random_20_75.txt", 32)
            ,("src/graphs/txt/mc_input_random_40_200.txt", 122)
        ];

        test_data.into_iter()
            .for_each(|(fname, cuts)| {
                let g = Graph::import_text_graph(fname, ' ', '\0').unwrap_or_else(|| panic!("Cannot open file: {}",fname));
                let mc = g.minimum_cut();
                assert!(mc.is_some());
                let edges = mc.unwrap().export_edges();
                println!(">> Min-cut: {:?}",edges);
                assert_eq!( edges.len(), cuts );
                println!("--------------------");
        })
    }
}