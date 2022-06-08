/*
 * @lc app=leetcode id=1575 lang=rust
 *
 * [1575] Count All Possible Routes
 */

// @lc code=start
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();
        let (start_pos, finish_pos) = (locations[start as usize], locations[finish as usize]);
        let mut locations = locations;
        locations.sort();
        let (start, finish) = (
            locations.iter().position(|x| *x == start_pos).unwrap(),
            locations.iter().position(|x| *x == finish_pos).unwrap(),
        );
        let fuel = fuel as usize;
        let mut dpl = vec![vec![0; fuel + 1]; n];
        let mut dpr = vec![vec![0; fuel + 1]; n];
        dpl[start][0] = 1;
        dpr[start][0] = 1;
        let p =  1_000_000_007;
        for used in 0..=fuel {
            for city in (0..n - 1).rev() {
                let delta = (locations[city + 1] - locations[city]) as usize;
                if delta <= used {

                    dpl[city][used] = 
                        (((if used == delta {
                        0
                    } else {dpl[city + 1][used - delta]}) * 2)%p + dpr[city + 1][used - delta])%p
                           
                    ;
                }
            }
            for city in 1..n {
                let delta =( locations[city] - locations[city - 1]) as usize;
                if delta <= used {
                    dpr[city][used] = 
                        (((if used == delta {
                        0
                    } else {dpr[city - 1][used - delta] })* 2)%p + dpl[city - 1][used - delta])
                            % p
                    ;
                }
            }
        }
        let mut ans = dpl[finish].iter().cloned().reduce(|acc,x| (x+acc)% p).unwrap() + dpr[finish].iter().cloned().reduce(|acc,x| (x+acc)% p).unwrap();
        if start == finish {
            ans -= 1;
        }
        ans % p
    }
}
// @lc code=end
