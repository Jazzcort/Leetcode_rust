use crate::util::linked_list::ListNode;
pub struct Solution;

// impl Solution {
//     pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut arr: Vec<Option<Box<ListNode>>> = vec![];

//         let mut cur = head.as_ref();

//         loop {
//             match cur {
//                 Some(node) => {
//                     let val = node.val;
//                     let mut summ = 0;
//                     let mut arr2: Vec<Option<Box<ListNode>>> = vec![];
//                     let mut flag = false;

//                     while !arr.is_empty() {

//                         let cur_node = arr.pop().unwrap();
//                         summ += cur_node.as_ref().unwrap().val;
//                         arr2.push(cur_node);

//                         if val + summ == 0 {
//                             flag = true;
//                             break;
//                         }
//                     }

//                     while !flag && !arr2.is_empty() {
//                         arr.push(arr2.pop().unwrap());
//                     }
//                     if !flag && node.val != 0 {
//                         arr.push(cur.clone().cloned());
//                     }

//                     cur = cur.unwrap().next.as_ref();
//                 }
//                 None => {
//                     break;
//                 }
//             }
//         }

//         let mut last = None;

//         while !arr.is_empty() {
//             let mut cur = arr.pop().unwrap();
//             cur.as_mut().unwrap().next = last.clone();
//             last = cur;
//         }

//         last
//     }
// }

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr: Vec<i32> = vec![];

        let mut cursor = head.as_ref();

        while match cursor {
            Some(node) => {
                if node.val != 0 {
                    arr.push(node.val);
                }

                cursor = cursor.unwrap().next.as_ref();

                true
            }
            None => {
                false
            },
        } {}

        let n = arr.len();

        for i in 0..n {
            let mut summ = 0;
            for j in i..n  {
                summ += arr[j];
                if summ == 0 {
                    for k in i..=j {
                        arr[k] = 0;
                    }
                    break;
                }
            }
        }

        let mut head = None;

        while !arr.is_empty() {
            let num = arr.pop().unwrap();
            if num != 0 {
                let mut cur = ListNode::new(num);
                cur.next = head.clone();
                head = Some(Box::new(cur));
            }
        }


        head
    }
}
