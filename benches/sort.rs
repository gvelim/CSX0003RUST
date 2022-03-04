#![feature(test)]
extern crate test;

use csx3::random_sequence;
use csx3::{
    sort::{
        count::CountSort,
        merge::MergeSort,
        quick::QuickSort
    },
    merge::Merge};
use test::Bencher;

const LENGTH:usize = 5000;

#[bench]
fn bench_mergesort_mut_adjacent(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(LENGTH);
    b.iter(||{
        v.clone().mergesort_mut(Merge::merge_mut_adjacent)
    });
}
#[bench]
fn bench_mergesort_mut(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(LENGTH);
    b.iter(||{
        v.clone().mergesort_mut(Merge::merge_mut)
    });
}
#[bench]
fn bench_mergesort(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(LENGTH);
    b.iter(||{
        v.clone().mergesort()
    });
}

#[bench]
fn bench_countsort(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(LENGTH);
    b.iter(||{
        v.clone().count_sort()
    });
}

#[bench]
fn bench_quicksort(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(LENGTH);
    b.iter(||{
        v.clone().quick_sort()
    });
}

#[bench]
fn bench_std_vector_sort(b: &mut Bencher) {
    let v: Vec<i16> = random_sequence(LENGTH);
    b.iter(||{
        v.clone().sort()
    });
}
