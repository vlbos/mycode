/*
 * @lc app=leetcode id=1850 lang=rust
 *
 * [1850] Minimum Adjacent Swaps to Reach the Kth Smallest Number
 */

// @lc code=start
impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let n = num.len();
        let next_permutation = |nums: &mut Vec<u8>| {
           let mut i = nums.len() - 1;
            while i > 0 {
                if nums[i - 1] < nums[i] {
                    let mut j = i;
                    while j < nums.len() && nums[i - 1] < nums[j] {
                        j += 1;
                    }
                    nums.swap(i - 1, j - 1);
                    break;
                }
                i -= 1;
            }
            nums[i..].sort_unstable();
        };
        let mut nums: Vec<u8> = num.bytes().collect();
        let mut numsk = nums.clone();
        for _ in 0..k {
            next_permutation(&mut numsk);
        }
        let mut ans = 0;
        for i in 0..n {
            if nums[i] != numsk[i] {
                for j in i + 1..n {
                    if nums[j] == numsk[i] {
                        for k in (i..j).rev() {
                            ans += 1;
                            nums.swap(k, k + 1);
                        }
                        break;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
