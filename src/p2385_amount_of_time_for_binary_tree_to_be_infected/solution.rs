use std::cell::RefCell;
use std::rc::Rc;
use crate::util::tree_node::TreeNode;

pub struct Solution {}

#[allow(non_snake_case)]
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {

        fn find_target(node: &Option<Rc<RefCell<TreeNode>>>, tar: i32) -> (i32, i32) {

            match node {
                Some(x) => {

                    let (l_dep, level_L) = find_target(&x.borrow().left, tar);
                    let (r_dep, level_R) = find_target(&x.borrow().right, tar);

                    if x.borrow().val == tar {
                        return (l_dep.max(r_dep) + 1, 1);
                    }

                    if level_L > 0 {
                        (l_dep.max(r_dep + 1 + level_L), level_L + 1)
                    } else if level_R > 0 {
                        (r_dep.max(l_dep + 1 + level_R), level_R + 1)
                    } else {
                        (l_dep.max(r_dep) + 1, 0)
                    }
                },
                None => {(0, 0)}
            }
        }

        find_target(&root, start).0 - 1

    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::util::tree_node::Tree;

    #[test]
    fn p2385_case1() {
        let tree1 = Tree::from(vec![Some(1),Some(5),Some(3),None,Some(4),Some(10),Some(6),Some(9),Some(2)]);

        assert_eq!(4, Solution::amount_of_time(tree1.get_head(), 3));
    }

    #[test]
    fn p2385_case2() {
        let tree1 = Tree::from(vec![Some(1),Some(5),Some(3),None,Some(4),Some(10),Some(6),Some(9),Some(2)]);

        assert_eq!(3, Solution::amount_of_time(tree1.get_head(), 1));
        
    }
}