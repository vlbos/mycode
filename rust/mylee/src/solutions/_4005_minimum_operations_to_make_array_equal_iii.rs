// [4005. Minimum Operations to Make Array Equal III](https://leetcode.com/problems/minimum-operations-to-make-array-equal-iii/)

// You are given an integer array `nums`.
// In one operation, you may choose **any** element `nums[i]` and perform one of the following:
// * **Multiply** `nums[i]` by an integer `k`, where `k \>= 2`.
// * **Divide** `nums[i]` by an integer `k`, where `2 \<= k \< nums[i]`, provided that `nums[i]` is divisible by `k`.
// Return the **minimum** number of operations required to make all elements of `nums` **equal**.
// **Example 1:**
// **Input:** nums = [6,12,8]
// **Output:** 3
// **Explanation:**
// We can perform following operates to make all numbers to 6:
// * Divide `nums[1] = 12` by 2 to get 6.
// * Divide `nums[2] = 8` by 4 to get 2.
// * Multiply `nums[2] = 2` by 3 to get 6.
// **Example 2:**
// **Input:** nums = [5,15,20]
// **Output:** 2
// **Explanation:**
// We can perform following operates to make all numbers to 5:
// * Divide `nums[1] = 15` by 3 to get 5.
// * Divide `nums[2] = 20` by 4 to get 5.
// **Example 3:**
// **Input:** nums = [7,7,7]
// **Output:** 0
// **Explanation:**
// All elements are already equal, so no operations are needed.
// **Constraints:**
// * `1 \<= nums.length \<= 105`
// * `1 \<= nums[i] \<= 10​​​​​​​9`

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let (c, max_val, n, mut cc) = (
            nums.iter().fold(HashMap::new(), |mut c, &x| {
                *c.entry(x).or_insert(0) += 1;
                c
            }),
            *nums.iter().max().unwrap(),
            nums.len() as i64,
            HashMap::new(),
        );
        for &x in c.keys() {
            for v in (x..=max_val).step_by(x as usize) {
                *cc.entry(v).or_insert(0) += c[&x];
                *cc.entry(x).or_insert(0) += *c.get(&v).unwrap_or(&0);
            }
        }
        cc.insert(1, n);
        n * 2 - *cc.values().max().unwrap()
    }
}

#[allow(dead_code)]
pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_operations_1() {
        assert_eq!(3, Solution::min_operations(vec![6, 12, 8]));
    }
    #[test]
    pub fn test_min_operations_2() {
        assert_eq!(2, Solution::min_operations(vec![5, 15, 20]));
    }
    #[test]
    pub fn test_min_operations_3() {
        assert_eq!(0, Solution::min_operations(vec![7, 7, 7]));
    }
}

// Python Enumerate Every Possible Target + Harmonic Series

// cy171
// Annual Badge 2025
// 18
// Jul 30, 2026
// Python3
// Intuition
// We can always make all elements equal to LCM in at most n operations — one operation per element at most, giving upper bound = n.

// If the target T is not in nums, then every single element needs at least 1 operation, costing at least n total which already meets our upper bound. Therefore:

// The optimal target must always be some value already present in nums.

// For each candidate target T, every other element falls into one of these cases:

// Elements x smaller than T:

// cost = 1 if T % x == 0 (x is divisor of T, multiply once)
// cost = 2 otherwise (need intermediate step)
// Elements x larger than T:

// cost = 1 if x % T == 0 (x is multiple of T, divide once)
// cost = 2 otherwise (need intermediate step)
// Approach
// Precompute two maps using harmonic series trick O(M log M):

// divisor[v] = count of nums elements that are divisors of v
// multiple[v] = count of nums elements that are multiples of v
// Then for each candidate target, derive total cost:

// Count of elements strictly below target
// below = acc - c[num]
// Count of elements strictly above target
// above = n - acc

// Start with worst case (cost 2 for everyone)
// then subtract 1 for each element that can reach in 1 op

// cost_below = 2 * below - (divisor[num] - c[num])
// cost_above = 2 * above - (multiple[num] - c[num])

// total = cost_below + cost_above

// Complexity
// Time complexity:

// O(n + M log M)
// M = max(nums)

// Space complexity:

// O(n)

// Code
// class Solution:
//     def minOperations(self, nums: List[int]) -> int:
//         c = Counter(nums)
//         n = len(nums)
//         max_val = max(nums) + 1
//         divisor = Counter()
//         multiple = Counter()
//         for num in c.keys():
//             for val in range(num, max_val, num):
//                 divisor[val] += c[num]
//                 multiple[num] += c[val]
//         ans = len(nums)
//         acc = 0
//         for num in c.keys():
//             acc += c[num]
//             if num == 1:
//                 continue
//             cost_below = 2 * (acc - c[num]) - divisor[num] + c[num]
//             cost_above = 2 * (n - acc) - multiple[num] + c[num]
//             ans = min(ans, cost_below + cost_above)
//         return ans
// You can skip a lot of step .

