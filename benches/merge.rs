#![feature(test)]
extern crate test;

use csx3::random_sequence;
use csx3::merge::{Merge, MergeIterator};
use test::Bencher;

const LENGTH: usize = 10000;

#[bench]
fn bench_merge_iterator(b:&mut Bencher) {
    let mut v: Vec<i16> = random_sequence(LENGTH);
    let (s1,s2) = v.split_at_mut(LENGTH >> 1);
    s1.sort();
    s2.sort();

    b.iter(|| {
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(LENGTH >> 1);
        let (_,_): (Vec<usize>,Vec<i16>) = MergeIterator::new(s1.iter(),s2.iter()).unzip();
    });
}
#[bench]
fn bench_merge_lazy(b: &mut Bencher) {
    let mut v: Vec<i16> = random_sequence(LENGTH);
    let (s1,s2) = v.split_at_mut(LENGTH >> 1);
    s1.sort();
    s2.sort();

    b.iter(||{
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(LENGTH >> 1);
        let (_,_) = s1.merge_lazy(s2);
    });
}
#[bench]
fn bench_merge_mut(b: &mut Bencher) {
    let mut v: Vec<i16> = random_sequence(LENGTH);
    let (s1,s2) = v.split_at_mut(LENGTH >> 1);
    s1.sort();
    s2.sort();

    b.iter(|| {
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(LENGTH >> 1);
        s1.merge_mut(s2)
    });
}
#[bench]
fn bench_merge_mut_adjacent(b: &mut Bencher) {
    let mut v: Vec<i16> = random_sequence(LENGTH);
    let (s1,s2) = v.split_at_mut(LENGTH >> 1);
    s1.sort();
    s2.sort();

    b.iter(|| {
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(LENGTH >> 1);
        let _ = s1.merge_mut_adjacent(s2);
    });
}
