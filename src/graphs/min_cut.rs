use rand::{Rng, thread_rng};
use super::*;

#[derive(Clone,Copy,Hash)]
struct Edge(Node,Node);
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl Eq for Edge {}
impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("E")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
/*
impl Edge {
    fn starts_at(&self, e: &Self) -> bool {
        self.0 == e.1
    }
    fn ends_at(&self, e: &Self) -> bool {
        self.1 == e.0
    }
    fn is_adjacent(&self, other:&Self) -> bool {
        (self.0 == other.0 || self.0 == other.1) || (self.1 == other.0 || self.1 == other.1)
    }
    fn is_loop(&self) -> bool {
        self.0 == self.1
    }
    fn start(&mut self, e: &Self) { self.0 = e.1; }
    fn end(&mut self, e: &Self) { self.1 = e.0; }
    fn reverse(&self) -> Edge { Edge(self.1, self.0) }
}
 */

trait MinimumCut {
    fn min_cuts(&self) -> Option<Graph>;
}

impl MinimumCut for Graph {
    fn min_cuts(&self) -> Option<Graph> {

        if self.edges.is_empty() {
            return None;
        }

        // define super node and super edge structures
        let mut super_nodes = HashMap::<Node,HashSet<Node>>::new();
        let mut super_edges = HashSet::<Edge>::new();

        // extract edges and nodes for constructing the supersets
        let Graph { edges, nodes:_ } = self;

        // initialise super node & super edge
        edges.iter()
            .for_each(|(src, dests)| {
                super_nodes
                    .entry(*src)
                    .or_insert( HashSet::new() )
                    .insert(*src);

                dests.iter()
                    .for_each(|dst| {
                        super_edges.insert( Edge(*src,*dst));
                    })
            });

        println!("Super Nodes: {:?}",super_nodes);
        println!("Super Edges: {:?}",super_edges);

        while super_nodes.len() > 2 {
            // select a random edge
            let len = super_edges.len();
            let idx = thread_rng().gen_range(0..len-1);
            // get a copy rather a reference so we don't upset the borrow checker
            let Edge(src,dst) = super_edges.iter().nth(idx).copied().unwrap();
            println!("Random Edge: ({src},{dst})");

            // remove nodes forming the random edge
            let super_src = super_nodes.remove(&src).unwrap();
            let super_dst = super_nodes.remove(&dst).unwrap();
            // combine the nodes into a new super-node
            // in this case we use the src we've just removed as the new super node
            let super_node = super_src.union(&super_dst).copied().collect::<HashSet<Node>>();
            println!("Merged super node: {src}->{:?}", super_node);
            super_nodes.entry(src).or_insert(super_node);

            // collapse / remove the obvious edge loops
            super_edges.remove(&Edge(src,dst));
            super_edges.remove(&Edge(dst,src));

            // find all bad edges; the ones affected
            let bad_edges = super_edges.iter()
                // remove the reference
                .copied()
                // filter out those not affected
                .filter_map(|e| if e.0 == dst || e.1 == dst { Some(e) } else { None } )
                // collect any remaining
                .collect::<HashSet<Edge>>();

            // now just remove, fix and reinsert edges
            for mut e in bad_edges {
                // we have only bad edges here hence this code does not have to deal with good edges
                // hence go and remove the bad edge
                print!("Remove:{:?}={} -- ",e,super_edges.remove(&e));
                // fix the edge
                if e.0 == dst { e.0 = src }
                if e.1 == dst { e.1 = src }
                // insert back the fixed edge
                println!("Insert:{:?}={}",e,super_edges.insert(e));
            }

            println!("Round done\n=======");
            println!("Super Nodes: {:?}",super_nodes);
            println!("Super Edges: {:?}",super_edges);
        }


        None
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
/*
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

        let adj_list: [Vec<Node>;4] = [
            vec![1, 2, 4],
            vec![2, 3, 1, 4],
            vec![3, 4, 2],
            vec![4, 1, 3, 2]
        ];
*/
        let g = Graph::import_edges( &adj_list ).expect("Error: Couldn't load edges");
        println!("{:?}",g.min_cuts());
    }
}