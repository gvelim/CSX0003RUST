use std::iter::from_fn;
use csx3::sort::*;
use csx3::trees::*;
use csx3::linkedlists::*;
use std::env::args;
use std::str::FromStr;

type MyType = i8;

fn main() {

    let n = args().into_iter()
        .skip(1)
        .map(|x| usize::from_str(&x) )
        .next()
        .expect("Use: main <n>\nn : number of elements to be sorted")
        .unwrap_or(16);

    let list: List<MyType> = from_fn(|| { Some(rand::random::<MyType>()) })
        .take(n)
        .collect();
    let mut v: Vec<MyType> = list.iter().map(|x| *x).collect();
    let bt : BinaryTree<MyType> = list.iter().map(|x| *x).collect();

    println!("List      : {:?}", v );
    println!("1st order min = {}", rand_selection(v.as_mut_slice(), 1));
    println!("2nd order min = {}", rand_selection(v.as_mut_slice(), 2));
    println!("3rd order min = {}", rand_selection(v.as_mut_slice(), 3));
    println!("List      : {:?}", v );
    merge_sort(&mut v);
    println!("Merge Sort: {:?}", v);
    quick_sort(&mut v);
    println!("Quick Sort: {:?}", v);
    println!("bTree Sort: {:?}", bt.iter().collect::<Vec<_>>());
    println!("List Sort : {:?}", list.sort_with_count() );

}
