// 0000700. Search in a Binary Search Tree
// https://leetcode.com/problems/search-in-a-binary-search-tree/description/
// You are given the root of a binary search tree (BST) and an integer val.
//
// Find the node in the BST that the node's value equals val and return the subtree rooted with that node. If such a node does not exist, return null.
//
// Example 1:
//
// Input: root = [4,2,7,1,3], val = 2
// Output: [2,1,3]
//
// Example 2:
//
// Input: root = [4,2,7,1,3], val = 5
// Output: []
//
//
// Constraints:
//
//     The number of nodes in the tree is in the range [1, 5000].
//     1 <= Node.val <= 107
//     root is a binary search tree.
//     1 <= val <= 107
//

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// ---
pub fn search_bst(
    mut root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    while let Some(node) = root {
        match node.borrow().val.cmp(&val) {
            std::cmp::Ordering::Less => root = node.borrow().right.clone(),
            std::cmp::Ordering::Equal => return Some(node.clone()),
            std::cmp::Ordering::Greater => root = node.borrow().left.clone(),
        }
    }
    None
}

pub fn search_bst_recur(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        match node.borrow().val.cmp(&val) {
            std::cmp::Ordering::Less => return search_bst_recur(node.borrow().right.clone(), val),
            std::cmp::Ordering::Equal => return Some(node.clone()),
            std::cmp::Ordering::Greater => {
                return search_bst_recur(node.borrow().left.clone(), val)
            }
        }
    }
    None
}
// ---

pub fn testcase() {
    let node_3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let node_1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node_7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let node_2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let node_4 = Rc::new(RefCell::new(TreeNode::new(4)));

    node_2.borrow_mut().left = Some(Rc::clone(&node_1));
    node_2.borrow_mut().right = Some(Rc::clone(&node_3));
    node_4.borrow_mut().left = Some(Rc::clone(&node_2));
    node_4.borrow_mut().right = Some(Rc::clone(&node_7));

    let res = search_bst(Some(node_2), 2);
    eprintln!("{} {:?}", module_path!(), res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let node_3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node_1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node_7 = Rc::new(RefCell::new(TreeNode::new(7)));
        let node_2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node_4 = Rc::new(RefCell::new(TreeNode::new(4)));

        node_2.borrow_mut().left = Some(Rc::clone(&node_1));
        node_2.borrow_mut().right = Some(Rc::clone(&node_3));
        node_4.borrow_mut().left = Some(Rc::clone(&node_2));
        node_4.borrow_mut().right = Some(Rc::clone(&node_7));

        let expected = Some(node_2.clone());
        let res = search_bst(Some(node_4.clone()), 2);
        assert_eq!(res, expected);

        let expected = Some(node_2);
        let res = search_bst_recur(Some(node_4), 2);
        assert_eq!(res, expected);
    }
}
