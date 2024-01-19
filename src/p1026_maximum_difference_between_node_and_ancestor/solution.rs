use std::rc::Rc;
use std::cell::RefCell;
use crate::util::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> (i32, i32) {
            match node {
                Some(x) => {
                    let (left_min, left_max) = helper(&x.borrow().left, res);
                    let (right_min, right_max) = helper(&x.borrow().right, res);

                    let cur_min = left_min.min(right_min.min(x.borrow().val));
                    let cur_max = left_max.max(right_max.max(x.borrow().val));

                    let mut tmp = cur_min.abs_diff(x.borrow().val).max(cur_max.abs_diff(x.borrow().val)) as i32;

                    *res = *res.max(&mut tmp);

                    (cur_min, cur_max)
                },
                None => (i32::MAX, i32::MIN)
            }
        }

        helper(&root, &mut res);

        res
    }
}