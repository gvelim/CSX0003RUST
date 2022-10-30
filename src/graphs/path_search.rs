use std::collections::{VecDeque, BinaryHeap};
use super::{*, NodeType::{NC}, NodeState::{Discovered, Undiscovered}};

trait BFSearch {
    type Output;
    type QueueItem;

    /// Initialise the Path search given the starting node
    fn initiate(&mut self, start:Node) -> &mut Self;
    /// Pull an Item from the queue
    fn pop(&mut self) -> Option<Self::QueueItem>;
    /// Extract Node value from a Queued Item
    fn node_from_queued(&self, qt: &Self::QueueItem) -> Node;
    /// Pre-process item retrieved from the queue and before proceeding with queries the Edges
    /// return true to proceed or false to abandon node processing
    fn pre_process_node(&mut self, _node: Node) -> bool { true }
    /// Process node after all edges have been processes and pushed in the queue
    fn post_process_node(&mut self, _node: Node) { }
    /// Has the node been Discovered ?
    fn is_discovered(&self, _node: NodeType) -> bool;
    /// Process the Edge node and
    /// return 'true' to proceed with push or 'false' to skip the edge node
    fn pre_process_edge(&mut self, src: Node, dst: NodeType) -> bool;
    /// Construct a Queued Item from the Node
    fn node_to_queued(&self, node: Node) -> Self::QueueItem;
    /// Push to the queue
    fn push(&mut self, item: Self::QueueItem);
    /// Retrieve path
    fn extract_path(&self, start: Node) -> Self::Output;
    /// Path Search function
    fn path_search(&mut self, g: &Graph, start: Node, goal:Node) -> Option<Self::Output> {
        // initiate BFSearch given a start node
        self.initiate(start);
        // until no items left for processing
        while let Some(qt) = self.pop() {
            //Extract the src node
            let src = self.node_from_queued(&qt);
            // pre-process and if false abandon and proceed to next item
            if !self.pre_process_node(src) { continue };
            // if we have reached our goal return the path
            if src == goal {
                return Some(self.extract_path(goal))
            }
            // given graph's edges
            // get src node's edges and per their NodeType
            if let Some(edges) = g.edges.get(&src) {
                edges.iter()
                    .for_each(|&dst| {
                        // if dst hasn't been visited AND pre-processing results to true,
                        // then push to explore further
                        if !self.is_discovered(dst)
                            && self.pre_process_edge(src, dst) {
                            // push dst node for further processing
                            self.push(self.node_to_queued(dst.into()))
                        }
                    });
                // Process node after edges have been discovered and pushed for further processing
                self.post_process_node(src);
            }
        }
        None
    }
}

trait PathSearch {
    fn path_distance(&self, start:Node, goal:Node) -> Option<(Vec<Node>, Cost)>;
    fn path_shortest(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)>;
}

impl PathSearch for Graph {

    // ANCHOR: graphs_search_path_shortest
    fn path_distance(&self, start:Node, goal:Node) -> Option<(Vec<Node>, Cost)> {
        struct PDState {
            tracker: Tracker,
            queue: VecDeque<Node>
        }
        impl PDState {
            fn new(g: &Graph) -> PDState {
                PDState {
                    tracker: g.get_tracker(Undiscovered, 0, None),
                    queue: VecDeque::<Node>::new()
                }
            }
        }
        impl BFSearch for PDState {
            type Output = (Vec<Node>, Cost);
            type QueueItem = Node;

            fn initiate(&mut self, node:Node) -> &mut Self {
                self.queue.push_back(node);
                self.tracker[node].visited(Discovered);
                self
            }
            fn pop(&mut self) -> Option<Self::QueueItem> { self.queue.pop_front() }
            fn node_from_queued(&self, node: &Self::QueueItem) -> Node { *node }
            fn is_discovered(&self, node: NodeType) -> bool { self.tracker[node.into()].is_discovered() }
            fn pre_process_edge(&mut self, src: Node, dst: NodeType) -> bool {
                let level = self.tracker[src].dist + 1;
                // mark visited, calculate distance & store parent for distance
                self.tracker[dst.into()].visited(Discovered)
                    .distance(level)
                    .parent(src);
                true
            }
            fn node_to_queued(&self, node: Node) -> Self::QueueItem { node }
            fn push(&mut self, item: Self::QueueItem) { self.queue.push_back(item) }
            fn extract_path(&self, start: Node) -> Self::Output { self.tracker.extract(start) }
        }

        PDState::new(&self).path_search(&self, start, goal )
    }
    // ANCHOR_END: graphs_search_path_shortest
    // ANCHOR: graphs_search_path_min_cost
    fn path_shortest(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
        struct PSState {
            tracker: Tracker,
            queue: BinaryHeap<Step>
        }
        impl PSState {
            fn new(g:&Graph) -> PSState {
                PSState {
                    // reset all node costs to MAX value with no path-parent nodes
                    tracker: g.get_tracker(Undiscovered, Cost::MAX, None),
                    // We are using a BinaryHeap queue in order to always have first in the queue
                    // the node with lowest cost to explore next
                    queue: BinaryHeap::new()
                }
            }
        }
        impl BFSearch for PSState {
            type Output = (Vec<Node>,Cost);
            type QueueItem = Step;

            fn initiate(&mut self, start: Node) -> &mut Self {
                // set cost at start node to zero with no parent node
                self.tracker[start].distance(0);
                // push start node in the BinaryHeap queue
                self.queue.push(Step(start,0));
                self
            }
            fn pop(&mut self) -> Option<Self::QueueItem> { self.queue.pop() }
            fn node_from_queued(&self, qt: &Self::QueueItem) -> Node {
                let &Step(node, _) = qt;
                node
            }
            fn post_process_node(&mut self, node: Node) {
                self.tracker[node].visited(Discovered);
            }
            fn is_discovered(&self, node: NodeType) -> bool { self.tracker[node.into()].is_discovered() }
            fn pre_process_edge(&mut self, src:Node, dst: NodeType) -> bool {
                if let NC(dst, cost) = dst {
                    // calc the new path cost to edge
                    let edge_cost = self.tracker[src].dist + cost;
                    // if new cost is better than previously found
                    if edge_cost > self.tracker[dst].dist  { false }
                    else {
                        // set the new lower cost @node along with related parent Node
                        self.tracker[dst].distance(edge_cost).parent(src);
                        true
                    }
                }
                else {
                    panic!("pre_process_edge(): Must use NodeType::NC")
                }
            }
            fn node_to_queued(&self, node: Node) -> Self::QueueItem {
                Step(node, self.tracker[node].dist )
            }
            fn push(&mut self, item: Self::QueueItem) { self.queue.push(item) }
            fn extract_path(&self, start: Node) -> Self::Output { self.tracker.extract(start) }
        }

        PSState::new(self).path_search(self,start,goal)

    }
// ANCHOR_END: graphs_search_path_min_cost
}

