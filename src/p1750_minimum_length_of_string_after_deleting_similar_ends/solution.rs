pub struct Solution {}

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let (mut i, mut j) = (0_usize, s.len() - 1);
        let bytes = s.as_bytes();

        while i != j && bytes[i] == bytes[j] {
            let cur = bytes[i];

            let (mut ni, mut nj) = (i, j);

            while ni < j && bytes[ni] == cur {
                ni += 1;
            }

            if ni == j {
                return 0;
            }

            while bytes[nj] == cur {
                nj -= 1;
            }

            i = ni;
            j = nj;

        }

        (j - i + 1) as i32
    }
}