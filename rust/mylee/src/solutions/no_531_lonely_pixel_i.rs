// 531\. Lonely Pixel I
// ====================

// Given a picture consisting of black and white pixels, find the number of **black** lonely pixels.

// The picture is represented by a 2D char array consisting of 'B' and 'W', which means black and white pixels respectively.

// A black lonely pixel is character 'B' that located at a specific position where the same row and same column don't have any other black pixels.

// **Example:**

// **Input:**
// \[\['W', 'W', 'B'\],
//  \['W', 'B', 'W'\],
//  \['B', 'W', 'W'\]\]

// **Output:** 3
// **Explanation:** All the three 'B's are black lonely pixels.

// **Note:**

// 1.  The range of width and height of the input 2D array is \[1,500\].

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

// @lc code=start
impl Solution {
    pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
        // let rows = picture.len();
        // let cols = if rows == 0 { 0 } else { picture[0].len() };
        // if rows == 0 || cols == 0 {
        //     return 0;
        // };
        // let mut row_b = vec![(0usize, 0usize); rows];
        // let mut col_b = vec![0usize; cols];
        // for i in 0..rows {
        //     for j in 0..cols {
        //         if picture[i][j] == 'B' {
        //             row_b[i] = (row_b[i].0 + 1, j);
        //             col_b[j] += 1;
        //         }
        //     }
        // }
        // row_b.into_iter().fold(0i32, |acc, (cnt, col)| {
        //     acc + if cnt == 1 && col_b[col] == 1 { 1 } else { 0 }
        // })
        let (m, n) = (picture.len(), picture[0].len());
        let (mut rows, mut cols) = (vec![0; m], vec![0; n]);
        picture.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &v)| {
                if v == 'B' {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            })
        });
        picture
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, &v)| {
                        if v == 'B' && rows[i] == 1 && cols[j] == 1 {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_find_lonely_pixel_1() {
        assert_eq!(
            Solution::find_lonely_pixel(vec![
                vec!['W', 'W', 'B'],
                vec!['W', 'B', 'W'],
                vec!['B', 'W', 'W']
            ]),
            3
        );
    }
}
