/*
 * @lc app=leetcode id=493 lang=rust
 *
 * [493] Reverse Pairs
 */

// @lc code=start
impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
         fn reverse_pairs_recursive(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
            if left == right {
                return 0;
            }
            let mid = (left + right) / 2;
            let n1 = reverse_pairs_recursive(nums, left, mid);
            let n2 = reverse_pairs_recursive(nums, mid + 1, right);
            let mut ans = n1 + n2;
            let (mut i, mut j) = (left, mid + 1);
            while i <= mid {
                while j <= right && (nums[i] as i64) > 2 * (nums[j] as i64) {
                    j += 1;
                }
                ans += j - mid - 1;
                i += 1;
            }
            let mut sorted = vec![0; right - left + 1];
            let (mut p1, mut p2) = (left, mid + 1);
            let mut p = 0;
            while p1 <= mid || p2 <= right {
                if p1 > mid {
                    sorted[p] = nums[p2];
                    p2 += 1;
                } else if p2 > right {
                    sorted[p] = nums[p1];
                    p1 += 1;
                } else {
                    if nums[p1] < nums[p2] {
                        sorted[p] = nums[p1];
                        p1 += 1;
                    } else {
                        sorted[p] = nums[p2];
                        p2 += 1;
                    }
                }

                p += 1;
            }
            for i in 0..sorted.len() {
                nums[left + i] = sorted[i];
            }
            ans  
        }
        let mut nums = nums;
        let n =  nums.len() - 1;
        reverse_pairs_recursive(&mut nums, 0,n) as _
    }
}
// @lc code=end
