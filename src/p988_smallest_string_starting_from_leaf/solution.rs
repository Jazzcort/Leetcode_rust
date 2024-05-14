use std::rc::Rc;
use std::cell::RefCell;
use crate::util::tree_node::TreeNode;
pub struct Solution;
impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut arr = vec![];
        
        fn helper(node: &Rc<RefCell<TreeNode>>, pre: &str, arr: &mut Vec<String>) {
            let c = (node.borrow().val as u8 + 97) as char;
            let cur = c.to_string() + pre;

            if node.borrow().left == None && node.borrow().right == None {
                arr.push(cur);
            } else {
                if node.borrow().left != None {
                    helper(&node.borrow().left.clone().unwrap(), cur.as_str(), arr)
                }

                if node.borrow().right != None {
                    helper(&node.borrow().right.clone().unwrap(), cur.as_str(), arr)
                }
            }
        }

        helper(&root.unwrap(), "", &mut arr);
        arr.sort_unstable();
        
        arr[0].clone()
    }
}