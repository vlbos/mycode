/*
 * @lc app=leetcode id=1508 lang=rust
 *
 * [1508] Range Sum of Sorted Subarray Sums
 */

// @lc code=start
impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let mut sum = 0;
        let mut presum = vec![0];
        for &v in &nums {
            sum += v;
            presum.push(sum);
        }
        let mut prepresum = Vec::new();
        sum = 0;
        for &v in &presum {
            sum += v;
            prepresum.push(sum);
        }
        let get_count = |x: i32| -> i32 {
            let mut count = 0;
            let mut j = 1;
            for i in 0..n {
                while j <= n && presum[j] - presum[i] <= x {
                    j += 1;
                }
                j -= 1;
                count += (j - i) as i32;
            }
            count
        };
        let get_kth = |k: i32| -> i32 {
            let (mut low, mut high) = (0, presum[n]);
            while low < high {
                let mid = (low + high) / 2;
                let count = get_count(mid);
                if count < k {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            low
        };
        let get_sum = |k: i32| -> i32 {
            let num = get_kth(k);
            let (mut total, mut count) = (0, 0);
            let mut j = 1;
            for i in 0..n {
                while j <= n && presum[j] - presum[i] <= num {
                    j += 1;
                }
                j -= 1;
                total += prepresum[j] - prepresum[i] - presum[i] * (j - i) as i32;
                total %= 1000_000_007;
                count += (j - i) as i32;
            }
            total += num * (k - count);
            total %= 1000_000_007;
            total
        };
        (get_sum(right) - get_sum(left - 1)) % 1000_000_007
    }
}
// @lc code=end
