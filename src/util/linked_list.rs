#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(arr: Vec<i32>) -> Self {
        let mut i = 0;
        let n = arr.len();
        let mut head = Box::new(Self::new(arr[i]));
        i += 1;

        let mut cur = head.as_mut();
        while i < n {
            cur.next = Some(Box::new(Self::new(arr[i])));
            i += 1;

            cur = cur.next.as_mut().unwrap();
        }
        head.as_ref().to_owned()
    }
}
