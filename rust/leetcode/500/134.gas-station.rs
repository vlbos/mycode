/*
 * @lc app=leetcode id=134 lang=rust
 *
 * [134] Gas Station
 */

// @lc code=start
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut i = 0;
        while i < n {
            let mut gassum = 0;
            let mut costsum = 0;
            let mut cnt = 0;
            while cnt < n {
                let j = (i + cnt) % n;
                gassum += gas[j];
                costsum += cost[j];
                if costsum > gassum {
                    break;
                }
                cnt+=1;
            }
            if cnt == n {
                return i as i32;
            } else {
                i = i + cnt + 1;
            }
        }
        -1
    }
}
// @lc code=end
