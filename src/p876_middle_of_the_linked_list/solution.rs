use crate::util::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut slow_ptr, mut fast_ptr) = (head.clone(), head.clone());

        while match fast_ptr {
            Some(_) => true,
            None => false,
        } && match fast_ptr.as_ref().unwrap().next {
            Some(_) => true,
            None => false,
        } {
            slow_ptr = slow_ptr.unwrap().next;
            fast_ptr = fast_ptr.unwrap().next.unwrap().next;
        }

        slow_ptr
    }
}
