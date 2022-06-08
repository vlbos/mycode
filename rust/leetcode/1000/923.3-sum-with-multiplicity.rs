/*
 * @lc app=leetcode id=923 lang=rust
 *
 * [923] 3Sum With Multiplicity
 */

// @lc code=start
impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let modmax = 1_000_000_007;
        let mut arr = arr;
        arr.sort();
        let mut ans = 0i64;
        let n = arr.len();
        for i in 0..n {
            let t = target - arr[i];
            let (mut j, mut k) = (i + 1, n - 1);
            while j < k {
                if arr[j] + arr[k] < t {
                    j += 1;
                } else if arr[j] + arr[k] > t {
                    k -= 1;
                } else if arr[j] != arr[k] {
                    let (mut l, mut r) = (1, 1);
                    while j + 1 < k && arr[j] == arr[j + 1] {
                        l += 1;
                        j += 1;
                    }
                    while k - 1 > j && arr[k] == arr[k - 1] {
                        r += 1;
                        k -= 1;
                    }
                    ans += l * r;
                    ans %= modmax;
                    j += 1;
                    k -= 1;
                } else {
                    ans += ((k - j + 1) * (k - j)) as i64/2;
                    ans %= modmax;
                   break;
                }
            }
        }
        ans as _
    }
}
// @lc code=end
