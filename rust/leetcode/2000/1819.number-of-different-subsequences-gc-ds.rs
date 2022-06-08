/*
 * @lc app=leetcode id=1819 lang=rust
 *
 * [1819] Number of Different Subsequences GCDs
 */

// @lc code=start
impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let mut nums: std::collections::HashSet<i32> = nums.into_iter().collect();
        let n = *nums.iter().max().unwrap() as usize;
        let mut ans = 0;
        fn gcd(a:usize,b:usize)->usize{
            if b==0{a}else{gcd(b,a%b)}
        }
        for i in 1..=n {
            let mut k = n + 1;
            for j in (i..=n).step_by(i) {
                if !nums.contains(&(j as i32)) {
                    continue;
                }
                if k == n + 1 {
                    k = j;
                } else {
                    k = gcd(k, j);
                }
                if k == i {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}
// @lc code=end
