use rand::Rng;
use super::*;

trait MinimumCut {
    fn min_cuts(&self) -> Option<HashSet<(Node,Node)>>;
}

impl MinimumCut for Graph {
    fn min_cuts(&self) -> Option<HashSet<(Node,Node)>> {

        if self.edges.is_empty() {
            return None;
        }

        let mut nodes = self.nodes.clone();
        let mut edges = self.edges.clone().into_iter()
            .fold( HashSet::<(Node,Node)>::new(),|mut edges, (src, dst)| {
                dst.into_iter().for_each(|dst| {
                   edges.insert((src,dst));
                });
                edges
            });

        println!("{:?}", edges);

        while nodes.len() > 2 {
            //println!("{:?}",edges);
            // choose a random Node / Edge to start with;
            let idx = rand::thread_rng().gen_range(1..edges.len());
            let col_edge = edges.iter().nth(idx).copied().unwrap();

            // extract edges from src node
            // and remove dst node
            edges.remove(&col_edge);
            let col_src = col_edge.0;
            let col_dst = col_edge.1;
            nodes.remove(&col_dst);

            println!("Edges: {:?} => {:?}", col_edge, edges);
            println!("Nodes: {:?}", nodes);
            // Collect all edges originating from destination node

            let (del_edges, new_edges) = edges.iter()
                .filter(|edge|
                    col_dst == edge.1 || col_dst == edge.0
                )
                .fold( (HashSet::new(), HashSet::new()), |(mut old,  mut new), edge| {
                    old.insert(*edge);
                    if edge.1 == col_dst {
                        new.insert( (edge.0, col_src));
                    } else if edge.0 == col_dst {
                        new.insert( (col_src, edge.1));
                    } else {
                        panic!("{:?}",edge);
                    }
                    new.remove(&(col_src, col_src));
                    (old, new)
                });
            println!("tmp: {:?} => -{:?} +{:?}", col_edge, del_edges, new_edges);

            // find and replace all dst references to src node
            del_edges.into_iter().for_each(|edge| { edges.remove(&edge); } );
            new_edges.into_iter().for_each(|edge| { edges.insert( edge); } );
        }
        Some( edges )
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

        let g = Graph::import_edges( &adj_list).expect("Error: Couldn't load edges");
        println!("{:?}",g.min_cuts());
    }
}