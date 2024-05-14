// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use crate::util::tree_node::TreeNode;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        fn helper(ptr: &Rc<RefCell<TreeNode>>, pre: i32, total: &mut i32) {

            let num = ptr.borrow().val + pre * 10;
            if ptr.borrow().left == None && ptr.borrow().right == None {
                *total += num;
            } else if ptr.borrow().left == None {
                helper(ptr.borrow().right.as_ref().unwrap(), num, total);
            } else if ptr.borrow().right == None {
                helper(ptr.borrow().left.as_ref().unwrap(), num, total);
            } else {
                helper(ptr.borrow().left.as_ref().unwrap(), num, total);
                helper(ptr.borrow().right.as_ref().unwrap(), num, total);
            }
            
        }

        let mut res = 0;

        if root != None {
            helper(&root.unwrap(), 0, &mut res);
        } 

        res
    }
}