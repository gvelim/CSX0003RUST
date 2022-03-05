#![feature(test)]
extern crate test;

use csx3::random_sequence;
use csx3::merge::{Merge, MergeIterator};
use test::Bencher;

const LENGTH: usize = 25000;

#[bench]
fn bench_merge_iterator(b:&mut Bencher) {
    let mut v: Vec<i16> = random_sequence(LENGTH);
    let mut v1: Vec<i16> = random_sequence(LENGTH);
    v1.sort();
    v.sort();
    v.extend(v1);
    let mid = v.len() >> 1;

    b.iter(|| {
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(mid);
        let (_,_): (Vec<usize>,Vec<i16>) = MergeIterator::new(s1.iter(),s2.iter()).unzip();
    });
}
#[bench]
fn bench_merge_lazy(b: &mut Bencher) {
    let mut v: Vec<i16> = random_sequence(LENGTH);
    let mut v1: Vec<i16> = random_sequence(LENGTH);
    v1.sort();
    v.sort();
    v.extend(v1);
    let mid = v.len()>>1;

    b.iter(||{
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(mid);
        let (_,_) = s1.merge_lazy(s2);
    });
}
#[bench]
fn bench_merge_mut(b: &mut Bencher) {
    let mut v: Vec<i16> = random_sequence(LENGTH);
    let mut v1: Vec<i16> = random_sequence(LENGTH);
    v1.sort();
    v.sort();
    v.extend(v1);
    let mid = v.len()>>1;

    b.iter(|| {
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(mid);
        s1.merge_mut(s2)
    });
}
#[bench]
fn bench_merge_mut_adjacent(b: &mut Bencher) {
    let mut v: Vec<i16> = random_sequence(LENGTH);
    let mut v1: Vec<i16> = random_sequence(LENGTH);
    v1.sort();
    v.sort();
    v.extend(v1);
    let mid = v.len()>>1;

    b.iter(|| {
        let mut t = v.clone();
        let (s1,s2) = t.split_at_mut(mid);
        let _ = s1.merge_mut_adjacent(s2);
    });
}
