use mergeshort::divnconq::merge_sort;

fn main() {

    let mut v = vec![91,82,73,64,5,09,18,2,73,6,45,90,18,27,364,50,91,82,7,364,5];

    println!("array: {:?}", merge_sort(&mut v));
}
