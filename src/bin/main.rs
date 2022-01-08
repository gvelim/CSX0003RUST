use std::iter::from_fn;
use csx3::sort::*;
use csx3::trees::*;
use csx3::linkedlists::*;

fn main() {

    let list: List<i8> = from_fn(|| { Some(rand::random::<i8>()) })
        .take(8)
        .collect();
    let mut v: Vec<i8> = list.iter().map(|x| *x).collect();
    let bt : BinaryTree<i8> = list.iter().map(|x| *x).collect();

    println!("Merge Sort: {:?}", merge_sort(&v));
    quick_sort(&mut v);
    println!("Quick Sort: {:?}", v);
    println!("bTree Sort: {:?}", bt.iter().collect::<Vec<_>>());
    println!("List      : {:?}", list.iter().collect::<Vec<_>>() );
    println!("List      : {:?}", list.sort_with_count().1 );
}
