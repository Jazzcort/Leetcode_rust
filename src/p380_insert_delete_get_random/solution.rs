use std::collections::HashMap;
use rand::Rng;

pub struct RandomizedSet {
    dict: HashMap<i32, usize>,
    lst: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    pub fn new() -> Self {
        RandomizedSet {
            dict: HashMap::new(),
            lst: vec![]
        }
    }
    
    pub fn insert(&mut self, val: i32) -> bool {
        if self.dict.contains_key(&val) {
            false
        } else {
            self.dict.insert(val, self.lst.len());
            self.lst.push(val);
            true
        } 
    }
    
    pub fn remove(&mut self, val: i32) -> bool {
        if self.dict.contains_key(&val) {
            let n = self.lst.len();
            let (ind, last_elem) = (self.dict.get(&val).unwrap(), self.lst[n - 1].clone());

            let tmp1 = self.lst.get_mut(n - 1).unwrap();
            *tmp1 = val.clone();
            let tmp2 = self.lst.get_mut(*ind).unwrap();
            *tmp2 = last_elem.clone();
           
            self.dict.insert(last_elem.clone(), ind.clone());

            self.lst.pop();
            self.dict.remove(&val);

            true
        } else {
            false
        }
    }
    
    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();

        let ind = (rng.gen::<f32>() * self.lst.len() as f32).floor() as usize;

        self.lst[ind]
    }
}