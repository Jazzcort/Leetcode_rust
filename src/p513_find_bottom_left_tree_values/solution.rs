use std::rc::Rc;
use std::cell::RefCell;
use crate::util::tree_node::TreeNode;
use std::collections::VecDeque;


pub struct Solution {}

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut que: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        let mut res = 0;
        
        que.push_back(root.unwrap());

        
        while !que.is_empty() {
            let n = que.len();

            res = que.front().unwrap().borrow().val;

            for _ in 0..n {
                let cur = que.pop_front().unwrap();

                let node = cur.borrow();

                match &node.left {
                    Some(x) => {
                        que.push_back(x.clone())
                    }
                    None => {}
                }

                match &node.right {
                    Some(x) => {
                        que.push_back(x.clone())
                    }
                    None => {}
                }

            }
            
        }

        res
    }
}