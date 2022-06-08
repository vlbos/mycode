/*
 * @lc app=leetcode id=1982 lang=rust
 *
 * [1982] Find Array Given Subset Sums
 */

// @lc code=start
impl Solution {
    pub fn recover_array(n: i32, sums: Vec<i32>) -> Vec<i32> {
        let mut sums = sums;
        sums.sort();
        let n = n as u32;
        let mut ans = Vec::new();
        for i in (2..=n).rev() {
            let d = sums[1] - sums[0];
            let (mut left, mut right) = (0, 0);
            let (mut s, mut t) = (Vec::new(), Vec::new());
            let mut used = vec![false; 1 << i];
            loop {
                while left < (1 << i) && used[left] {
                    left += 1;
                }
                if left == (1 << i) {
                    break;
                }
                s.push(sums[left]);
                used[left] = true;
                while used[right] || sums[right] != sums[left] + d {
                    right += 1;
                }

                t.push(sums[right]);
                used[right] = true;
            }
            if s.iter().find(|&x| *x == 0).is_some() {
                ans.push(d);
                sums = s;
            } else {
                ans.push(-d);
                sums = t;
            }
        }
        ans.push(sums[1] + sums[0]);
        ans
    }
}
// @lc code=end
