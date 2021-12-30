use std::iter::from_fn;
use mergeshort::dnc::*;
use mergeshort::trees::*;
use mergeshort::linkedlists::*;

fn main() {

    let list: List<i8> = from_fn(|| { Some(rand::random::<i8>()) })
        .take(16)
        .collect();
    let v: Vec<i8> = list.iter().map(|x| *x).collect();
    let bt : BinaryTree<i8> = list.iter().map(|x| *x).collect();

    println!("Merge Sort: {:?}", merge_sort(&v));
    println!("bTree Sort: {:?}", bt.iter().collect::<Vec<_>>());
    println!("List      : {:?}", list.iter().collect::<Vec<_>>() );

}
