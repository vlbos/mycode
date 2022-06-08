/*
 * @lc app=leetcode id=1005 lang=rust
 *
 * [1005] Maximize Sum Of Array After K Negations
 */

// @lc code=start
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut currminvalue = i32::MAX;
        let mut zeroindex = false;
        let mut sum = 0;
        let mut r = Vec::new();
        for n in &nums {
            if *n < 0 {
                r.push(*n);
            } else if *n == 0 {
                zeroindex = true;
            } else {
                currminvalue = currminvalue.min(*n);
                sum += *n;
            }
        }
        let nn = r.len();
        let ku = k as usize;
        if nn > 0 {
            r.sort();
            for i in 0..nn {
                if i < ku {
                    currminvalue = currminvalue.min(r[i].abs());
                }
                sum += if i < ku { r[i].abs() } else { r[i] };
            }
        }
        let mut kk = if nn > ku { 0 } else { ku - nn };
        if kk > 0 && !zeroindex && kk % 2 != 0 {
            sum -= currminvalue * 2;
        }
        sum
    }
}
// @lc code=end
