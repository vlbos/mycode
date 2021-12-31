/*
 * @lc app=leetcode id=18 lang=rust
 *
 * [18] 4Sum
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::new();
        }
        let mut ns = nums;
        ns.sort();
        let n = ns.len();
        let mut ans = Vec::new();

        let mut l = 0;
        let mut r = 0;
        let mut sum = 0;
        for i in 0..ns.len() - 3 {
            if i > 0 && ns[i] == ns[i - 1] {
                continue;
            }
            if ns[i] + ns[i + 1] + ns[i + 2] + ns[i + 3] > target {
                break;
            }
            if ns[i] + ns[n - 1] + ns[n - 2] + ns[n - 3] < target {
                continue;
            }
            for j in i + 1..ns.len() - 2 {
                if j > i + 1 && ns[j] == ns[j - 1] {
                    continue;
                }
                if ns[i] + ns[j] + ns[j + 1] + ns[j + 2] > target {
                    break;
                }
                if ns[i] + ns[j] + ns[n - 1] + ns[n - 2] < target {
                    continue;
                }
                l = j + 1;
                r = n - 1;
                while l < r {
                    sum = ns[i] + ns[j] + ns[l] + ns[r];
                    if sum == target {
                        ans.push(vec![ns[i], ns[j], ns[l], ns[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && ns[l - 1] == ns[l] {
                            l += 1;
                        }
                        while l < r && ns[r + 1] == ns[r] {
                            r -= 1;
                        }
                    } else if sum > target {
                        r -= 1;
                    } else {
                        l += 1;
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
