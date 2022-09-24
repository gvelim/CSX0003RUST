use std::collections::{VecDeque, HashMap};
use crate::graphs::*;

trait BFS {
    fn calc_distances(&self, start:Node) -> HashMap<usize, HashSet<Node>>;
    fn dist_to_adjacentlist(distances: Vec<usize>) -> HashMap<usize, HashSet<Node>>;
}

impl BFS for Graph {

    fn calc_distances(&self, start:Node) -> HashMap<usize, HashSet<Node>> {

        // setup queue
        let mut queue = VecDeque::<Node>::new();

        // holds whether a node has been visited
        let mut visited = vec![false; self.nodes.len()];
        let mut distance = vec![0usize; self.nodes.len()];

        queue.push_back(start);
        visited[start-1] = true;

        while let Some(src) = queue.pop_front() {

            self.edges
                // get graph edges from src node
                .get(&src)
                .unwrap()
                // scan each dst from src node
                .iter()
                .for_each(|&dst| {
                    match dst {
                        NodeType::N(dst) | NC(dst, _) => {
                            // if not visited yet
                            if !visited[dst-1] {
                                // mark visited
                                visited[dst-1] = true;
                                // calculate distance
                                distance[dst-1] = distance[src-1] + 1;
                                // push at the back of the queue for further scanning
                                queue.push_back(dst);
                            }
                        }
                    }
                });
        }

        Graph::dist_to_adjacentlist(distance)
    }

    // Input array format [ index = node, distance ]
    // for example, [ 0, 1, 1, 2 ] means
        // starting Node 1: 0 distance
        // then ... Node 2: 1, Node 3: 1, Node 4: 2
    fn dist_to_adjacentlist(distances: Vec<usize>) -> HashMap<usize, HashSet<Node>> {
        distances.into_iter()
            .enumerate()
            .fold( HashMap::<usize, HashSet<Node>>::new(), |mut out, (node, dist)| {
                out.entry(dist)
                    .or_insert(HashSet::<Node>::new())
                    .insert(node+1);
                out
            })
    }

}

#[cfg(test)]
mod test {
    use crate::graphs::{Graph, Node};
    use super::*;

    #[test]
    fn test_bfs_calc_distances() {
        // ( input graph, starting node, array with expected distances)
        let test_data:(Vec<Vec<Node>>, Node, Vec<usize>) =
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
                vec![0, 1, 1, 2, 2, 3, 3, 4]
            );
        let (inp, start, out) = test_data;
        let g = Graph::import_edges(&inp).expect("couldn't load edges");

        let out = Graph::dist_to_adjacentlist(out);
        let inp = g.calc_distances(start);
        println!("Output: {:?}", inp);
        println!("Expected: {:?}", out);
        assert_eq!(inp, out);
    }
}