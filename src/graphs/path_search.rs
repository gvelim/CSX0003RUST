use std::cmp::Ordering;
use std::collections::{VecDeque, BinaryHeap};
use std::ops::{Index, IndexMut};
use crate::graphs::*;
use NodeType::{NC};
use crate::graphs::path_search::NodeState::{Discovered, Undiscovered};

// ANCHOR: graphs_search_path_utils
// ANCHOR: graphs_search_path_utils_Step
#[derive(Debug,Copy, Clone)]
pub struct Step(pub Node, pub Cost);

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
        other.1.cmp(&self.1) //then_with(|| other.0.cmp(&self.0))
    }
}
// ANCHOR_END: graphs_search_path_utils_Step
// ANCHOR: graphs_search_path_utils_NodeTrack
#[derive(Debug,Clone,PartialEq)]
pub enum NodeState {
    Undiscovered,
    Discovered,
    Processed
}
#[derive(Debug,Clone)]
pub struct NodeTrack {
    visited:NodeState,
    dist:Cost,
    parent:Option<Node>
}
impl NodeTrack {
    pub fn visited(&mut self, s:NodeState) -> &mut Self {
        self.visited = s; self
    }
    pub fn distance(&mut self, d:Cost) -> &mut Self {
        self.dist = d; self
    }
    pub fn parent(&mut self, n:Node) -> &mut Self {
        self.parent =Some(n); self
    }
    pub fn is_discovered(&self) -> bool {
        self.visited != Undiscovered
    }
}
#[derive(Debug)]
pub struct Tracker {
    list: HashMap<Node, NodeTrack>
}
pub trait Tracking {
    fn extract(&self, start:Node) -> (Vec<Node>, Cost) {
        (self.extract_path(start), self.extract_cost(start))
    }
    fn extract_path(&self, start: Node) -> Vec<Node>;
    fn extract_cost(&self, start: Node) -> Cost;
}
impl Tracking for Tracker {
    fn extract_path(&self, start:Node) -> Vec<Node> {
        let mut path = VecDeque::new();
        // reconstruct the shortest path starting from the target node
        path.push_front(start);
        // set target as current node
        let mut cur_node= start;
        // backtrace all parents until you reach None, that is, the start node
        while let Some(parent) = self[cur_node].parent {
            path.push_front(parent);
            cur_node = parent;
        }
        path.into()
    }
    fn extract_cost(&self, start:Node) -> Cost {
        self[start].dist
    }
}
impl Index<Node> for Tracker {
    type Output = NodeTrack;

    fn index(&self, index: Node) -> &Self::Output {
        &self.list.get(&index).unwrap_or_else(|| panic!("Error: cannot find {index} in tracker {:?}", &self))
    }
}
impl IndexMut<Node> for Tracker {
    fn index_mut(&mut self, index: Node) -> &mut Self::Output {
        self.list.get_mut(&index).unwrap_or_else(|| panic!("Error: cannot find {index} in tracker"))
    }
}
// ANCHOR_END: graphs_search_path_utils_NodeTrack
// ANCHOR: graphs_search_path_utils_NodeTrack_graph
impl Graph {
    pub fn get_tracker(&self, visited:NodeState, dist:Cost, parent:Option<Node>) -> Tracker {
        let n = NodeTrack { visited, dist, parent };
        Tracker{
            list: self.nodes.iter()
                .fold( HashMap::new(), |mut out, &node| {
                    out.entry(node)
                        .or_insert( n.clone() );
                    out
                })
        }
    }
}
// ANCHOR_END: graphs_search_path_utils_NodeTrack_graph
// ANCHOR: graphs_search_path_utils

trait PathSearch {
    fn path_distance(&self, start:Node, goal:Node) -> Option<(Vec<Node>, Cost)>;
    fn path_shortest(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)>;
}

impl PathSearch for Graph {
    // ANCHOR: graphs_search_path_shortest
    fn path_distance(&self, start:Node, goal:Node) -> Option<(Vec<Node>, Cost)> {

        // setup queue
        let mut queue = VecDeque::<Node>::new();

        // holds whether a node has been visited, if yes, it's distance and parent node
        let mut tracker= self.get_tracker(Undiscovered, 0, None);

        queue.push_back(start);
        tracker[start].visited(Discovered);

        while let Some(src) = queue.pop_front() {

            if src == goal {
                return Some(tracker.extract(src))
            }

            self.edges
                // get graph edges from src node
                .get(&src)
                .unwrap_or_else(|| panic!("path_distance(): Cannot extract edges for node {src}"))
                // scan each dst from src node
                .iter()
                .map(|&ntype| ntype.into() )
                .filter(|&dst| {
                    // if visited do not proceed
                    if tracker[dst].is_discovered() { false }
                    else {
                        let level = tracker[src].dist + 1;
                        // mark visited, calculate distance & store parent for distance
                        tracker[dst].visited(Discovered)
                            .distance(level)
                            .parent(src);
                        true
                    }
                })
                // push at the back of the queue for further scanning
                .for_each(|dst| queue.push_back(dst) )
        }
        None
    }
    // ANCHOR_END: graphs_search_path_shortest
    // ANCHOR: graphs_search_path_min_cost
    fn path_shortest(&self, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {

        // We are using a BinaryHeap queue in order to always have first in the queue
        // the node with lowest cost to explore next

        let mut queue = BinaryHeap::new();

        // reset all node costs to MAX value with no path-parent nodes
        let mut tracker= self.get_tracker(Undiscovered, Cost::MAX, None);

        // set cost at start node to zero with no parent node
        tracker[start].distance(0);

        // push start node in the BinaryHeap queue
        queue.push(Step(start,0));

        // while queue has nodes pick the node with the lowest cost
        while let Some(Step(node, _)) = queue.pop() {

            let path_cost= tracker[node].dist;
            // if we have found the the target node
            // then we have completed our search
            // (Dijkstra's algo property - all nodes are processed once)
            if node == goal {
                let path = tracker.extract_path(node);
                println!("\t Path!: {:?} [{path_cost}]", path);
                return Some((path, path_cost));
            }
            if let Some(edges) = self.edges.get(&node) {
                edges.iter()
                    .map(|&edge|
                        if let NC(node, cost) = edge { (node, cost) }
                        else { panic!("Must use NodeType::NC") }
                    )
                    .filter_map(|(edge, cost)| {
                        if tracker[edge].is_discovered() { None }
                        else {
                            // calc the new path cost to edge
                            let edge_cost = path_cost + cost;
                            // if new cost is better than previsously found
                            if edge_cost > tracker[edge].dist  { None }
                            else {
                                // set the new lower cost @node along with related parent Node
                                tracker[edge].distance(edge_cost).parent(node);
                                Some((edge, edge_cost))
                            }
                        }
                    })
                    .for_each(|(edge, edge_cost)| {
                        // push unprocessed edge and cost to the queue
                        queue.push(Step(edge, edge_cost));
                    });
            }
            tracker[node].visited(Discovered);
        }
        println!("Cannot find a path !!");
        None
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
                assert_eq!(path.unwrap().1, cost);
            })
    }
}