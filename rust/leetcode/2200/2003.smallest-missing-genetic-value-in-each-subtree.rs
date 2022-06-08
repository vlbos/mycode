/*
 * @lc app=leetcode id=2003 lang=rust
 *
 * [2003] Smallest Missing Genetic Value in Each Subtree
 */

// @lc code=start
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len();
        let mut ans = vec![1; n];
        let mut g = vec![Vec::new(); n];
        for (i, &v) in parents.iter().enumerate().skip(1) {
            g[v as usize].push(i);
        }
        use std::collections::HashMap;
        let mut in_set = HashMap::new();
        fn f(v: i32, nums: &Vec<i32>, g: &Vec<Vec<usize>>, in_set: &mut HashMap<i32, bool>) {
            in_set.insert(nums[v as usize], true);
            for &u in &g[v as usize] {
                if !in_set.contains_key(&nums[u]) {
                    f(u as i32, nums, g, in_set);
                }
            }
        }
        let mut x = if let Some(i) = nums.iter().position(|u| *u == 1) {
            i as i32
        } else {
            -1
        };
        let mut cur = 2;
        while x >= 0 {
            f(x, &nums, &g, &mut in_set);
            while in_set.contains_key(&cur) {
                cur += 1;
            }
            ans[x as usize] = cur;
            x = parents[x as usize];
        }
        ans
    }
}
// @lc code=end
