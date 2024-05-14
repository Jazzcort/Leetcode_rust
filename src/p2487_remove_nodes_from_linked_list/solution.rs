use crate::util::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut stck: Vec<Box<ListNode>> = vec![];

        while let Some(mut node) = head {
            head = node.next;

            while !stck.is_empty() && stck.last().unwrap().val < node.val {
                let _ = stck.pop().unwrap();
            }

            node.next = None;
            stck.push(node);
            
        }

        let mut cur = Some(stck.pop().unwrap());
        while !stck.is_empty() {
            let mut tmp = stck.pop().unwrap();
            tmp.next = cur;
            cur = Some(tmp);
        }
        
       cur
    }
}