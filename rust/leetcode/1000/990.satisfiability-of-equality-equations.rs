/*
 * @lc app=leetcode id=990 lang=rust
 *
 * [990] Satisfiability of Equality Equations
 */

// @lc code=start
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        fn find(parents: &mut Vec<usize>, x: usize) -> usize {
            if parents[x] == x {
                return x;
            }
            find(parents, parents[x])
        }
        let merge = |parents: &mut Vec<usize>, x: usize, y: usize| {
            let px = find(parents, x);
            let py = find(parents, y);
            parents[px] = py;
        };
        let mut parents = (0..26).collect::<Vec<usize>>();
        for e in &equations {
            let s = e.bytes().collect::<Vec<u8>>();
            if s[1] == b'=' {
                let px = (s[0] - b'a') as usize;
                let py = (s[3] - b'a') as usize;
                merge(&mut parents, px, py);
            }
        }
        for e in &equations {
            let s = e.bytes().collect::<Vec<u8>>();
            if s[1] == b'!' {
                let px = find(&mut parents, (s[0] - b'a') as usize);
                let py = find(&mut parents, (s[3] - b'a') as usize);
                if px == py {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end
