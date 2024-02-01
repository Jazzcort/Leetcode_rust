pub struct MyQueue {
    to_push: Vec<i32>,
    to_pop: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            to_push: vec![],
            to_pop: vec![],
        }
    }

    pub fn push(&mut self, x: i32) {
        self.to_push.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.to_pop.is_empty() {
            while !self.to_push.is_empty() {
                self.to_pop.push(self.to_push.pop().unwrap())
            }
        }
        self.to_pop.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        if self.to_pop.is_empty() {
            self.to_push[0]
        } else {
            self.to_pop[self.to_pop.len() - 1]
        }
    }

    pub fn empty(&self) -> bool {
        if self.to_push.is_empty() && self.to_pop.is_empty() {
            true
        } else {
            false
        }
    }
}