// class Solution:
//     def minOperations(self, nums: List[int]) -> int:
//         c, max_val, n, cc = Counter(nums), max(nums) + 1, len(nums), Counter()
//         for num in c.keys():
//             for val in range(num, max_val + 1, num):
//                 cc[val] += c[num]
//                 cc[num] += c[val]
//         cc[1] = n
//         return 2 * n - max(cc.values())

// Intuition
// For any target value greater than 1, every element requires at most two operations to become the target.

// Suppose the current value is x and the target is t.

// If x == t, no operation is required.
// If x divides t, we can multiply x by t / x in one operation.
// If t divides x, we can divide x by x / t in one operation.
// Otherwise, we can always use two operations:
// x→x∗t→t

// First multiply x by t, then divide xt by x.

// The value 1 is a special case:

// 1 can become any target greater than 1 in one multiplication.
// A value greater than 1 cannot become 1, because dividing x by x is forbidden by the condition k < x.
// Therefore, unless all elements are already 1, the final target must be greater than 1.

// We may initially assume that every element requires two operations. For a chosen target t, we then count how many operations can be saved:

// Every occurrence of t saves two operations.
// Every different value comparable with t under divisibility saves one operation.
// The remaining problem is therefore to find all pairs of values where one divides the other.

// Approach
// First, store the frequency of every distinct value in an ordered map.

// Let the sorted distinct values be:

// v
// 0
// ​
//  <v
// 1
// ​
//  <⋯<v
// m−1
// ​

// For every candidate target values[i].first, initialise:

// saving[i] = 2 * frequency[target];
// This accounts for elements already equal to the target.

// Next, for every distinct value d, find all larger existing values divisible by d.

// A direct scan through all pairs would require (O(m^2)) divisibility checks. Instead, we use the fact that every divisible value must be one of:

// 2∗d,3∗d,4∗d,…

// However, we do not enumerate all multiples up to the maximum value. Most of them may not appear in the array.

// Instead, maintain the next possible multiple of d and use lower_bound to jump directly to the first existing value not smaller than that multiple.

// Suppose lower_bound returns an existing value x.

// If x % d == 0, then d and x form a divisible pair.
// Otherwise, the next possible multiple of d after x is:
// (⌊
// d
// x
// ​
//  ⌋+1)d

// For every divisible pair d < x:

// Choosing d as the target allows every occurrence of x to reach it in one operation, so:
// saving[d] += frequency[x];
// Choosing x as the target allows every occurrence of d to reach it in one operation, so:
// saving[x] += frequency[d];
// After processing all divisible pairs, the operation count for target t is:

// 2n−saving[t]

// We initialise the answer as n. This is always achievable by choosing a common multiple greater than every element and multiplying every element once.

// Targets equal to 1 are skipped because values greater than 1 cannot be transformed into 1.

// Here is the code of this version (there is an optimised one below but still cannot guarentee the worst complexity to be nlog(m))

// using ll = long long;

// class Solution {
// public:
//     ll minOperations(vector<int>& nums) {
//         map<ll, ll> freq;

//         for (int x : nums)
//             ++freq[x];

//         ll n = nums.size();

//         if (freq.size() == 1)
//             return 0;

//         vector<pair<ll, ll>> values(freq.begin(), freq.end());

//         ll m = values.size();
//         vector<ll> saving(m);

//         for (ll i = 0; i < m; ++i)
//             saving[i] = values[i].second << 1;

//         for (ll i = 0; i < m; ++i) {
//             ll d = values[i].first;
//             ll next = d * 2;

//             while (next <= values.back().first) {
//                 auto it = lower_bound(
//                     values.begin() + i + 1,
//                     values.end(),
//                     next,
//                     [](const pair<ll, ll>& p, ll value) {
//                         return p.first < value;
//                     }
//                 );

//                 if (it == values.end())
//                     break;

//                 ll j = it - values.begin();
//                 ll x = it->first;

//                 if (x % d == 0) {
//                     saving[i] += values[j].second;
//                     saving[j] += values[i].second;
//                     next = x + d;
//                 } else {
//                     next = (x / d + 1) * d;
//                 }
//             }
//         }

//         ll ans = n;

//         for (ll i = 0; i < m; ++i) {
//             if (values[i].first == 1)
//                 continue;

//             ans = min(ans, (n << 1) - saving[i]);
//         }

//         return ans;
//     }
// };
// Complexity
// Let (m) be the number of distinct values, and let (J) be the total number of lower_bound jumps performed over all distinct values.

// Time complexity:
// O(nlogm+Jlogm)

// Building the ordered frequency map costs (O(n\log m)). Every jump performs one binary search over the sorted distinct values.

