#![feature(test)]
extern crate test;

use csx3::random_sequence;
use csx3::merge::Merge;
use test::Bencher;

#[bench]
fn bench_merge_lazy(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(10000);
    let mid = v.len()>>1;

    b.iter(||{
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(mid);
        let (_,_) = s1.merge_lazy(s2);
    });
}
#[bench]
fn bench_merge_mut(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(10000);
    let mid = v.len()>>1;

    b.iter(move || {
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(mid);
        s1.merge_mut(s2)
    });
}
#[bench]
fn bench_merge_mut_adjacent(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(10000);
    let mid = v.len()>>1;

    b.iter(move ||{
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(mid);
        s1.merge_mut_adjacent(s2)
    });
}
