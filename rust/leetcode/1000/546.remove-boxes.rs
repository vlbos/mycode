/*
 * @lc app=leetcode id=546 lang=rust
 *
 * [546] Remove Boxes
 */

// @lc code=start
impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        fn calculate_points(
            boxes: &Vec<i32>,
            l: i32,
            r: i32,
            k: i32,
            dp: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if l > r {
                return 0;
            }
            if dp[l as usize][r as usize][k as usize] == 0 {
                let (mut r1, mut k1) = (r, k);
                while r1 > l && boxes[r1 as usize] == boxes[r1 as usize - 1] {
                    r1 -= 1;
                    k1 += 1;
                }
                dp[l as usize][r as usize][k as usize]  = calculate_points(boxes, l, r1 - 1, 0, dp) + (k1 + 1) * (k1 + 1);
                for i in l..r1 {
                    if boxes[r1 as usize] == boxes[i as usize] {
                        dp[l as usize][r as usize][k as usize]  = dp[l as usize][r as usize][k as usize] .max(
                            calculate_points(boxes, l, i, k1 + 1, dp)
                                + calculate_points(boxes, i + 1, r1 - 1, 0, dp),
                        );
                    }
                }
            }
            dp[l as usize][r as usize][k as usize] 
        }
        let mut dp = vec![vec![vec![0; 100]; 100]; 100];
        calculate_points(&boxes, 0, boxes.len() as i32 - 1, 0, &mut dp) 
    }
}
// @lc code=end