// For a fixed value d, the number of jumps is bounded by both the number of remaining distinct values and the number of possible multiples of d:

// J
// d
// ​
//  ≤min(m,⌊
// d
// max(nums)
// ​
//  ⌋)

// Therefore:

// J≤∑
// d is distinct and d≥1
// ​

// ​
//  min(m,⌊
// d
// max(nums)
// ​
//  ⌋)

// The algorithm is usually much faster than checking all pairs because it skips intervals containing no possible multiple. Its theoretical worst case can still reach:

// O(m
// 2
//  logm)

// but this requires an adversarially arranged sparse set of values.

// Space complexity:
// O(m)

// The frequency map, sorted distinct values, and savings array each contain one entry per distinct value.

// Advanced Optimization
// In the basic implementation, every jump uses lower_bound on almost the entire suffix of the sorted distinct values.

// For a fixed divisor candidate d, however, all visited positions are strictly increasing. Once the previous search reaches index j, the next search only needs to consider positions after j.

// Therefore, instead of repeatedly performing a global binary search, we can use galloping search, also called exponential search.

// Suppose the next required multiple is target, and the previous search ended at position pos.

// Start from pos + 1.
// If this value is already at least target, return it immediately.
// Otherwise, examine positions at exponentially increasing distances:
// 1,2,4,8,…

// Once the searched value becomes at least target, perform a binary search only inside the final bounded interval.
// This adapts the search cost to the actual distance between two consecutively visited positions.

// If the next valid position is only one or two indices away, the search takes O(1) time instead of O(log m). A logarithmic cost is paid only when a large section of the sorted values is skipped.

// The value 1 can also be handled separately. Since 1 can become every target greater than 1 in one operation, its frequency can be added directly to the saving of every other candidate target. There is no need to run the multiple-search procedure with d = 1.

// This optimization does not reduce the number of logical jumps, but it reduces the cost of locating each jump.

// Complexity
// Let:

// m be the number of distinct values;
// J_i be the number of positions visited while processing the (i)-th distinct value;
// g_{i,1},g_{i,2},....,g_{i,J_i} be the index gaps between consecutive searches.
// For a fixed value, the visited positions are strictly increasing, so:

// ∑
// k=1
// J
// i
// ​

// ​
//  g
// i,k
// ​
//  ≤m

// A galloping search across a gap of length (g) costs:

// O(log(g+1))

// Therefore, the total search cost for one value is:

// O(∑
// k=1
// J
// i
// ​

// ​
//  log(g
// i,k
// ​
//  +1))

// Because the logarithm is concave, this is bounded by:

// O(J
// i
// ​
//  log(1+
// J
// i
// ​

// m
// ​
//  ))

// Thus, the total time complexity is:

// O(nlogm+∑
// i=1
// m
// ​
//  J
// i
// ​
//  log(1+
// J
// i
// ​

// m
// ​
//  ))

// The first term comes from constructing the ordered frequency map.

// Since:

// J
// i
// ​
//  log(1+
// J
// i
// ​

// m
// ​
//  )=O(m)

// for every (i), the worst-case complexity becomes:

// O(nlogm+m
// 2
//  )

// This improves the previous worst-case bound:

// O(nlogm+m
// 2
//  logm)

// by removing the additional binary-search factor.

// On typical sparse inputs, consecutive searches are often close to each other, so galloping search usually performs substantially fewer comparisons than repeatedly calling lower_bound over the entire remaining suffix.

// Time complexity:
// O(nlogm+∑
// i=1
// m
// ​
//  J
// i
// ​
//  log(1+
// J
// i
// ​

// m
// ​
//  ))

// with a worst-case bound of:

// O(nlogm+m
// 2
//  )

// Space complexity:
// O(m)

// The frequency map, sorted distinct-value array, and saving array each contain at most one entry per distinct value.

// Code
// using ll = long long;

// class Solution {
// public:
//     ll minOperations(vector<int>& nums) {
//         unordered_map<ll, ll> freq;

//         for (int x : nums)
//             ++freq[x];

//         ll n = nums.size();

//         if (freq.size() == 1)
//             return 0;

//         vector<pair<ll, ll>> values;

//         for (auto& [x, count] : freq)
//             values.push_back({x, count});

//         sort(values.begin(), values.end());

//         ll m = values.size();
//         ll mx = values.back().first;

//         vector<ll> saving(m);

//         for (ll i = 0; i < m; ++i)
//             saving[i] = values[i].second * 2;

//         ll first = 0;

//         if (values[0].first == 1) {
//             for (ll i = 1; i < m; ++i)
//                 saving[i] += values[0].second;
//             first = 1;
//         }

//         auto next = [&](ll start, ll target) -> ll {
//             if (start >= m)
//                 return m;

