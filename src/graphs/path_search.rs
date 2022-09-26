use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use crate::graphs::*;
use NodeType::{NC};

trait PathSearch {
    fn shortest_path(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)>;
}

impl PathSearch for Graph {
    fn shortest_path(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {

        // We are using a BinaryHeap queue in order to always have first in the queue
        // the node with lowest cost to explore next
        struct Step(Node,Cost);
        impl Eq for Step {}
        impl PartialEq<Self> for Step {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        impl PartialOrd<Self> for Step {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Step {
            fn cmp(&self, other: &Self) -> Ordering {
                // binary head is a max-heap implementation pushing to the top the biggest element self.cmp(other)
                // hence we need to reverse the comparison other.cmp(self)
                other.1.cmp(&self.1)
            }
        }

        let mut queue = BinaryHeap::new();

        // reset all node costs to MAX value with no path-parent nodes
        let mut node_cost = self.nodes.iter()
            .fold( HashMap::<Node,(Cost, Option<Node>)>::new(), |mut cost_history, node| {
                cost_history.entry(*node).or_insert( (Cost::MAX, None));
                cost_history
            });
        // set cost at start node to zero with no parent node
        node_cost.entry(start)
            .and_modify(
                |c| *c = (0, None)
            );

        // push start node in the BinaryHeap queue
        queue.push(Step(start,0));

        // while queue has nodes pick the node with the lowest cost
        while let Some(Step(node,_)) = queue.pop() {

            let path_cost = node_cost[&node].0;

            // if we have found the the target node
            // then we have completed our search
            // (Dijkstra's algo property - all nodes are processed once)
            if node == goal {
                let mut path = VecDeque::new();
                // reconstruct the shortest path starting from the target node
                path.push_front(node);
                // set target as current node
                let mut cur_node= node;
                // backtrace all parents until you reach None, that is, the start node
                while let Some(parent) = node_cost[&cur_node].1 {
                    path.push_front(parent);
                    cur_node = parent;
                }
                println!("\t Path!: {:?} [{path_cost}]", path);
                return Some((path.into(), path_cost));
            } else {
                if let Some(edges) = self.edges.get(&node) {
                    edges.iter()
                        .filter_map(|&edge| match edge {
                            NC(node, cost) => Some((node, cost)),
                            _ => panic!("Must use NodeType::NC")
                        })
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
                                queue.push(Step(edge, edge_cost));
                            }
                        })
                }
            }
        }

        println!("Cannot find a path !!");
        None
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