// 3935. Power Update After K-th Largest Insertion I
// ## Description
// You are given an integer array `nums` and an integer `p`.
// You are also given a 2D integer array `queries`,
// where each `queries[i] = [vali, ki]` and the difference between **consecutive** `ki` values is always **less** than 10.
// For each query:
// * Insert `vali` into `nums`.
// * Let `x` be the `kith` **largest** element in the current `nums`.
// * **Update** `p` to `px % (109 + 7)`.
// Return an array `ans` where the `ans[i]` represents the value of `p` after processing the `ith` query.

// **Example 1:**
// **Input:** nums = [2], p = 4, queries = [[3,1],[1,2]]
// **Output:** [64,4096]
// **Explanation:**
// |`i`|`vali`|Current `nums`|`ki`|`kith`largest|p |New `p = pk % (109 + 7)`|
// |0  |3     |[2, 3]        |1   |3            |4 |43 % (109 + 7) = 64|
// |1  |1     |[2, 3, 1]     |2   |2            |64|642 % (109 + 7) = 4096|
// Thus, `ans = [64, 4096]`.
// **Example 2:**
// **Input:** nums = [7,5], p = 6, queries = [[4,3],[7,2]]
// **Output:** [1296,220296870]
// **Explanation:**
// |`i`|`vali`|Current​​​​​​​`nums`|`ki`|`kith`largest|`p` |New `p = pk % (109 + 7)`|
// |0  |4     |[7, 5, 4]    |3   |4            |6   |64 % (109 + 7) = 1296|
// |1  |7     |[7, 5, 4, 7] |2   |7            |1296|12967 % (109 + 7) = 220296870|
// Thus, `ans = [1296, 220296870]`

// **Constraints:**
// * `1 \<= nums.length \<= 2 × 104`
// * `1 \<= nums[i] \<= 106`
// * `​​​​​​​1 \<= p \<= 106`
// * `1 \<= queries.length \<= 2 × 104`
// * `​​​​​​​1 \<= vali \<= 106`
// * `1 \<= ki \<= n + i + 1`
// * `|ki - ki - 1| \< 10` for `i \> 0`

//  vector<int> powerUpdate(vector<int>& nums, int p, vector<vector<int>>& queries) {

#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn power_update(nums: Vec<i32>, p: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;
        let pow_mod = |mut x: i64, mut p: i32| -> i64 {
            let mut ans = 1;
            while p > 0 {
                if p & 1 == 1 {
                    ans = (ans * x) % MOD;
                }
                x = (x * x) % MOD;
                p >>= 1;
            }
            ans
        };
        let mut sorted: Vec<_> = queries.iter().map(|q| q[0]).chain(nums.clone()).collect();
        sorted.sort_unstable();
        sorted.dedup();
        let val2idx: std::collections::HashMap<_, _> =
            sorted.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut bits = vec![0; val2idx.len()];
        let update = |mut x: i32, val: i32, bits: &mut Vec<i32>| {
            x += 1;
            let n = bits.len() as i32;
            while x < n {
                bits[x as usize] += val;
                x += x & -x;
            }
        };
        let query = |mut x: i32, bits: &Vec<i32>| {
            x += 1;
            let mut ans = 0;
            while x > 0 {
                ans += bits[x as usize];
                x -= x & -x;
            }
            ans
        };
        let kth_element = |mut k: i32, bits: &Vec<i32>| {
            let n = bits.len() as i32;
            let (mut total, mut pos) = (0, 0);
            for i in (0..32 - (n - 1).leading_zeros()).rev() {
                if pos + (1 << i) < n && total + bits[(pos + (1 << i)) as usize] < k {
                    total += bits[(pos + (1 << i)) as usize];
                    pos += 1 << i;
                }
            }
            pos
        };
        for &x in &nums {
            update(val2idx[&x] as i32, 1, &mut bits);
        }
        let mut ans = Vec::with_capacity(queries.len());
        let mut total = nums.len() as i32;
        let mut p = p as i64;
        for q in queries {
            update(val2idx[&q[0]] as i32, 1, &mut bits);
            total += 1;
            let i = kth_element(total - q[1] + 1, &bits);
            p = pow_mod(p, sorted[i as usize]);
            ans.push(p as i32);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_power_update_1() {
        assert_eq!(
            vec![64, 4096],
            Solution::power_update(vec![2], 4, lc_matrix![[3, 1], [1, 2]])
        );
    }
    #[test]
    pub fn test_power_update_2() {
        assert_eq!(
            vec![1296, 220296870],
            Solution::power_update(vec![7, 5], 6, lc_matrix![[4, 3], [7, 2]])
        );
    }
}
