use std::rc::Rc;
use std::cell::RefCell;
use crate::util::tree_node::TreeNode;
pub struct Solution;
impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {

        fn helper(node: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(t_node) = node {
                let mut handle = t_node.borrow_mut();
                handle.left = helper(handle.left.clone(), target);
                handle.right = helper(handle.right.clone(), target);
                match (handle.left.clone(), handle.right.clone()) {
                    (None, None) => {
                        if handle.val == target {
                            None
                        } else {
                            Some(t_node.clone())
                        }
                    }
                    _ => {Some(t_node.clone())}
                }
            } else {
                node
            }
        }

        helper(root, target)
    }
}