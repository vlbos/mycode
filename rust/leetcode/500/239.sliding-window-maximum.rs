/*
 * @lc app=leetcode id=239 lang=rust
 *
 * [239] Sliding Window Maximum
 */

// @lc code=start
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
       let k = k as usize;
        let mut q =
            nums[..k].iter().enumerate().map(|x|(*x.1,x.0)).collect::<std::collections::BinaryHeap<(i32, usize)>>();
        let mut ans = vec![q.peek().unwrap().0];
        for i in k..nums.len() {
            q.push((nums[i], i));
            while q.peek().unwrap().1 <= i - k {
                q.pop();
            }
            ans.push(q.peek().unwrap().0);
        }
        ans
    }
}
// @lc code=end
