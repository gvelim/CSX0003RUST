use std::iter::from_fn;
use mergeshort::dnc::*;
use mergeshort::trees::*;
use mergeshort::linkedlists::*;

fn main() {

    let list: List<i32> = from_fn(|| { Some(rand::random::<i8>() as i32) })
        .take(25)
        .collect();
    let v: Vec<i32> = list.iter().map(|x| *x).collect();
    let bt : BinaryTree<i32> = list.iter().map(|x| *x as i32).collect();

    println!("Merge Sort: {:?}", merge_sort(&v));
    println!("bTree Sort: {:?}", bt.iter().collect::<Vec<&i32>>());
    println!("List      : {:?}", list.iter().collect::<Vec<_>>() );

}
