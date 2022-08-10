// 302\. Smallest Rectangle Enclosing Black Pixels
// ===============================================

// An image is represented by a binary matrix with `0` as a white pixel and `1` as a black pixel.
// The black pixels are connected, i.e., there is only one black region.
//  Pixels are connected horizontally and vertically.
// Given the location `(x, y)` of one of the black pixels,
// return the area of the smallest (axis-aligned) rectangle that encloses all black pixels.

// **Example:**

// **Input:**
// \[
//   "0010",
//   "0110",
//   "0100"
// \]
// and `x = 0,` `y = 2`

// **Output:** 6

// ### Difficulty:

// Hard

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)
// @lc code=start
impl Solution {
    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        // if image.is_empty() {
        //     return 0;
        // }
        // if image[0].is_empty() {
        //     return 0;
        // }
        // let row_len = image.len();
        // let col_len: usize = image[0].len();
        // let mut rows = vec![false; row_len];
        // let mut cols = vec![false; col_len];
        // for i in 0..row_len {
        //     for j in 0..col_len {
        //         rows[i] = rows[i] || image[i][j] == '1';
        //         cols[j] = cols[j] || image[i][j] == '1';
        //     }
        // }
        // let height = rows.into_iter().filter(|r| *r).count();
        // let width = cols.into_iter().filter(|c| *c).count();
        // (height * width) as i32
        let mut xy = vec![vec![x; 2], vec![y; 2]];
        for (i, row) in image.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == '1' {
                    for k in 0..2 {
                        let vv = if k == 0 { i as i32 } else { j as i32 };
                        xy[k][0] = xy[k][0].min(vv);
                        xy[k][1] = xy[k][1].max(vv);
                    }
                }
            }
        }
        (xy[0][1] - xy[0][0] + 1) * (xy[1][1] - xy[1][0] + 1)
    }
}
// @lc code=end 
#[allow(dead_code)] 
 struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_area() {
        let input = ["0010", "0110", "0100"]
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        assert_eq!(Solution::min_area(input, 0, 2), 6);
    }
}
