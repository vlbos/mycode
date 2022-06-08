/*
 * @lc app=leetcode id=1769 lang=rust
 *
 * [1769] Minimum Number of Operations to Move All Balls to Each Box
 */

// @lc code=start
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let b = boxes.as_bytes();
        let (mut pre,mut precnt) = (0,0);
        let mut ans = vec![0; n];
        for i in 0..n {
            ans[i] = precnt;
            pre+=(b[i]-b'0') as i32;
            precnt +=pre;
        }
        pre = 0;
        precnt = 0;
        for i in (0..n).rev() {
            ans[i] += precnt;
            pre+=(b[i]-b'0') as i32;
            precnt +=pre;
        }
        ans
    }
}
// @lc code=end
