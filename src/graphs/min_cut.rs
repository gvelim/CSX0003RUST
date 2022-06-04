use rand::Rng;
use super::*;

trait MinimumCut {
    fn min_cuts(&self) -> Option<Graph>;
}

impl MinimumCut for Graph {
    fn min_cuts(&self) -> Option<Graph> {
        if self.edges.is_empty() {
            return None;
        }

        let mut graph = self.clone();

        while graph.nodes.len() > 2 {
            println!("{:?}",&graph);
            // choose a random Node / Edge to start with;
            let idx = rand::thread_rng().gen_range(1..graph.nodes.len());
            let src = graph.nodes.iter().nth(idx).unwrap().clone();

            // extract edges from src node
            let old_dst = graph.edges.remove(&src).unwrap();
            println!("{src}=>{:?}", old_dst);

            // Collapse all edges originating from source node
            let new_dst = old_dst.iter()
                // for each dst node
                .fold( HashSet::<Node>::new(), |new_src_edges, dst| {
                    // extract edges originating from dst node
                    let dst_edges = graph.edges.remove(dst).unwrap();
                    // collapse/remove dst node
                    graph.nodes.remove(dst);
                    // move edges originating from dst node onto source node
                    new_src_edges.union(&dst_edges).copied().collect()
                });
            println!("{src}=>{:?}", new_dst);
            println!("{src}=>{:?}",new_dst.difference(&old_dst));

            // find and replace all dst references to src node
            graph.edges.iter_mut()
                .for_each(|n| {

                });

            graph.edges.insert(src, new_dst.difference(&old_dst).copied().collect());
        }
        Some( graph )
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

        let g = Graph::import_edges( &adj_list).expect("Error: Couldn't load edges");
        println!("{:?}",g.min_cuts());
    }
}