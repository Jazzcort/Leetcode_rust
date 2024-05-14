use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}



#[macro_export]
macro_rules! wrap_vec  {
    ( $num: expr ) => {
        {
            let mut tmp_vec = vec![];

            let str_vec: Vec<&str> = $num.split(",").collect();

            for number in str_vec.into_iter() {
                match number.parse::<i32>() {
                    Ok(x) => { tmp_vec.push(Some(x)) }
                    Err(_) => { tmp_vec.push(None) }
                }
            }
         
            dbg!($num);

            tmp_vec
        }
    };
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tree {
    head: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl Tree {
    pub fn create_empty() -> Self {
        return Tree { head: None };
    }

    pub fn from(arr: Vec<Option<i32>>) -> Self {
        if arr.len() == 0 {
            return Self::create_empty();
        }

        // let mut deque: VecDeque<i32> = VecDeque::from(arr);
        let mut ind = 0;
        let length = arr.len();

        let mut nodes: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        let head = Some(Rc::new(RefCell::new(TreeNode::new(match arr[ind] {
            Some(x) => x,
            None => panic!("invalid array!"),
        }))));

        nodes.push_back(Rc::clone(head.as_ref().unwrap()));

        ind += 1;

        while !nodes.is_empty() && ind < length {
            let n = nodes.len();

            for _ in 0..n {
                let cur = nodes.pop_front().unwrap();

                if ind < length {
                    // let b = cur.borrow_mut().left;

                    cur.borrow_mut().left = match arr[ind] {
                        Some(x) => Some(Rc::new(RefCell::new(TreeNode {
                            val: x,
                            left: None,
                            right: None,
                        }))),
                        None => None,
                    };

                    match cur.borrow().left.as_ref() {
                        Some(x) => {
                            nodes.push_back(Rc::clone(x));
                        }
                        None => {}
                    }

                    ind += 1;
                } else {
                    break;
                }

                if ind < length {
                    cur.borrow_mut().right = match arr[ind] {
                        Some(x) => Some(Rc::new(RefCell::new(TreeNode {
                            val: x,
                            left: None,
                            right: None,
                        }))),
                        None => None,
                    };

                    match cur.borrow().right.as_ref() {
                        Some(x) => {
                            nodes.push_back(Rc::clone(x));
                        }
                        None => {}
                    }

                    ind += 1;
                } else {
                    break;
                }
            }
        }

        Tree { head }
    }

    pub fn get_head(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.head.to_owned()
    }

    // pub fn wrap_vec(vec: &Vec<i32>) -> Vec<Option<i32>> {
    //     let mut res = vec![];
    //     for num in vec.into_iter() {
    //         res.push()
    //     }
    // }
    // pub fn to_vec(&self) -> Vec<i32> {

    //   vec![1]
    // }
}
