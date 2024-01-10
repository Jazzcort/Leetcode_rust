use std::rc::Rc;
use std::cell::RefCell;
use crate::util::tree_node::{Tree, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {

        fn leaves(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) -> i32 {
            match node {
                Some(x) => {
                    let cur = x.borrow();

                    let left = leaves(&cur.left, res);
                    let right = leaves(&cur.right, res);

                    if left == 0 && right == 0 {
                        res.push(cur.val);
                    }
                    1

                },
                None => {0}
            }
        }

        let mut arr1: Vec<i32> = vec![];
        let mut arr2: Vec<i32> = vec![];

        leaves(&root1,  &mut arr1);
        leaves(&root2, &mut arr2);

        if arr1.len() != arr2.len() {
            return false
        } else {
            for i in 0..arr1.len() {
                if arr1[i] != arr2[i] {
                    return false
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case1() {
        let tree1 = Tree::from(vec![Some(3),Some(5),Some(1),Some(6),Some(2),Some(9),Some(8),None,None,Some(7),Some(4)]);
        let tree2 = Tree::from(vec![Some(3),Some(5),Some(1),Some(6),Some(7),Some(4),Some(2),None,None,None,None,None,None,Some(9),Some(8)]);

        assert_eq!(true, Solution::leaf_similar(tree1.get_head(), tree2.get_head()));
    }

    #[test]
    fn test_case2() {
        let tree1 = Tree::from(vec![Some(1), Some(2), Some(3)]);
        let tree2 = Tree::from(vec![Some(1), Some(3), Some(2)]);

        assert_eq!(false, Solution::leaf_similar(tree1.get_head(), tree2.get_head()));
    }

    // Solution::leaf_similar();
}