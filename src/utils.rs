use std::ops::{Index, IndexMut};
use std::slice::IterMut;

#[derive(Debug)]
struct VirtualSlice<T> {
    vv: Vec<*mut T>
}

impl<T> VirtualSlice<T> {
    fn new(s1: &mut[T]) -> VirtualSlice<T> {
        let mut vec = Vec::new();
        s1.iter_mut()
            .for_each(|item| {
                vec.push(item as *mut T);
            });
        VirtualSlice {
            vv: vec,
        }
    }
    fn len(&self) -> usize {
        self.vv.len()
    }
    fn chain(&mut self, s1: &mut [T]) {
         s1.iter_mut()
            .for_each(|item| {
                self.vv.push(item as *mut T);
            });
    }
    fn iter_mut(&mut self) -> IterMut<*mut T> {
        self.vv.iter_mut()
    }
    fn swap(&self, a: usize, b:usize) {
        unsafe {
            std::ptr::swap::<T>(
                self.vv[a],
                self.vv[b]
            )
        }
    }
}

impl<T> Index<usize> for VirtualSlice<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            & *self.vv[index]
        }
    }
}

impl<T> IndexMut<usize> for VirtualSlice<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            &mut *self.vv[index]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_virt_slice_new_and_iter() {
        let s1 = &mut [1, 3, 5, 7, 9];
        let _s3 = &mut [0, 0, 0];
        let s2 = &mut [2, 4, 6, 8 , 10];

        let mut v = VirtualSlice::new(s1);
        v.chain(s2);
        println!("{:?}", v);
        v.iter_mut()
            .for_each(|ptr| {
                unsafe {
                    **ptr = 12;
                }
            });
        v[0] = 11;
        v[s1.len()] = 9;
        v.swap(0, s1.len());
        println!("{:?}{:?}", s1, s2);
        assert_eq!(s1, &mut [9, 12, 12, 12, 12]);
        assert_eq!(s2, &mut [11, 12, 12, 12, 12]);
    }
}