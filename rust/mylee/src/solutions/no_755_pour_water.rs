// 755\. Pour Water
// ================

// We are given an elevation map, `heights[i]` representing the height of the terrain at that index.
// The width at each index is 1. After `V` units of water fall at index `K`, how much water is at each index?

// Water first drops at index `K` and rests on top of the highest terrain or water at that index. Then, it flows according to the following rules:

// *   If the droplet would eventually fall by moving left, then move left.
// *   Otherwise, if the droplet would eventually fall by moving right, then move right.
// *   Otherwise, rise at it's current position.
// Here, "eventually fall" means that the droplet will eventually be at a lower level if it moves in that direction.
// Also, "level" means the height of the terrain plus any water in that column.

// We can assume there's infinitely high terrain on the two sides out of bounds of the array.
//  Also, there could not be partial water being spread out evenly on more than 1 grid block - each unit of water has to be in exactly one block.

// **Example 1:**

// **Input:** heights = \[2,1,1,2,1,2,2\], V = 4, K = 3
// **Output:** \[2,2,2,3,2,2,2\]
// **Explanation:**
// #       #
// #       #
// ##  # ###
// #########
//  0123456    <- index

// The first drop of water lands at index K = 3:

// #       #
// #   w   #
// ##  # ###
// #########
//  0123456

// When moving left or right, the water can only move to the same level or a lower level.
// (By level, we mean the total height of the terrain plus any water in that column.)
// Since moving left will eventually make it fall, it moves left.
// (A droplet "made to fall" means go to a lower height than it was at previously.)

// #       #
// #       #
// ## w# ###
// #########
//  0123456

// Since moving left will not make it fall, it stays in place.  The next droplet falls:

// #       #
// #   w   #
// ## w# ###
// #########
//  0123456

// Since the new droplet moving left will eventually make it fall, it moves left.
// Notice that the droplet still preferred to move left,
// even though it could move right (and moving right makes it fall quicker.)

// #       #
// #  w    #
// ## w# ###
// #########
//  0123456

// #       #
// #       #
// ##ww# ###
// #########
//  0123456

// After those steps, the third droplet falls.
// Since moving left would not eventually make it fall, it tries to move right.
// Since moving right would eventually make it fall, it moves right.

// #       #
// #   w   #
// ##ww# ###
// #########
//  0123456

// #       #
// #       #
// ##ww#w###
// #########
//  0123456

// Finally, the fourth droplet falls.
// Since moving left would not eventually make it fall, it tries to move right.
// Since moving right would not eventually make it fall, it stays in place:

// #       #
// #   w   #
// ##ww#w###
// #########
//  0123456

// The final answer is \[2,2,2,3,2,2,2\]:

//     #
//  #######
//  #######
//  0123456

// **Example 2:**

// **Input:** heights = \[1,2,3,4\], V = 2, K = 2
// **Output:** \[2,3,3,4\]
// **Explanation:**
// The last droplet settles at index 1, since moving further left would not cause it to eventually fall to a lower height.

// **Example 3:**

// **Input:** heights = \[3,1,3\], V = 5, K = 1
// **Output:** \[4,4,4\]

// **Note:**

// 1.  `heights` will have length in `[1, 100]` and contain integers in `[0, 99]`.
// 2.  `V` will be in range `[0, 2000]`.
// 3.  `K` will be in range `[0, heights.length - 1]`.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Airbnb](https://leetcode.ca/tags/#Airbnb) [Robinhood](https://leetcode.ca/tags/#Robinhood)

// @lc code=start
impl Solution {
    pub fn pour_water(mut heights: Vec<i32>, v: i32, k: i32) -> Vec<i32> {
        // let len = heights.len() as i32;
        // let mut v = v;
        // while v > 0 {
        //     let mut i = k - 1;
        //     let mut left_low = k;
        //     while i >= -1 {
        //         let h = if i >= 0 {
        //             heights[i as usize]
        //         } else {
        //             i32::max_value()
        //         };
        //         let prev_i = (i + 1) as usize;
        //         let prev_h = heights[prev_i];
        //         if h > prev_h {
        //             break;
        //         } else if h < prev_h {
        //             left_low = i;
        //         }
        //         i -= 1;
        //     }
        //     if left_low == k {
        //         let mut i = k + 1;
        //         let mut right_low = k;
        //         while i <= len {
        //             let h = if i < len {
        //                 heights[i as usize]
        //             } else {
        //                 i32::max_value()
        //             };
        //             let prev_i = i as usize - 1;
        //             let prev_h = heights[prev_i];
        //             if h > prev_h {
        //                 break;
        //             } else if h < prev_h {
        //                 right_low = i;
        //             }
        //             i += 1;
        //         }
        //         heights[right_low as usize] += 1;
        //     } else {
        //         heights[left_low as usize] += 1;
        //     }
        //     v -= 1;
        // }
        // heights
        for _ in 0..v {
            let mut cur = k as usize;
            while cur > 0 && heights[cur] >= heights[cur - 1] {
                cur -= 1;
            }
            while cur < heights.len() - 1 && heights[cur] >= heights[cur + 1] {
                cur += 1;
            }
            while cur > k as usize && heights[cur] >= heights[cur - 1] {
                cur -= 1;
            }
            heights[cur] += 1;
        }
        heights
    }
}
// @lc code=end

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_pour_water_1() {
        assert_eq!(
            Solution::pour_water(vec![2, 1, 1, 2, 1, 2, 2], 4, 3),
            vec![2, 2, 2, 3, 2, 2, 2]
        )
    }

    #[test]
    pub fn test_pour_water_2() {
        assert_eq!(
            Solution::pour_water(vec![1, 2, 3, 4], 2, 2),
            vec![2, 3, 3, 4]
        );
    }

    #[test]
    pub fn test_pour_water_3() {
        assert_eq!(Solution::pour_water(vec![3, 1, 3], 5, 1), [4, 4, 4]);
    }
}