#[cfg(test)]
mod test {
    use crate::graphs::{Graph, Node};
    use super::*;

    #[test]
    fn  test_path_search_small_graph() {
        // ( input graph, starting node, array with expected distances)
        let test_data:(Vec<Vec<Node>>, Node, Vec<(Node, Option<(Vec<Node>,usize)>)>) =
            (
                vec![
                    vec![1, 2, 3],
                    vec![2, 4],
                    vec![3, 7],
                    vec![4, 6],
                    vec![5, 7],
                    vec![6, 8],
                    vec![7, 8],
                    vec![8, 1],
                ],
                1,
                vec![
                    (1, Some((vec![1],0))),
                    (2, Some((vec![1,2],1))),
                    (3, Some((vec![1,3],1))),
                    (4, Some((vec![1,2,4],2))),
                    (5, None),
                    (6, Some((vec![1,2,4,6],3))),
                    (7, Some((vec![1,3,7],2))),
                    (8, Some((vec![1,3,7,8],3)))
                ]
            );
        let (inp, start, out) = test_data;
        let g = Graph::import_edges(&inp).expect("couldn't load edges");

        out.into_iter()
            .for_each(|(goal, exp)|{
                let out = g.path_distance(start, goal);
                println!("Inp: {start}->{goal} => Output: {:?} / Expected: {:?}", out, exp);
                assert_eq!(out, exp);
            });
    }
    #[test]
    fn test_path_search_large_graph() {
        let data = vec![
            (7 as Node, 3 as Cost),
            (37, 2),
            (59, 2),
            (82, 1),
            (99, 2),
            (115, 2),
            (133, 2),
            (165, 3),
            (188, 2),
            (197, 2)
        ];
        let g = Graph::import_text_graph("src/graphs/txt/ps_input_random_10_16.txt", '\t',',').expect("graph couldn't be loaded");

        data.into_iter()
            .for_each(|(goal, dist)| {
                let path = g.path_distance(1, goal);
                println!("1->{goal} => {:?} :: Expect: {dist}", path);
                assert!(path.is_some());
                assert_eq!(path.unwrap().1, dist);
            })
    }
    #[test]
    fn test_path_shortest_small_graph() {
        let data = vec![
            (1 as Node, 0 as Cost),
            (2, 0),
            (3, 1),
            (4, 1),
            (5, 4),
            (6, 3)
        ];
        let edge_list = include!("small_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        data.into_iter()
            .for_each(|(goal, cost)| {
                let path = g.path_shortest(2, goal);
                assert!(path.is_some());
                println!("2->{goal} => {:?} :: Expect: {cost}", path);
                assert_eq!(path.unwrap().1, cost);
            })
    }
    #[test]
    fn test_path_shortest_large_graph() {
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
        let g = Graph::import_text_graph("src/graphs/txt/ps_input_random_10_16.txt",'\t',',').expect("graph couldn't be loaded");

        data.into_iter()
            .for_each(|(goal, cost)| {
                let path = g.path_shortest(1, goal);
                assert!(path.is_some());
                println!("1->{goal} => {:?} :: Expect: {cost}", path);
                assert_eq!(path.unwrap().1, cost);
            })
    }
}