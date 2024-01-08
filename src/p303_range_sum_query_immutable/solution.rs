pub struct NumArray {
    arr: Vec<i32>

}

impl NumArray {

    pub fn new(nums: Vec<i32>) -> Self {
        let mut new_vec: Vec<i32> = vec![0; nums.len()];
        let mut cur: i32 = 0;

        for i in 0..nums.len() {
            cur += nums[i];
            new_vec[i] = cur;
        }

        NumArray {
            arr: new_vec
        }
    }
    
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut red = 0;

        if left > 0 {
            red = self.arr[left as usize - 1];
        }
        
        return self.arr[right as usize] - red
    }
}