/*
 * @lc app=leetcode id=1803 lang=rust
 *
 * [1803] Count Pairs With XOR in a Range
 */

// @lc code=start
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        let n = 20011;
        let mut ne = vec![vec![0; 2]; n * 20];
        let mut cnt = vec![0; n * 15];
        let insert = |x: i32, idx: &mut i32, ne: &mut Vec<Vec<i32>>, cnt: &mut Vec<i32>| {
            let mut p = 0;
            for i in (0..=15).rev() {
                let j = (x >> i & 1) as usize;
                if ne[p][j] == 0 {
                    *idx += 1;
                    ne[p][j] = *idx;
                }
                p = ne[p][j] as usize;
                cnt[p] += 1;
            }
        };
        let query = |x: i32, hi: i32, ne: &Vec<Vec<i32>>, cnt: &Vec<i32>| {
            let mut ans = 0;
            let mut p = 0;
            for i in (0..=15).rev() {
                let j = (x >> i & 1) as usize;
                let k = hi >> i & 1;
                if k > 0 {
                    if ne[p][j] > 0 {
                        ans += cnt[ne[p][j] as usize];
                    }
                    if ne[p][1 - j] == 0 {
                        return ans;
                    }
                    p = ne[p][1 - j] as usize;
                } else {
                    if ne[p][j] == 0 {
                        return ans;
                    }
                    p = ne[p][j] as usize;
                }
            }
            ans += cnt[p];
            ans
        };

        let mut idx = 0;
        let mut ans = 0;
        for &num in &nums {
            insert(num, &mut idx, &mut ne, &mut cnt);
            ans += query(num, high, &ne, &cnt) - query(num, low - 1, &ne, &cnt);
        }
        ans
    }
}
// @lc code=end
