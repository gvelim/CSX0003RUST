use super::*;

trait ConnectedComponents {
    fn scc(&self, start: Node) -> HashMap<Node, HashSet<Node>>;
}

impl ConnectedComponents for Graph {
    fn scc(&self, start: Node) -> HashMap<Node, HashSet<Node>> {
        HashMap::default()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_scc_small() {
        assert!(true);
    }
}