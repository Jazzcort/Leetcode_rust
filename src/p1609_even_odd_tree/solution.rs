use std::rc::Rc;
use std::cell::RefCell;
use crate::util::tree_node::TreeNode;
use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut que: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        que.push_back(root.unwrap());
        let mut is_even_ind = true;

        while !que.is_empty() {

            if is_even_ind {
                let mut pre = 0;
                for node_ref in que.iter() {
                    let node = node_ref.borrow();
                    if node.val % 2 == 0  || node.val <= pre {
                        return false;
                    }

                    pre = node.val;
                }
            } else {
                let mut pre = 1_000_000_0;
                for node_ref in que.iter() {
                    let node = node_ref.borrow();
                    if node.val % 2 != 0 || node.val >= pre {
                        return false;
                    }

                    pre = node.val;
                }
            }

            is_even_ind = !is_even_ind;

            let n = que.len();

            for _ in 0..n {
                let cur = que.pop_front().unwrap();
                let node = cur.borrow();

                match &node.left {
                    Some(x) => {
                        que.push_back(x.clone());
                    }
                    None => {}
                }

                match &node.right {
                    Some(x) => {
                        que.push_back(x.clone());
                    }
                    None => {}
                }
            }
        }

        true
    }
}