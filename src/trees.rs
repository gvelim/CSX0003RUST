
#[derive(Debug)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

#[derive(Debug)]
pub struct TreeNode<T> {
    val: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> BinaryTree<T>
    where T: Copy + Clone + Ord
{
    pub fn new(val: T) -> BinaryTree<T> {
        BinaryTree::NonEmpty(
            Box::new( TreeNode {
                val,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            }))
    }
    pub fn add(&mut self, val: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(
                    Box::new( TreeNode {
                        val,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty,
                    }));
            }
            BinaryTree::NonEmpty(node) => {
                if node.val < val {
                    node.left.add(val)
                } else {
                    node.right.add(val)
                }
            }
        }
    }
    pub fn iter(&self) -> TreeIter<T> {
        let mut titer = TreeIter { stack: Vec::new(), };
        titer.push_left_tree(self);
        titer
    }
}

pub struct TreeIter<'a, T> {
    stack: Vec<&'a TreeNode<T>>,
}

impl<'a, T> TreeIter<'a, T> {
    fn push_left_tree(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::NonEmpty(node) = tree {
            self.stack.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T>
    where T: Ord + Copy + Clone
{
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        self.push_left_tree(&node.right);
        Some(&node.val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_iter() {
        let mut a = BinaryTree::new(41);
        a.add(50);
        a.add(40);
        a.add(60);
        a.add(45);

        assert_eq!(
            a.iter()
                .map( |x| *x )
                .collect::<Vec<i32>>(),
            vec![60,50,45,41,40]
        );
    }
}
