use std::iter::from_fn;
use csx3::sort::*;
use csx3::trees::*;
use csx3::linkedlists::*;

type MyType = i8;

fn main() {

    let list: List<MyType> = from_fn(|| { Some(rand::random::<MyType>()) })
        .take(16)
        .collect();
    let mut v: Vec<MyType> = list.iter().map(|x| *x).collect();
    let bt : BinaryTree<MyType> = list.iter().map(|x| *x).collect();

    println!("Merge Sort: {:?}", merge_sort(&mut v));
    quick_sort(&mut v);
    println!("Quick Sort: {:?}", v);
    println!("bTree Sort: {:?}", bt.iter().collect::<Vec<_>>());
    println!("List      : {:?}", list.iter().collect::<Vec<_>>() );

}
