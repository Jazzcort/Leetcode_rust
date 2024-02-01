pub struct Solution {}

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        if nums.chunks(3).map(|c| c[2] - c[0]).max().unwrap() > k {
            return vec![]
        }
        
        nums.chunks(3).map(|c| c.to_vec()).collect()
    }
}

// Other approach
// impl Solution {
//     pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
//         let mut res: Vec<Vec<i32>> = vec![];
//         let mut cur_stack: Vec<i32> = vec![];
//         let mut sorted_nums = nums.clone();
//         sorted_nums.sort_unstable_by(|a, b| b.cmp(a));

//         while !sorted_nums.is_empty() {
//             let cur = sorted_nums.pop().unwrap();

//             if !cur_stack.is_empty() {
//                 for num in cur_stack.iter() {
//                     if (cur - *num).abs() > k {
//                         return vec![];
//                     }
//                 }
//             }

//             cur_stack.push(cur);

//             if cur_stack.len() == 3 {
//                 res.push(cur_stack);
//                 cur_stack = vec![];
//             }
//         }
//         res
//     }
// }