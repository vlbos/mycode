// [3416\. Subsequences with a Unique Middle Mode II 🔒](https://leetcode.com/problems/subsequences-with-a-unique-middle-mode-ii)
// ==============================================================================================================================

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// Description
// -----------

// Given an integer array `nums`, find the number of subsequences of size 5 of `nums` with a **unique middle mode**.

// Since the answer may be very large, return it **modulo** `109 + 7`.

// A **mode** of a sequence of numbers is defined as the element that appears the **maximum** number of times in the sequence.

// A sequence of numbers contains a **unique mode** if it has only one mode.

// A sequence of numbers `seq` of size 5 contains a **unique middle mode** if the _middle element_ (`seq[2]`) is a **unique mode**.

// **Example 1:**

// **Input:** nums = \[1,1,1,1,1,1\]

// **Output:** 6

// **Explanation:**

// `[1, 1, 1, 1, 1]` is the only subsequence of size 5 that can be formed from this list, and it has a unique middle mode of 1.

// **Example 2:**

// **Input:** nums = \[1,2,2,3,3,4\]

// **Output:** 4

// **Explanation:**

// `[1, 2, 2, 3, 4]` and `[1, 2, 3, 3, 4]` have unique middle modes because the number at index 2 has the greatest frequency in the subsequence. `[1, 2, 2, 3, 3]` does not have a unique middle mode because 2 and 3 both appear twice in the subsequence.

// **Example 3:**

// **Input:** nums = \[0,1,2,3,4,5,6,7,8\]

// **Output:** 0

// **Explanation:**

// There does not exist a subsequence of length 5 with a unique middle mode.

// **Constraints:**

// *   `5 <= nums.length <= 105`
// *   `-109 <= nums[i] <= 109`

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn subsequences_with_middle_mode( nums: Vec<i32>) -> i32 {
        const MOD:i64=1_000_000_007;
        use std::collections::HashMap;
        let n=nums.len();
        let mut p=HashMap::new();
        let mut s=nums.iter().fold(HashMap::new(),|mut s,&x| {*s.entry(x).or_insert(0)+=1;s});
        let mut ss=s.values().fold(0,|s,&x| (s+x as i64*x as i64)%MOD);
        let (mut pss,mut spp,mut pp,mut ps)=(0,0,0,0);
        let nc2=|n:i64|{n*(n-1)/2%MOD};
        let mut ans=0;
        for (i,&a) in nums.iter().enumerate(){
            let (mut sa, pa)=(s[&a] as i64,*p.get(&a).unwrap_or(&0) as i64);
            pss=(pss+pa*(-sa*sa+(sa-1)*(sa-1)))%MOD;
            spp=(spp-pa*pa)%MOD;
            ss=(ss-sa*sa+(sa-1)*(sa-1))%MOD;
            ps=(ps-pa)%MOD;
            s.entry(a).and_modify(|v|{*v-=1;});
            sa=s[&a] as i64;
            let ( l, r)=(i as i64,(n-i-1) as i64);
            ans=(ans+nc2(l)*nc2(r))%MOD;
            ans=(ans-nc2(l-pa)*nc2(r-sa))%MOD;
            let (pss_,spp_,pp_,ss_,ps_,p_,s_)=((pss-pa*sa*sa)%MOD,(spp-sa*pa*pa)%MOD,(pp-pa*pa)%MOD,(ss-sa*sa)%MOD,(ps-pa*sa)%MOD,l-pa,r-sa);
            let mut su=0;
            su=(su+ps_*(pa*(r-sa)))%MOD;
            su=(su+pss_*(-pa))%MOD;
            su=(su+ps_*(sa*(l-pa)))%MOD;
            su=(su+spp_*(-sa))%MOD;
            su=(su+(pp_-p_)*sa*(r-sa)/2)%MOD;
            su=(su+(ss_-s_)*pa*(l-pa)/2)%MOD;
            ans=(ans-su+MOD)%MOD;
            pss=(pss+sa*sa)%MOD;
            spp=(spp+sa*(-pa*pa+(pa+1)*(pa+1)))%MOD;
            pp=(pp-pa*pa+(pa+1)*(pa+1))%MOD;
            ps=(ps+sa)%MOD;
            *p.entry(a).or_insert(0)+=1;
        }
        ((ans+MOD)%MOD) as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_subsequences_with_middle_mode_1() {
        assert_eq!(
            6,
            Solution::subsequences_with_middle_mode(vec![1, 1, 1, 1, 1, 1])
        );
    }
    #[test]
    pub fn test_subsequences_with_middle_mode_2() {
        assert_eq!(
            4,
            Solution::subsequences_with_middle_mode(vec![1, 2, 2, 3, 3, 4])
        );
    }
}

