
fn main() {
    (0..=10).map(|e| e*e).any(|e| { println!("{e}"); e > 10} );
}

fn is_subsequence_v1(s: &str, t: &str) -> bool {
    let mut cur = 0;
    s.chars()
        .all(|e|
            t[cur..]
                .find(e)
                .and_then(|i| { cur += i+1; Some(i) })
                .is_some()
        )
}
fn is_subsequence_v2(s: &str, t: &str) -> bool {
    let mut iter = t.bytes();
    s.bytes().all(|e| iter.any(|tc| tc == e ) )
}
fn num_matching_subseq(s: &str, words: &[&str]) -> i32 {
    words.iter()
        .fold(0, |mut count, word| {
            is_subsequence_v2(word,s )
                .then(|| count += 1 );
            count
        })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        let data = vec![
            ("abc","ahbgdc",true)
            ,("axc","ahxgdc",true)
            ,("acx","ahxgdc",false)
            ,("axc","ahbgdc",false)
        ];

        data.into_iter()
            .for_each(|(s,t,result)|{
                assert_eq!(is_subsequence_v1(s,t),result);
                assert_eq!(is_subsequence_v2(s,t),result);
            });
    }
    #[test]
    fn test_num_matching_subseq() {
        let data = vec![
            (&["a","bb","acd","ace"],"abcde",3)
            ,(&["ahjpjau","ja","ahbwzgqnuk","tnmlanowax"],"dsahjpjauf",2)
        ];

        data.into_iter()
            .for_each(|(s,t,result)|{
                assert_eq!(num_matching_subseq(t,s),result);
            });
    }
}