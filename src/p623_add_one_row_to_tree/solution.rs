pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::util::tree_node::TreeNode;

impl Solution {
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, mut depth: i32) -> Option<Rc<RefCell<TreeNode>>> {

        if depth == 1 {
            let mut new_root = TreeNode::new(val);
            new_root.left = root;
            return Some(Rc::new(RefCell::new(new_root)))
        }

        let mut que = VecDeque::from(vec![root.clone().unwrap()]);

        while depth > 2 {
            let n = que.len();

            for _ in 0..n {
                let cur = que.pop_front().unwrap();
                let left = cur.borrow().left.clone();
                let right = cur.borrow().right.clone();
                if let Some(node) = left {
                    que.push_back(node);
                }
                if let Some(node) = right {
                    que.push_back(node);
                }

            }
            depth -= 1;
        }

        while !que.is_empty() {
            let cur = que.pop_front().unwrap();
            let left = cur.borrow().left.clone();
            let right = cur.borrow().right.clone();

            let mut new_left = TreeNode::new(val);
            new_left.left = left;

            let mut new_right = TreeNode::new(val);
            new_right.right = right;

            cur.borrow_mut().left = Some(Rc::new(RefCell::new(new_left)));
            cur.borrow_mut().right = Some(Rc::new(RefCell::new(new_right)));
        }
        
        root
    }
}