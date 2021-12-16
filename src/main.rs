

fn main() {

    let mut v = vec![91,82,73,64,5,09,18,2,73,6,45,0];

    println!("array: {:?}", merge_sort(&mut v));
}

fn merge_sort(v:&mut Vec<i32>) -> Vec<i32> {

    fn merge(left:&Vec<i32>, right:&Vec<i32>) -> Vec<i32> {

        let llen = left.len()-1;
        let rlen = right.len()-1;
        let mut output: Vec<i32> = Vec::new();

        let mut r = 0;
        let mut l = 0;
        let mut rv = right[r];
        let mut lv = left[l];

        let cond = loop {
            if rv > lv {
                //println!("({},{}) push: {}",rv,lv,lv);
                output.push(lv);
                l += 1;
                if l <= llen {
                    lv = left[l];
                } else {
                    break 0 ;
                }
            } else {
                //println!("({},{}) push: {}",rv,lv,rv);
                output.push(rv);
                r += 1;
                if r <= rlen {
                    rv = right[r];
                } else {
                    break 1;
                }
            }
        };

        match cond {
            0 => output.extend_from_slice(&right[r..]),
            1 => output.extend_from_slice(&left[l..]),
            _ => {}
        }


        print!("output: {},{:?} <> {},{:?},",rlen,right,llen,left);
        println!("=> {:?},",output);
        output
    }

    let len = v.len();
    println!("Input: ({}){:?} =>", len, v);
    match len {
        1 => v.clone(),
        2 => {
            if v[0] > v[1] {
                v.swap(0, 1);
            }
            v.clone()
        },
        _ => {
            let (left, right) = v.split_at(len >> 1 );
            let l = merge_sort(&mut Vec::from(left) );
            let r =  merge_sort(&mut Vec::from(right) );

            merge(&l,&r)
        }
    }
}
