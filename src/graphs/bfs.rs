use std::collections::VecDeque;
use crate::graphs::*;

trait BreadthFirstSearch {
    fn calc_distances(&self, start:Node) -> Vec<(Node, u32)>;
}

impl BreadthFirstSearch for Graph {
    fn calc_distances(&self, start:Node) -> Vec<(Node, u32)> {

        // setup queue
        let mut queue = VecDeque::<Node>::new();

        // holds whether a node has been visited
        let mut visited = vec![false; self.nodes.len()];
        let mut distance = vec![0u32; self.nodes.len()];

        queue.push_back(start);
        visited[start-1] = true;

        while let Some(src) = queue.pop_front() {

            // get graph edges from src node
            let edges = self.edges.get(&src).unwrap();

            // scan each dst from src node
            edges.iter()
                .for_each(|&dst| {
                    // if not visited yet
                    if !visited[dst-1] {
                        // mark visited
                        visited[dst-1] = true;
                        // calculate distance
                        distance[dst-1] = distance[src -1] + 1;
                        // push at the back of the queue for further scanning
                        queue.push_back(dst);
                    }
                });
        }

        distance.into_iter()
            .enumerate()
            .fold( Vec::<(Node, u32)>::new(),|mut  out, (node, dist)| {
                out.push( (node+1, dist));
                out
            })
    }
}

#[cfg(test)]
mod test {
    use crate::graphs::{Graph, Node};
    use super::*;

    #[test]
    fn test_bfs_shortest_path() {
        let test_data:(Vec<Vec<Node>>, Node, Vec<(Node, u32)>) =
            (
                vec![
                    vec![1, 2, 3],
                    vec![2, 4, 5],
                    vec![3, 5],
                    vec![4, 6],
                    vec![5, 7],
                    vec![6, 8],
                    vec![7, 8],
                    vec![8, 1],
                ],
                1,
                vec![
                    (1,0),
                    (2,1),
                    (3,1),
                    (4,2),
                    (5,2),
                    (6,3),
                    (7,3),
                    (8,4)
                ]
            );
        let (inp, start, out) = test_data;
        let g = Graph::import_edges(&inp).expect("couldn't load edges");
        assert_eq!(g.calc_distances(start), out);
    }

}