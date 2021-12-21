use mergeshort::{divnconq::merge_sort, trees::*};

fn main() {

    let mut v = vec![91,82,73,64,5,09,18,2,73,6,45,90,18,27,364,50,91,82,7,364,5];
    let mut bt = BinaryTree::new( 50);

    v.clone().into_iter().for_each( |x| bt.add(x));

    println!("Merge Sort: {:?}", merge_sort(&mut v));

    println!("bTree: {:?}", bt.iter().collect::<Vec<&i32>>());
}
