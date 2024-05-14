pub struct Solution;
// impl Solution {
//     pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
//         let maxx = nums.iter().fold(0, |ac, x| {
//             if *x > ac { *x } else { ac }
//         });
        
//         let (mut cnt, mut left) = (0, 0);

//         let mut res = 0;

//         for (ind, num) in nums.iter().enumerate() {
//             if *num == maxx {
//                 cnt += 1;
//             }

//             while cnt >= k {
//                 if nums[left] == maxx {
//                         cnt -= 1;
//                 }

//                 left += 1;
//             }

//             res += left;

//         }

//         res as i64
//     }
// }
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let maxx = nums.iter().fold(0, |ac, x| {
            if *x > ac { *x } else { ac }
        });

        let mut arr: Vec<usize> = vec![];
        for (ind, num) in nums.iter().enumerate() {
            if *num == maxx {
                arr.push(ind);
            }
        }

        if arr.len() < k as usize {
            return 0
        }

        let mut right = (k - 1) as usize;
        let mut res = 0;

        for _ in (k - 1) as usize..arr.len() {
            let left_side = arr[right - (k as usize - 1)] + 1;
            let right_side = if right == arr.len() - 1 { nums.len() - arr[right] } else { arr[right + 1] - arr[right] };
            res += left_side * right_side;
            right += 1;
        }
        res as i64
    }
}