// int (vector<int>& nums) {
// Time:  O(n)
// Space: O(n)

// freq table, prefix sum, combinatorics
// class Solution {
// public:
//     int subsequencesWithMiddleMode(vector<int>& nums) {
//         typedef __int128 int128_t;

//         const auto& nC2 = [](int128_t x) {
//             return x * (x - 1) / 2;
//         };

//         static const int MOD = 1e9 + 7;
//         int128_t result = 0;
//         unordered_map<int, int128_t> left, right;
//         for (const auto& x : nums) {
//             ++right[x];
//         }

//         int128_t left_x_sq = 0; // sum(left[x]^2 for x != v)
//         int128_t right_x_sq = 0; // sum(right[x]^2f or x != v)
//         int128_t left_x_right_x = 0;  // sum(left[x]*right[x] for x != v)
//         int128_t left_x_sq_right_x = 0;  // sum(left[x]^2*right[x] for x != v)
//         int128_t left_x_right_x_sq = 0;  //sum(left[x]*right[x]^2 for x != v)
//         for (const auto& [_, v] : right) {
//             right_x_sq += v * v;
//         }
//         for (int i = 0; i < size(nums); ++i) {
//             const int v = nums[i];
//             left_x_sq -= left[v] * left[v];
//             right_x_sq -= right[v]* right[v];
//             left_x_right_x -= left[v] * right[v];
//             left_x_sq_right_x -= left[v] * left[v] * right[v];
//             left_x_right_x_sq -= left[v] * right[v] * right[v];
//             --right[v];

//             const int l = i;
//             const int r = size(nums) - (i + 1);
//             // all possibles
//             result += nC2(l) * nC2(r);
//             // only mid is a
//             result -= nC2(l - left[v]) * nC2(r - right[v]);
//             // bb/a/ac
//             // sum((left[x]*(left[x]-1)//2)*right[v]*((r-right[v])-right[x]) for x != v)
//             result -= ((left_x_sq - (l - left[v])) * (r - right[v]) - (left_x_sq_right_x - left_x_right_x)) * right[v] / 2;
//             // ac/a/bb
//             // sum(left[v]*((l-left[v])-left[x])*(right[x]*(right[x]-1)//2) for x != v)
//             result -= ((right_x_sq - (r - right[v])) * (l - left[v]) - (left_x_right_x_sq - left_x_right_x) ) *left[v] / 2;
//             // ab/a/bc
//             // sum(left[v]*left[x]*right[x]*((r-right[v])-right[x]) for x != v)
//             result -= left[v] * left_x_right_x * (r - right[v]) - left[v] * left_x_right_x_sq;
//             // bc/a/ab
//             // sum(left[x]*((l-left[v])-left[x])*right[v]*right[x] for x != v)
//             result -= right[v] * left_x_right_x * (l - left[v]) - right[v] * left_x_sq_right_x;
//             // bb/a/ab
//             // sum((left[x]*(left[x]-1)//2)*right[v]*right[x] for x != v)
//             result -= right[v] * (left_x_sq_right_x - left_x_right_x) / 2;
//             // ab/a/bb
//             // sum((right[x]*(right[x]-1)//2)*left[v]*left[x] for x != v)
//             result -= left[v] * (left_x_right_x_sq - left_x_right_x) / 2;

//             ++left[v];
//             left_x_sq += left[v] * left[v];
//             right_x_sq += right[v] * right[v];
//             left_x_right_x += left[v] * right[v];
//             left_x_sq_right_x += left[v] * left[v] * right[v];
//             left_x_right_x_sq += left[v] * right[v] * right[v];
//         }

//         return result % MOD;
//     }
// };



