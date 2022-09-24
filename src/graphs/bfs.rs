use std::collections::{VecDeque, HashMap};
use crate::graphs::*;

trait BFS {
    fn shortest_path(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)>;
    fn calc_distances(&self, start:Node) -> HashMap<usize, HashSet<Node>>;
    fn dist_to_adjacentlist(distances: Vec<usize>) -> HashMap<usize, HashSet<Node>>;
}

impl BFS for Graph {

    fn shortest_path(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {

        let mut path: Vec<Node> = Vec::new();
        let mut queue = VecDeque::new();

        // reset all node costs to MAX value with no path-parent nodes
        let mut node_cost = self.nodes.iter()
            .fold( HashMap::<Node,(Cost,Option<Node>)>::new(), |mut cost_history, node| {
                cost_history.entry(*node).or_insert( (Cost::MAX, None));
                cost_history
            });
        // set cost at start node to zero with no parent
        node_cost.entry(start)
            .and_modify(
                |c| *c = (0, None)
            );
        let mut best_path = None;

        // push start node in the DFS queue
        queue.push_front(start);

        // while a node in the queue pick the node
        while let Some(node) = queue.pop_front() {

            let path_cost = node_cost[&node].0;

            // if node is the target node
            // assuming cost is the lowest cost
            if node == goal {
                // clear path for use in case we find another path
                // build the shortest path by pushing target node first
                path.clear();
                path.push(node);
                // set target as current node
                let mut cur_node= node;
                // backtrace all parents until you reached None, that is, the start node
                while let Some(parent) = node_cost[&cur_node].1 {
                    path.push(parent);
                    cur_node = parent;
                }
                best_path = Some((path.clone(), path_cost));
                println!("\t Path!: {:?}", best_path);
            } else {
                if let Some(edges) = self.edges.get(&node) {

                    edges.iter()
                        .filter_map(|&edge|
                            if let NodeType::NC(node,cost) = edge { Some((node,cost))} else { panic!("Must use NodeType::NC") }
                        )
                        .for_each(|(edge, cost)| {
                            // calc the new path cost to edge
                            let edge_cost = path_cost + cost;

                            // if new edge cost < currently known cost @edge
                            if edge_cost < node_cost[&edge].0 {

                                // set the new lower cost @node along with related parent Node
                                node_cost.entry(edge)
                                    .and_modify(|c|
                                        *c = (edge_cost, Some(node))
                                    );
                                // push_front for Depth First Search -> slower but finds all paths
                                // push_back for Breadth First Search -> faster but finds best only
                                queue.push_back(edge);
                            }
                        })
                }
            }

        }

        println!("Path: {:?}", best_path);
        best_path
    }

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
                        NodeType::N(dst) | NodeType::NC(dst, _) => {
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
    #[test]
    fn small_graph() {
        let edge_list = include!("small_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        let path = g.shortest_path(2, 5);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 4);
    }
}