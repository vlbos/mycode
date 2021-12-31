/*
 * @lc app=leetcode id=874 lang=rust
 *
 * [874] Walking Robot Simulation
 */

// @lc code=start
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
         let mut d = 0;
        let xyd = vec![1, 1, -1, -1];
        let mut i = 0;
        let mut j = 1;
        let mut xy = vec![0, 0].to_vec();
        let o: HashSet<Vec<i32>> = HashSet::from_iter(obstacles.iter().cloned());

        let co = |nxy: Vec<i32>, step: i32, j: usize, i: usize| -> Vec<i32> {
            let mut rnxy = nxy.clone();
            for n in 0..step {
                let mut nxyc = nxy.clone();
                nxyc[j] += xyd[i] * (n + 1);
                if o.contains(&nxyc) {
                    break;
                }
                rnxy = nxyc;
            }
  
            rnxy
        };

        for c in &commands {
            if *c == -1 {
                i += 1;
                i %= xyd.len();
                j = (i % 2 + 1) % 2;
            } else if *c == -2 {
                i += xyd.len() - 1;
                i %= xyd.len();
                j = (i % 2 + 1) % 2;
            } else {
                xy = co(xy.clone(), *c, j, i as usize);
                let dd = xy[0] * xy[0] + xy[1] * xy[1];

                if dd > d {
                    d = dd;
                }
            }
        }
        d
    }
}
// @lc code=end

