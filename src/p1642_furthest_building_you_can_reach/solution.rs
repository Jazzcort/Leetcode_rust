use std::collections::BinaryHeap;
pub struct Solution {}

// impl Solution {
//     pub fn furthest_building(heights: Vec<i32>, bricks: i32, mut ladders: i32) -> i32 {
//         let mut heap: BinaryHeap<i32> = BinaryHeap::new();
//         let mut cur_sum = 0;
//         let mut ind = 0;

//         while ind < heights.len() - 1 {
//             if heights[ind + 1] > heights[ind] {
//                 let diff = heights[ind + 1] - heights[ind];
//                 cur_sum += diff;
//                 heap.push(diff);

//                 while ladders > 0 && cur_sum > bricks {
//                     cur_sum -= heap.pop().unwrap();
//                     ladders -= 1;
//                 }

//                 if cur_sum > bricks {
//                     return ind as i32;
//                 }
//             }
//             ind += 1;
//         }

//         ind as i32
//     }
// }

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut ind = 0;

        while ind < heights.len() - 1 {
            if heights[ind + 1] > heights[ind] {
                heap.push(-(heights[ind + 1] - heights[ind]));

                if heap.len() > ladders as usize {
                    bricks += heap.pop().unwrap();
                }

                if bricks < 0 {
                    return ind as i32;
                }
            }
            ind += 1;
        }

        (heights.len() - 1) as i32
    }
}