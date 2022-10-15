use super::*;

trait ConnectedComponents {
    fn scc(&self, start: Node) -> Option<HashMap<Node, HashSet<Node>>>;
}

impl ConnectedComponents for Graph {
    fn scc(&self, start: Node) -> Option<HashMap<Node, HashSet<Node>>> {

        // intiate tracker


        Some(HashMap::default())
    }
}

#[cfg(test)]
mod test {
    use crate::graphs::Graph;
    use crate::graphs::scc::ConnectedComponents;

    #[test]
    fn test_scc_small() {
        let test_data = vec![
            ("src/graphs/txt/scc_input_mostlyCycles_1_8.txt", &[4,2,2,0,0])
            ,("src/graphs/txt/scc_input_mostlyCycles_8_16.txt", &[13,1,1,1,0])
            ,("src/graphs/txt/scc_input_mostlyCycles_12_32.txt", &[29,3,0,0,0])
        ];

        test_data.into_iter()
            .for_each(|(fname, cuts)| {
                let g = Graph::import_text_graph(fname, ' ', '\0').unwrap_or_else(|| panic!("Cannot open file: {}",fname));
                let mc = g.scc(3);
                assert!(mc.is_some());
                println!(">> Scc: {:?}",mc);
                let scc =
                    mc.unwrap()
                        .into_iter()
                        .fold(vec![], |mut out, (_,set)| {
                            out.push(set.len());
                            out
                        });
                assert_eq!( scc, cuts );
                println!("--------------------");
            });
        assert!(true);
    }
}