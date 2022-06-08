/*
 * @lc app=leetcode id=1649 lang=rust
 *
 * [1649] Create Sorted Array through Instructions
 */

// @lc code=start
impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let n = *instructions.iter().max().unwrap();
        let mut tree = vec![0; n as usize + 1];
        let lowbit = |x: i32| x & (-x);
        let update = |mut x: i32, tree: &mut Vec<i32>| {
            while x <= n {
                tree[x as usize] += 1;
                x += lowbit(x);
            }
        };
        let query = |mut x: i32, tree: &Vec<i32>| -> i32 {
            let mut ans = 0;
            while x > 0 {
                ans += tree[x as usize];
                x -= lowbit(x);
            }
            ans
        };
        let mut ans = 0;
        for (i, &x) in instructions.iter().enumerate() {
            let smaller = query(x - 1, &tree);
            let larger = i as i32 - query(x, &tree);
            ans += smaller.min(larger);
            ans %= 1_000_000_007;
            update(x, &mut tree);
        }
        ans
    }
}
// @lc code=end
