use crate::util::tree_node::TreeNode;

pub struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, low: &i32, high: &i32) -> i32 {
            match node {
                Some(x) => {
                    let tmp = x.borrow();
                    let left = helper(&tmp.left, low, high);
                    let right = helper(&tmp.right, low, high);
                    if tmp.val <= *high && tmp.val >= *low {
                        tmp.val + left + right
                    } else {
                        left + right
                    }
                },
                None => {0}
            }
        }
        
        helper(&root, &low, &high)
    }
}