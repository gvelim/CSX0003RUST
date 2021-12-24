
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
    /// Push to the end of the list; popped first
    pub fn push_last(&mut self, item: T) {
        match self {
            List::Empty => {
                *self = List::NotEmpty(
                    item,
                    Box::new(List::Empty)
                );
            }
            List::NotEmpty(_, next) => {
                next.push_last(item);
            }
        }
    }
    /// Push to the head of the list; popped last
    pub fn push_first(&mut self, item: T) {
        if let List::Empty = self {
            self.push_last(item);
        } else {
            // create a memory space to hold current head
            let mut old_head = List::Empty;
            // move current head to new memory space
            // while we have self ready to hold the new head
            std::mem::swap(self, &mut old_head);
            // create new head that points to old one
            *self = List::NotEmpty(
                item,
                Box::new(old_head),
            );
        }
    }
    /// Pop from the end of the list
    pub fn pop_last(&mut self) -> Option<T> {
        match self {
            List::Empty => None,
            List::NotEmpty(val, next) => {
                if List::Empty == **next {
                    // we arrived at the last one, since next == null
                    // move value out
                    let item = *val;
                    // make box from previous node to hold empty
                    *self = List::Empty;
                    Some(item)
                }
                else {
                    // not the last, move to the next
                    next.pop_last()
                }
            }
        }
    }
    /// Pop for the head of the list
    pub fn pop_first(&mut self) -> Option<T> {
        match self {
            List::Empty => None,
            List::NotEmpty(item, next) => {
                // move ownership
                let val = *item;
                // create a new empty head
                let mut new_head = Box::new(List::Empty);
                // move next node into the new head
                std::mem::swap( next, &mut new_head);
                // self take ownership of new head
                *self = *new_head;
                Some(val)
            }
        }
    }
    /// Iterate by
    pub fn iter(&self) -> ListIterByRef<T> {
        match self {
            List::Empty =>
                ListIterByRef {
                    cursor: &List::Empty,
                },
            List::NotEmpty(_, _) =>
                ListIterByRef {
                    cursor: self
                },
        }
    }
}

impl<T> Iterator for List<T>
    where T: Copy + Clone + PartialEq {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_first()
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
        iter.into_iter()
            .for_each( |item| list.push_first(item));
        list
    }
}

/// List by reference iterator
pub struct ListIterByRef<'a, T>
    where T: Copy + Clone + PartialEq {
    cursor: &'a List<T>,
}

impl<'a, T> Iterator for ListIterByRef<'a, T>
    where T: Copy + Clone + PartialEq {

    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            List::Empty => None,
            List::NotEmpty(value, next) => {
                self.cursor = next;
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

        list.push_first(1);
        list.push_first(2);
        list.push_first(3);

        assert_eq!(list,
                   List::NotEmpty(3,Box::new(
                       List::NotEmpty(2, Box::new(
                           List::NotEmpty(1, Box::new(
                               List::Empty
                           ))
                       ))
                   ))
        )
    }
    #[test]
    fn test_push_last() {
        let mut list = List::new();

        list.push_last(1);
        list.push_last(2);
        list.push_last(3);

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
        l.push_last(1);
        l.push_last(2);

        assert_eq!(l.pop_first(), Some(1));
        assert_eq!(l.pop_first(), Some(2));
        assert_eq!(l.pop_first(), None);
        assert_eq!(l.pop_first(), None);
    }
    #[test]
    fn test_pop_last() {
        let mut l = List::new();
        l.push_last(1);
        l.push_last(2);

        assert_eq!(l.pop_last(), Some(2));
        assert_eq!(l.pop_last(), Some(1));
        assert_eq!(l.pop_last(), None);
        assert_eq!(l.pop_last(), None);
    }
    #[test]
    fn test_iter() {
        let mut l = List::new();
        l.push_last(1);
        l.push_last(2);

        let mut iter = l.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);

        let m: List<i32> = List::new();
        assert_eq!(m.iter().cursor, &List::Empty);
    }
    #[test]
    fn test_into_iter() {
        let mut l = List::new();
        l.push_last(1);
        l.push_last(2);

        let mut iter = l.into_iter();

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        let l:List<i32> = List::new();
        assert_eq!(l.into_iter(), List::Empty);
    }
    #[test]
    fn test_from_iter() {
        let v = vec![1,2,3];

        let mut l : List<i32> = v.into_iter().collect();

        assert_eq!(l.pop_last(), Some(3));
        assert_eq!(l.pop_last(), Some(2));
        assert_eq!(l.pop_last(), Some(1));
        assert_eq!(l.pop_last(), None);
    }
}