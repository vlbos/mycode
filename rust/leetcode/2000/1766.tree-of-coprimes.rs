/*
 * @lc app=leetcode id=1766 lang=rust
 *
 * [1766] Tree of Coprimes
 */

// @lc code=start
impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }
        let mut ans = vec![0; n];
        let mut lasts = vec![Vec::new(); 50];
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        fn dfs(
            node: usize,
            pre: usize,
            level: i32,
            nums: &Vec<i32>,
            g: &Vec<Vec<usize>>,
            lasts: &mut Vec<Vec<(i32, i32)>>,
            ans: &mut Vec<i32>,
        ) {
            let (mut res, mut level1) = (-1, -1);
            for (i, v) in lasts.iter().enumerate() {
                if !v.is_empty()
                    &&  v[v.len() - 1].0 > level1
                    && gcd(i as i32+1, nums[node]) == 1
                {
                    level1 = v[v.len() - 1].0;
                    res = v[v.len() - 1].1;
                }
            }
            ans[node] = res as i32;
            for &ne in &g[node] {
                if ne != pre {
                    lasts[nums[node] as usize - 1].push((level, node as i32));
                    dfs(ne, node, level + 1, nums, g, lasts, ans);
                    lasts[nums[node] as usize - 1].pop();
                }
            }
        }
        dfs(0, n, 0, &nums, &g, &mut lasts, &mut ans);
        ans
    }
}
// @lc code=end
