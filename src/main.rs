

fn main() {

    let mut v = vec![91,82,73,64,5,09,18,2,73,6,45,0];

    println!("array: {:?}", merge_sort(&mut v));
}
/// Sort function based on the merge sort algorithm
fn merge_sort(v:&mut Vec<i32>) -> Vec<i32> {

    /// Merge subroutine
    /// Join to slices while in the right Order
    fn merge(left:&Vec<i32>, right:&Vec<i32>) -> Vec<i32> {

        enum Condition {
            LeftExit,
            RightExit,
        }

        let llen = left.len()-1;
        let rlen = right.len()-1;
        let mut output: Vec<i32> = Vec::new();

        let mut r = 0;
        let mut l = 0;

        // go into a loop until one of the slices goes off bounds
        // indicate which slice caused the exit for use by the
        // following append instruction
        let exit_cond = loop {
            if right[r] > left[l] {
                //println!("({},{}) push: {}",rv,lv,lv);
                output.push(left[l]);
                l += 1;
                if l > llen {
                    break Condition::LeftExit ;
                }
            } else {
                //println!("({},{}) push: {}",rv,lv,rv);
                output.push(right[r]);
                r += 1;
                if r > rlen {
                    break Condition::RightExit;
                }
            }
        };

        // append the remaining slice on the output vector
        match exit_cond {
            Condition::LeftExit => output.extend_from_slice(&right[r..]),
            Condition::RightExit => output.extend_from_slice(&left[l..]),
        }

        print!("output: {},{:?} <> {},{:?},",rlen,right,llen,left);
        println!("=> {:?},",output);
        output
    }

    let len = v.len();
    println!("Input: ({}){:?} =>", len, v);
    match len {
        // unity slice, just return it
        1 => v.clone(),
        // sort the binary slice and exit
        2 => {
            if v[0] > v[1] {
                v.swap(0, 1);
            }
            v.clone()
        },
        // if slice length longer than 2 then split recursively
        _ => {
            let (left, right) = v.split_at(len >> 1 );
            let l = merge_sort(&mut Vec::from(left) );
            let r =  merge_sort(&mut Vec::from(right) );

            // return a vector of the merged but ordered slices
            merge(&l,&r)
        }
    }
}
