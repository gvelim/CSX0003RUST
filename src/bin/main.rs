use mergeshort::{dnc::merge_sort, trees::*};

fn main() {

    let v = vec![91,82,73,64,5,09,18,2,73,6,45,90,18,27,364,50,91,82,7,364,5];
    let bt : BinaryTree<i32> = v.iter().map(|x| *x ).collect();

    println!("Merge Sort: {:?}", merge_sort(&v));
    println!("bTree Sort: {:?}", bt.iter().collect::<Vec<&i32>>());
}
