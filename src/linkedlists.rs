
/// I could have done it differently
/// but I wanted to see how far I could go with this simple enum structure
#[derive(Debug, PartialEq)]
pub enum List<T>
    where T: Copy + Clone + PartialEq {
    Empty,
    NotEmpty(T, Box<List<T>>),
}

/// List related methods
impl<T> List<T>
    where T: Copy + Clone + PartialEq {

    /// Construct an empty list
    pub fn new() -> List<T> {
        List::Empty
    }
    /// Push to the end of the list. inefficiently for now
    pub fn push(&mut self, item: T) {
        match self {
            List::Empty => {
                *self = List::NotEmpty(
                    item,
                    Box::new(List::Empty)
                );
            }
            List::NotEmpty(_, next) => {
                next.push(item);
            }
        }
    }
    /// Pop from the end of the list, inefficiently for now
    pub fn pop(&mut self) -> Option<T> {
        match self {
            List::Empty => None,
            List::NotEmpty(val, next) => {
                if List::Empty == **next {
                    let item = *val;
                    *self = List::Empty;
                    Some(item)
                }
                else {
                    next.pop()
                }
            }
        }
    }
    // pub fn iter(&self) -> ListIter<T> {
    //     match self {
    //         List::Empty => ListIter { cursor: List::Empty },
    //         List::NotEmpty(val, next) => {
    //             ListIter {
    //                 cursor: List::NotEmpty(
    //                     val,
    //                     next,
    //                 )
    //             }
    //         }
    //     }
    // }
}

/// List provides a "non-consuming" iterator... against the norm
/// '''
/// for i in &list
/// '''
impl<T> IntoIterator for List<T>
    where T: Copy + Clone + PartialEq {

    type Item = T;
    type IntoIter = ListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            List::Empty => ListIter { cursor: List::Empty },
            List::NotEmpty(val, next) => {
                ListIter {
                    cursor: List::NotEmpty(
                        val,
                        next,
                    )
                }
            }
        }
    }
}

/// A List can be constructed from other collections
/// '''
/// let mut l : List<i32> = v.into_iter().collect();
/// '''
impl<T> FromIterator<T> for List<T>
    where T: Copy + Clone + PartialEq {

    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut list = List::Empty;
        for item in iter {
            list.push(item);
        }
        list
    }
}

pub struct ListIterByRef<'a, T>
    where T: Copy + Clone + PartialEq {
    cursor: &'a List<T>,
}


/// A List Iterator
/// Sets up a cursor that references current node
pub struct ListIter<T>
    where T: Copy + Clone + PartialEq {
    cursor: List<T>,
}

impl<T> Iterator for ListIter<T>
    where T: Copy + Clone + PartialEq {

    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            List::Empty => None,
            List::NotEmpty(value, ref mut next) => {

                // WE CANNOT MOVE A BOXED VALUE, hence we need to resort to this trick
                // create a tmp memory space,
                let mut tmp = Box::new( List::Empty );
                //  swap the values and
                std::mem::swap( next, &mut tmp);
                // assign the tmp to self
                self.cursor = *tmp;
                Some(value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list,
                   List::NotEmpty(1,Box::new(
                       List::NotEmpty(2, Box::new(
                           List::NotEmpty(3, Box::new(
                               List::Empty
                           ))
                       ))
                   ))
        )
    }
    #[test]
    fn test_pop() {
        let mut l = List::new();
        l.push(1);
        l.push(2);

        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
        assert_eq!(l.pop(), None);
    }
    #[test]
    fn test_list_into_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);

        let mut iter = l.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None)
    }
    #[test]
    fn test_from_iter() {
        let v = vec![1,2,3];

        let mut l : List<i32> = v.into_iter().collect();

        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
    }
}