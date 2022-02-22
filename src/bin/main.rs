use csx3::sort::*;
use csx3::trees::*;
use csx3::linkedlists::*;
use csx3::select::*;
use std::env::args;
use std::str::FromStr;
use csx3::random_sequence;

type MyType = i8;

fn main() {

    let n = match args().into_iter()
        .skip(1)
        .map(|x| usize::from_str(&x) )
        .next()
        .unwrap_or(Ok(16)) {
             Ok(n) => n,
             Err(err) => {
                 println!("Error: {}\nUse: main <n>\n\t n = number of elements to be sorted;\n\t     default 16", err);
                 std::process::exit(1)
             }
    };

    let list : List<MyType> = random_sequence(n);
    let mut v: Vec<MyType> = list.iter().copied().collect();


    println!("List           : {:?}", v );

    println!("Random Selection");
    let mut arr: Vec<MyType> = list.iter().copied().collect();
    println!("1st order stat= {:?}", r_selection(&mut arr, 1));
    println!("2nd order stat= {:?}", r_selection(&mut arr, 2));
    println!("3rd order stat= {:?}", r_selection(&mut arr, 3));

    println!("Deterministic Selection");
    let mut arr: Vec<MyType> = list.iter().copied().collect();
    println!("1st order stat= {:?}", d_selection(&mut arr, 1));
    println!("2nd order stat= {:?}", d_selection(&mut arr, 2));
    println!("3rd order stat= {:?}", d_selection(&mut arr, 3));

    println!("MergeSort Immut: {:?}", mergesort(&v));
    println!("MergeSort Mut  : ({}, {:?})", mergesort_mut(&mut v, merge_mut), v);
    quick_sort(&mut v);
    println!("Quick Sort     : {:?}", v);

    let mut arr: Vec<MyType> = list.iter().copied().collect();
    arr.count_sort();
    println!("Count Sort     : {:?}", arr);

    let bt : BinaryTree<MyType> = list.iter().copied().collect();
    println!("bTree Sort     : {:?}", bt.iter().collect::<Vec<_>>());
    println!("List Sort      : {:?}", list.sort_with_count() );

}
