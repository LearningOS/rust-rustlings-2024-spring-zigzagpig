/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        // if self.root.is_none() {
        //     self.root = Some(Box::new(TreeNode::new(value)));
        // } else {
        //     self.root.as_mut().unwrap().insert(value);
        // }
        match self.root {
            Some(_) => self.root.as_mut().unwrap().insert(value),
            None => self.root = Some(Box::new(TreeNode::new(value))),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut p = &self.root;
        while let Some(node) = p {
            match value.cmp(&node.value) {
                Ordering::Less => p = &node.left,
                Ordering::Equal => return true,
                Ordering::Greater => p = &node.right,
            }
        }
        false
        // //TODO
        // if self.root.is_none() {
        //     return false;
        // }

        // let mut node = self.root.as_ref().unwrap();
        // loop {
        //     if node.value == value {
        //         return true;
        //     } else if node.value > value {
        //         if node.left.is_some() {
        //             node = node.left.as_ref().unwrap();
        //         } else {
        //             return false;
        //         }
        //     } else if node.value < value {
        //         if node.right.is_some() {
        //             node = node.right.as_ref().unwrap();
        //         } else {
        //             return false;
        //         }
        //     }
        // }
        // true
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        // if self.
        match value.cmp(&self.value) {
            Ordering::Less => match self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut node) => node.insert(value),
            },
            Ordering::Equal => (),
            Ordering::Greater => match self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut node) => node.insert(value),
            },
        }

        // if value < self.value {
        //     if self.left.is_none() {
        //         self.left = Some(Box::new(TreeNode::new(value)));
        //     } else {
        //         self.left.as_mut().unwrap().insert(value);
        //         // TreeNode::insert(&self.left.unwrap().as_mut(), value);
        //     }
        // } else if value > self.value {
        //     if self.right.is_none() {
        //         self.right = Some(Box::new(TreeNode::new(value)));
        //     } else {
        //         self.right.as_mut().unwrap().insert(value);
        //         // TreeNode::insert(&self.right.unwrap().as_mut(), value);
        //     }
        // } else {
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
