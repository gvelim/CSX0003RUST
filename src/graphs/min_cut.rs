use rand::{Rng, thread_rng};
use super::*;

trait MinimumCut {
    fn min_cuts(&self) -> Option<Graph>;
}

impl MinimumCut for Graph {
    fn min_cuts(&self) -> Option<Graph> {

        if self.edges.is_empty() {
            return None;
        }
        let Graph { mut edges, mut nodes } = self.clone();

        while nodes.len() > 2 {
            // pick at random src node out of the nodes remaining
            let s_node = thread_rng().gen_range(1..edges.len()) as Node;

            // pick dst node out of the first edge in the adjacent list
            // edge (src, dst)
            let d_node = edges.get(&s_node).unwrap().clone().iter().nth(0).copied().unwrap();
            println!("Edge({s_node},{d_node})");

            // collect all incoming edges to src node
            // remove src node from the adjacent list
            let s_edges = edges.remove(&s_node).unwrap();
            // collect all incoming edges ro dst node
            // remove dst node from the adjacent list
            let d_edges = edges.remove(&d_node).unwrap();
            
            // Collapsing src and dst into a new node
            // the new node owns the collection of edges 
            let mut col_edges: HashSet<Node> = s_edges.union(&d_edges).copied().collect();

            // clear edge collection from loops
            col_edges.remove(&s_node);
            col_edges.remove(&d_node);

            print!("src: {:?}->{:?} ", s_node, s_edges);
            print!("dst: {:?}->{:?} ", d_node, d_edges);
            println!("col->{:?}",col_edges);

            // scan the remaining graph and remove edges with reference to src and dst nodes
            // for each node, remove edges refer to dst and src nodes
            // since we don't create a new node out of src and dst
            // we keep src as the new and remove dst only
            edges.iter_mut()
                .for_each(| (_, edges)| {
                    edges.remove(&d_node);
                });

            // remove dst node; already removed from adjacent list
            nodes.remove(&d_node);
            
            // add src back, however as the new node with merged edges 
            edges.entry(s_node).or_insert(col_edges);
            println!("Round Finish: \nEdges:{:?}\nNodes:{:?}\n",edges, nodes);

        }

        Some( Graph{edges, nodes} )
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
*/
        let adj_list: [Vec<Node>;4] = [
            vec![1, 2, 4],
            vec![2, 3, 1, 4],
            vec![3, 4, 2],
            vec![4, 1, 3, 2]
        ];

        let g = Graph::import_edges( &adj_list ).expect("Error: Couldn't load edges");
        println!("{:?}",g.min_cuts());
    }
}