pub struct Solution;
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (_rows, cols) = (matrix.len(), matrix[0].len());

        let mut heights = vec![0; cols + 1];
        let mut res = 0;

        for row in matrix.into_iter() {
            for c in 0..cols {
                heights[c] = if row[c] == '1' { heights[c] + 1 } else { 0 }
            }
            
            let mut stck = vec![];
            for ind in 0..heights.len() {
                while !stck.is_empty() && heights[ind] < heights[*stck.last().unwrap()] {
                    let h = heights[stck.pop().unwrap()];
                    let w = if stck.is_empty() { ind } else { ind - *stck.last().unwrap() - 1 };
                    res = res.max((h * w) as i32);
                }


                stck.push(ind);

               
            }
        }
        res
    }
}