//             if (values[start].first >= target)
//                 return start;

//             ll step = 1;

//             while (start + step < m &&
//                    values[start + step].first < target) {
//                 step <<= 1;
//             }

//             ll left = start + (step >> 1) + 1;
//             ll right = min(m, start + step + 1);

//             return lower_bound(
//                 values.begin() + left,
//                 values.begin() + right,
//                 target,
//                 [](const pair<ll, ll>& p, ll value) {
//                     return p.first < value;
//                 }
//             ) - values.begin();
//         };

//         for (ll i = first; i < m; ++i) {
//             ll d = values[i].first, nextmul = d * 2, start = i + 1;

//             while (nextmul <= mx) {
//                 ll j = next(start, nextmul);

//                 if (j == m) break;

//                 ll x = values[j].first;
//                 if (x % d == 0) {
//                     saving[i] += values[j].second;
//                     saving[j] += values[i].second;
//                 }

//                 nextmul = (x / d + 1) * d;
//                 start = j + 1;
//             }
//         }

//         ll ans = n;

//         for (ll i = first; i < m; ++i)
//             ans = min(ans, (n<<1) - saving[i]);

//         return ans;
//     }
// };

// pollard rho abuse

// leetgoat_dot_io
// Aug LeetCoding Challenge
// 32
// Jul 29, 2026
// Python3
// hehe

// Code
// import random
// from math import gcd

// def isPrime(n):
//     if n < 2:
//         return False
//     smallPrimes = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
//     for p in smallPrimes:
//         if n % p == 0:
//             return n == p

//     d = n - 1
//     s = 0
//     while d & 1 == 0:
//         d >>= 1
//         s += 1

//     for a in (2, 325, 9375, 28178, 450775, 9780504, 1795265022):
//         if a % n == 0:
//             continue
//         x = pow(a, d, n)
//         if x == 1 or x == n - 1:
//             continue
//         for _ in range(s - 1):
//             x = (x * x) % n
//             if x == n - 1:
//                 break
//         else:
//             return False
//     return True

// def pollardRho(n):
//     if n & 1 == 0:
//         return 2
//     if n % 3 == 0:
//         return 3
//     while True:
//         c = random.randrange(1, n)
//         f = lambda x: (x * x + c) % n
//         x = random.randrange(0, n)
//         y = x
//         d = 1
//         while d == 1:
//             x = f(x)
//             y = f(f(y))
//             d = gcd(abs(x - y), n)
//         if d != n:
//             return d

// # Prime factorization
// # Output shape: [(p1, e1), (p2, e2), ...] where n = Π (pi ** ei)
// # Time: expected ~O(n^(1/4) * log n), randomized
// # Supports: integers up to 2^64 (≈ 1.8e19)
// def primeFactorize(n):
//     factors = {}

//     def dfs(m):
//         if m == 1:
//             return
//         if isPrime(m):
//             factors[m] = factors.get(m, 0) + 1
//             return
//         d = pollardRho(m)
//         dfs(d)
//         dfs(m // d)

//     for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
//         if n % p == 0:
//             e = 0
//             while n % p == 0:
//                 n //= p
//                 e += 1
//             factors[p] = factors.get(p, 0) + e

//     if n > 1:
//         dfs(n)

//     return sorted(factors.items())

// # All positive divisors of n
// # Output shape: [d1, d2, ...]
// # Time: expected ~O(n^(1/4) * log n + D), where D = number of divisors
// def allFactors(n):
//     primeFactors = primeFactorize(n)
//     res = [1]
//     for p, e in primeFactors:
//         cur = []
//         mul = 1
//         for _ in range(e):
//             mul *= p
//             for v in res:
//                 cur.append(v * mul)
//         res += cur
//     return res

// class Solution:
//     def minOperations(self, nums: List[int]) -> int:
//         # for each value, check how many values are factors of this (They take 1 op), check how many are multiples (they take 1 op), check how many are equal (they take 0 ops), and everything else takes 2 ops

//         res = len(nums) # worst case multiply everything to their lcm

//         c = Counter(nums)

//         facToMultiple = Counter()

//         for v in nums:
//             allFacs = allFactors(v)
//             for fac in allFacs:
//                 if fac == v:
//                     continue
//                 facToMultiple[fac] += 1

//         if max(nums) == 1:
//             return 0

//         for v in nums:
//             # but we cannot set everything to 1
//             if v == 1:
//                 continue
//             allFacs = allFactors(v)
//             same = c[v]
//             smaller = 0
//             for fac in allFacs:
//                 if fac != v:
//                     smaller += c[fac]
//             bigger = facToMultiple[v]
//             othersNonOnes = len(nums) - same - smaller - bigger
//             resHere = (2 * othersNonOnes) + bigger + smaller
//             res = min(res, resHere)

//         return res