// // Recall from solution 1 that after counting all the subsequences with `a` as
// // the middle mode number, we need to subtract the cases where `a` is not a
// // unique mode or not a mode.
// //
// // To avoid the need of looping through all numbers that are not `a`, we can
// // maintain the sums that are not related to `a` in the loop.
// //
// // So, during the simplification of the formula, keep the running sums of
// // pss, spp, pp, ss, and ps as the first item.
// // (for cleaner notation, abbreviate p[b] and s[b] to just p and s)
// //
// //   sum(b != a) (p[a] * p * s) * (r - sa - s)
// //             + (sa * s * p) * (l - p[a] - p)
// //             + (p, 2) * sa * (r - sa)
// //             + (s, 2) * p[a] * (l - p[a])
// //
// //   sum(b != a) (p * s) * (p[a] * (r - sa)) + (p * s^2) * (-p[a])
// //             + (s * p) * (sa * (l - p[a])) + (s * p^2) * (-sa)
// //             + (p^2 - p) * (sa * (r - sa) / 2)
// //             + (s^2 - s) * (p[a] * (l - p[a]) / 2)

// class Solution {
//  public:
//   // Same as 3395. Subsequences with a Unique Middle Mode I
//   int subsequencesWithMiddleMode(vector<int>& nums) {
//     int ans = 0;
//     unordered_map<int, int> p;  // prefix counter
//     unordered_map<int, int> s;  // suffix counter\

//     for (const int num : nums)
//       ++s[num];

//     long pss = 0;
//     long spp = 0;
//     long pp = 0;
//     long ss = 0;
//     long ps = 0;

//     for (const auto& [_, freq] : s)
//       ss = (ss + static_cast<long>(freq) * freq) % kMod;

//     for (int i = 0; i < nums.size(); ++i) {
//       const int a = nums[i];
//       long sa = s[a];
//       const long pa = p[a];

//       // Update running sums after decrementing sa.
//       pss = (pss + pa * (-sa * sa + (sa - 1) * (sa - 1))) % kMod;
//       spp = (spp - pa * pa) % kMod;  // (-sa + (sa - 1)) * pa * pa
//       ss = (ss - sa * sa + (sa - 1) * (sa - 1)) % kMod;
//       ps = (ps - pa) % kMod;  // -pa * (-sa + (sa - 1))

//       sa = --s[a];

//       const int l = i;
//       const int r = nums.size() - i - 1;

//       // Start with all possible subsequences with `a` as the middle number.
//       ans = (ans + nC2(l) * nC2(r)) % kMod;

//       // Minus cases where frequency of `a` is 1, so it's not a mode.
//       ans = (ans - nC2(l - pa) * nC2(r - sa)) % kMod;

//       // Minus the values where `b != a`.
//       const long pss_ = (pss - pa * sa * sa) % kMod;
//       const long spp_ = (spp - sa * pa * pa) % kMod;
//       const long pp_ = (pp - pa * pa) % kMod;
//       const long ss_ = (ss - sa * sa) % kMod;
//       const long ps_ = (ps - pa * sa) % kMod;
//       const long p_ = l - pa;
//       const long s_ = r - sa;

//       // Minus cases where `a` is not a "unique" mode or not a mode.
//       long subtract = 0;
//       subtract = (subtract + ps_ * (pa * (r - sa))) % kMod;
//       subtract = (subtract + pss_ * (-pa)) % kMod;
//       subtract = (subtract + ps_ * (sa * (l - pa))) % kMod;
//       subtract = (subtract + spp_ * (-sa)) % kMod;
//       subtract = (subtract + (pp_ - p_) * sa * (r - sa) / 2) % kMod;
//       subtract = (subtract + (ss_ - s_) * pa * (l - pa) / 2) % kMod;
//       ans = (ans - subtract + kMod) % kMod;

//       // Update running sums after incrementing p[a].
//       pss = (pss + sa * sa) % kMod;  // (-pa + (pa + 1)) * sa * sa
//       spp = (spp + sa * (-pa * pa + (pa + 1) * (pa + 1))) % kMod;
//       pp = (pp - pa * pa + (pa + 1) * (pa + 1)) % kMod;
//       ps = (ps + sa) % kMod;  // (-pa + (pa + 1)) * sa

//       ++p[a];
//     }

//     return (ans + kMod) % kMod;
//   }

//  private:
//   static constexpr int kMod = 1'000'000'007;

//   // Returns C(n, 2)
//   long nC2(long n) {
//     return n * (n - 1) / 2 % kMod;
//   }
// };