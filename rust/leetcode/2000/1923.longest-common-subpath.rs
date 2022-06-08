/*
 * @lc app=leetcode id=1923 lang=rust
 *
 * [1923] Longest Common Subpath
 */

// @lc code=start
impl Solution {
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        use rand::distributions::{Distribution, Uniform};
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let p1 = 1_000_000_007;
        let p2 = 1_000_000_009;
        let die = Uniform::from(1_000_000i64..10_000_000i64);
        let base1 = die.sample(&mut rng);
        let base2 = die.sample(&mut rng);
        let m = paths.len();
        let (mut left, mut right) = (1, paths.iter().min_by_key(|x| x.len()).unwrap().len() as i32);
        let mut ans = 0;
        use std::collections::HashSet;
        while left <= right {
            let len = ((left + right) / 2) as usize;
            let (mut mult1, mut mult2) = (1, 1);
            for _ in 0..len {
                mult1 = mult1 * base1 % p1;
                mult2 = mult2 * base2 % p2;
            }
            let mut s = HashSet::new();
            let mut check = true;
            for i in 0..m {
                let (mut hash1, mut hash2) = (0, 0);

                for j in 0..len {
                    hash1 = (hash1 * base1 + (paths[i][j] as i64)) %p1;
                    hash2 = (hash2 * base2 + (paths[i][j] as i64)) % p2;
                }
                let mut t = HashSet::new();
                if i == 0 || s.contains(&vec![hash1, hash2]) {
                    t.insert(vec![hash1, hash2]);
                }
                for j in len..paths[i].len() {
                    hash1 = ((hash1 * base1 % p1 - (paths[i][j - len] as i64) * mult1 % p1
                        + (paths[i][j] as i64))
                        % p1
                        + p1)
                        % p1;
                    hash2 = ((hash2 * base2%p2 - (paths[i][j - len] as i64) * mult2 %p2
                        + (paths[i][j] as i64))
                        % p2
                        + p2)
                        % p2;
                    if i == 0 || s.contains(&vec![hash1, hash2]) {
                        t.insert(vec![hash1, hash2]);
                    }
                }
                if t.is_empty() {
                    check = false;
                    break;
                }
                s = t;
            }
            let len = len as i32;
            if check {
                ans = len;
                left = len + 1;
            } else {
                right = len - 1;
            }
        }
        ans
    }
}
// @lc code=end
