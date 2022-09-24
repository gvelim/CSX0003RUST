use std::collections::{VecDeque, HashMap};
use crate::graphs::*;

trait PathSearch {
    fn shortest_path(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)>;
}

impl PathSearch for Graph {
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
        let mut best_cost = Cost::MAX;

        // push start node in the DFS queue
        queue.push_front(start);

        // while a node in the queue pick the node
        while let Some(node) = queue.pop_front() {

            let path_cost = node_cost[&node].0;

            // if node is the target node
            // assuming cost is the lowest cost
            if node == goal && path_cost < best_cost {
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
                best_cost = path_cost;
                println!("\t Path!: {:?}", best_path);
            } else {
                if let Some(edges) = self.edges.get(&node) {

                    edges.iter()
                        .filter_map(|&edge|
                            if let NC(node,cost) = edge { Some((node,cost))} else { panic!("Must use NodeType::NC") }
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

}


#[cfg(test)]
mod test {
    use crate::graphs::{Graph, Node};
    use super::*;

    #[test]
    fn small_graph() {
        let edge_list = include!("small_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        let path = g.shortest_path(2, 5);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 4);
    }

    #[test]
    fn large_graph() {
        let data = vec![
            (7 as Node, 588 as Cost),
            (37, 405),
            (59, 675),
            (82, 521),
            (99, 909),
            (115, 328),
            (133, 418),
            (165, 957),
            (188, 830),
            (197, 839)
        ];
        let g = Graph::import_text_graph("src/graphs/input_random_10_16.txt").expect("graph couldn't be loaded");

        data.into_iter()
            .for_each(|(goal, cost)| {
                let path = g.shortest_path(1, goal);
                assert!(path.is_some());
                assert_eq!(path.unwrap().1, cost);
            })
    }
}