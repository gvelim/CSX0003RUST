use crate::linkedlists::List::NotEmpty;

#[derive(Debug, PartialEq)]
enum List<T> {
    Empty,
    NotEmpty(T, Box<List<T>>),
}

impl<T> List<T> where T: Copy + Clone {
    fn new() -> List<T> {
        List::Empty
    }

    fn push(&mut self, item: T) {
        match self {
            List::Empty => {
                *self = List::NotEmpty(
                    item,
                    Box::new(List::Empty)
                );
            }
            NotEmpty(val, next) => {
                next.push(*val);
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
}