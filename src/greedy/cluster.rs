use std::collections::{BinaryHeap};
use crate::graphs::{
    Graph, Edge, NodeType::NC,
    min_cut::SuperNodes
};

// ANCHOR: graphs_mst_cluster_def
struct ClusterSet {
    mst: Graph,
    clusters : SuperNodes
}

impl ClusterSet {
    /// spacing of a clustering. It's the distance between the closest together pair of separated points
    /// We want all of the separated points to be as far apart as possible.
    /// That is, we want the spacing to be big. The bigger the better
    fn spacing(&self) -> Edge {
        self.crossing_edges().pop().unwrap_or_else(|| panic!("spacing(): no edges found spanning the clusters"))
    }
    fn crossing_edges(&self) -> BinaryHeap<Edge>{

        let mut input = self.mst.get_edges_by_cost();
        let mut output = BinaryHeap::<Edge>::new();

        while let Some(edge) = input.pop() {
            let Edge(src, dst) = edge;
            if self.clusters.find_supernode(&src) != self.clusters.find_supernode(&dst.into()) {
                output.push(edge);
            }
        }
        output
    }
}

// ANCHOR_END: graphs_mst_cluster_def
// ANCHOR: graphs_mst_cluster_impl
trait Clustering {
    fn find_clusters(&self, k: usize) -> Option<ClusterSet>;
}

impl Clustering for Graph {

    fn find_clusters(&self, k: usize) -> Option<ClusterSet> {

        // Get the ordered heap by lowest cost Edge on top
        let mut heap = self.get_edges_by_cost();
        // Keeps the graph's components, that is, a super node is a graph component's lead node
        // The initial state is for each node to be a lead component node with a component of its own
        let mut snodes = self.get_super_nodes();
        // the output graph that will hold *only* the edges
        // that form the minimum spanning tree
        let mut graph = Graph::new();
        let mut clusters = None;

        // As long as more than 2 components
        while snodes.len() > 1 {
            // get the edge with the lowest cost
            // otherwise if we've run out of edges while there are 2 or more components
            // then the graph IS NOT CONNECTED
            let Some(edge) = heap.pop() else { return None };
            let Edge(src, NC(dst, _)) = edge else { panic!("find_clusters() - Cannot find NodeType::NC") };
            // print!("({src:2}->{dst:2}):{cost:6} - ");

            // if src is not a super node then get its super node
            let src = snodes.find_supernode(&src);
            // if dst is not a super node then get its super node
            let dst = snodes.find_supernode(&dst);

            // if src component differs from dst component then merge the two and save the edge connecting them
            if src != dst {
                snodes.merge_nodes(src, dst);
                graph.push_edge(edge);
                // println!("Store");
            } else {
                // println!("Skip");
            }
            if snodes.len() == k {
                clusters = Some(snodes.clone())
            }
        }
        Some(ClusterSet{
            mst: graph,
            clusters: clusters.unwrap()
        })
    }
}
// ANCHOR_END: graphs_mst_cluster_impl

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_clustering() {
        let test_data = vec![
            ("src/greedy/txt/cst_input_completeRandom_1_8.txt", 21)
            ,("src/greedy/txt/cst_input_completeRandom_10_32.txt", 90)
            ,("src/greedy/txt/cst_input_completeRandom_20_128.txt", 578)
            ,("src/greedy/txt/cst_input_completeRandom_30_1024.txt", 5999)
        ];
        for (filename, result) in test_data {
            println!("{filename}");
            let mut g = Graph::new();
            let edge = g.load_file_mst(filename)
                .find_clusters(4)
                .unwrap_or_else(|| panic!("Returned None instead of a ClusterSet"))
                .spacing();
            print!("Edge: {:?}", edge);
            let Edge(_,NC(_,distance)) = edge else { panic!("NodeType::N used instead of NodeType::NC") };
            println!(" => Expected {:?}", result);
            assert_eq!( distance, result );
        }
    }
}