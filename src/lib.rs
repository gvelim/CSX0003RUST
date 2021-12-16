

/// Divide and Conquere algorithms
pub mod  divnconq {

    /// Sort function based on the merge sort algorithm
    pub fn merge_sort(v: &mut Vec<i32>) -> Vec<i32> {

        /// Merge subroutine
        /// Join to slices while in the right Order
        fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
            enum Condition {
                LeftExit,
                RightExit,
            }

            let l_len = left.len() - 1;
            let r_len = right.len() - 1;
            let mut output: Vec<i32> = Vec::with_capacity(l_len + r_len + 2);

            let mut r = 0;
            let mut l = 0;

            // go into a loop until one of the slices goes off bounds
            // indicate which slice caused the exit for use by the
            // following append instruction
            match loop {
                if right[r] > left[l] {
                    output.push(left[l]);
                    l += 1;
                    if l > l_len {
                        break Condition::LeftExit;
                    }
                } else {
                    output.push(right[r]);
                    r += 1;
                    if r > r_len {
                        break Condition::RightExit;
                    }
                }
            } {
                // append the remaining slice on the output vector
                // based on the loop exit condition
                Condition::LeftExit => output.extend_from_slice(&right[r..]),
                Condition::RightExit => output.extend_from_slice(&left[l..]),
            }

            print!("merge: {},{:?} <> {},{:?},", r_len, right, l_len, left);
            println!("=> {:?},", output);
            output
        }

        let len = v.len();
        println!("Input: ({}){:?} =>", len, v);
        match len {
            0 => panic!("cannot sort an emptry vector"),
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
                let (left, right) = v.split_at(len >> 1);
                let l = merge_sort(&mut Vec::from(left));
                let r = merge_sort(&mut Vec::from(right));

                // return a vector of the merged but ordered slices
                merge(&l, &r)
            }
        }
    }
}