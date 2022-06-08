/*
 * @lc app=leetcode id=1505 lang=rust
 *
 * [1505] Minimum Possible Integer After at Most K Adjacent Swaps On Digits
 */

// @lc code=start
impl Solution {
    pub fn min_integer(num: String, k: i32) -> String {
        let n = num.len();
        let mut tree = vec![0; n + 1];
        let lowbit = |x: i32| -> i32 { x & (-x) };
        let update = |mut x: i32, tree: &mut Vec<i32>| {
            while x < tree.len() as i32 {
                tree[x as usize] += 1;
                x += lowbit(x);
            }
        };
        let query = |mut x: i32, tree: &Vec<i32>| {
            let mut ans = 0;
            while x > 0 {
                ans += tree[x as usize];
                x -= lowbit(x);
            }
            ans
        };
        let query_range = |x: i32, y: i32, tree: &Vec<i32>| -> i32 { query(y,tree) - query(x - 1,tree) };
        let mut pos = vec![Vec::new(); 10];
        let bn = num.as_bytes();
        for i in (0..n).rev() {
            pos[(bn[i] - b'0') as usize].push(i as i32 + 1);
        }
        let mut ans = Vec::new();
        let mut k = k;
        for i in 1..=n {
            for j in 0..10 {
                if pos[j].is_empty() {
                    continue;
                }
                let lastj = pos[j][pos[j].len() - 1];
                let behind = query_range(lastj + 1, n as i32,&tree);
                let dist = lastj + behind - i as i32;
                if dist <= k {
                    update(pos[j].pop().unwrap(), &mut tree);
                    ans.push(b'0' + j as u8);
                    k -= dist;
                    break;
                }
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end
