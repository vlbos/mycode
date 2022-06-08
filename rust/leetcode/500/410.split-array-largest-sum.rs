/*
 * @lc app=leetcode id=410 lang=rust
 *
 * [410] Split Array Largest Sum
 */

// @lc code=start
impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut l = *nums.iter().max().unwrap();
        let mut r = nums.iter().sum::<i32>();
        let check = |x: i32| -> bool {
            let mut sum = 0;
            let mut cnt=1;
            for &num in &nums {
                if sum + num > x {
                    cnt += 1;
                    sum = num;
                } else {
                    sum += num;
                }
            }
            cnt <= m
        };
        while l < r {
            let mid = l + (r - l) / 2;
            if check(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}
// @lc code=end
