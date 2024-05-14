use crate::ListNode;
pub struct Solution;
impl Solution {
    pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn helper(node: Option<&mut Box<ListNode>>) -> i32 {
            if let Some(x) = node {
                let mut carr = helper(x.next.as_mut());
                carr += x.val * 2;

                x.val = carr % 10;

                carr / 10

            } else {
                0
            }
        }

        let res = helper(head.as_mut());

        if res != 0 {
            let mut new_head = ListNode::new(res);
            new_head.next = head;
            Some(Box::new(new_head))
        } else {
            head
        }
    }
}