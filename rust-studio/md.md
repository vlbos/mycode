4042. Valid K-Unique Subarrays II 🔒 - LeetCode Wiki
[ Skip to content ](#4042-valid-k-unique-subarrays-ii)
[ ](<https://github.com/doocs/leetcode/edit/main/solution/4000-4099/4042.Valid K-Unique Subarrays II/README_EN.md>) [ ](<https://github.com/doocs/leetcode/raw/main/solution/4000-4099/4042.Valid K-Unique Subarrays II/README_EN.md>) # [4042. Valid K-Unique Subarrays II 🔒](https://leetcode.com/problems/valid-k-unique-subarrays-ii)
[![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)
## Description
You are given an integer array `nums` of length `n` and an integer `k`.
You are also given integers `l0` and `r0`, which define the first query, and an integer `q`, representing the total number of queries to process.
A **subarray** `nums[li..ri]` is considered **valid** if:
* It contains **exactly** `k` **distinct** numbers, and
* Every distinct number in it occurs an **even** number of times.
For query 0, set `l0 = l0` and `r0 = r0`.
Let `ansi` denote the result of the `ith` query, where `ansi = 1` if `nums[li..ri]` is **valid**, and `ansi = 0` otherwise.
For each `i \> 0`, generate the next query as follows:
* If `ansi-1 = 1`, set `gi-1 = li-1 + ri-1`. Otherwise, set `gi-1 = ri-1 - li-1`.
* Compute `li = (li-1 XOR gi-1) % n` and `ri = (ri-1 XOR gi-1) % n`.
* If `li \> ri`, swap them.
Return a boolean array `ans`, where `ans[i]` is `true` if `ansi = 1`, and `false` otherwise.
**Example 1:**
**Input:** nums = [1,2,2,1], k = 2, l0 = 1, r0 = 2, q = 2
**Output:** [false,true]
**Explanation:**
|`i`|`[li, ri]`|Subarray|Distinct numbers|Counts|Validity check|`ans[i]`|`[li+1, ri+1]`|
|0|[1, 2]|[2, 2]|{2} → 1|{2:2}|`false`: The subarray contains fewer than `k` distinct numbers.|`ans0 = 0`|`g0 = 2 - 1 = 1
l1 = (1 XOR 1) % 4 = 0
r1 = (2 XOR 1) % 4 = 3`|
|1|[0, 3]|[1, 2, 2, 1]|{1,2} → 2|{1:2,2:2}|`true`: The subarray contains exactly `k` distinct numbers, each occurring an even number of times.|`ans1 = 1`|-|
Thus, `ans = [false, true]`.
**Example 2:**
**Input:** nums = [1,2,3,3,4], k = 1, l0 = 2, r0 = 3, q = 2
**Output:** [true,false]
**Explanation:**
|`i`|`[li, ri]`|Subarray|Distinct numbers|Counts|Validity check|`ans[i]`|`[li+1, ri+1]`|
|0|[2, 3]|[3, 3]|{3} → 1|{3:2}|`true`: The subarray contains exactly `k` distinct numbers, each occurring an even number of times.|`ans0 = 1`|`g0 = 2 + 3 = 5
l1 = (2 XOR 5) % 5 = 7 % 5 = 2
r1 = (3 XOR 5) % 5 = 6 % 5 = 1`
Since `l1 \> r1`, swap them to obtain `[l1, r1] = [1, 2]`.|
|1|[1, 2]|[2, 3]|{2,3} → 2|{2:1,3:1}|`false`: The subarray contains 2 distinct numbers instead of exactly `k = 1`.|`ans1 = 0`|-|
Thus, `ans = [true, false]`.
**Constraints:**
* `2 \<= n == nums.length \<= 5 × 105`
* `1 \<= nums[i] \<= 5 × 105`
* `1 \<= k \<= n`
* `0 \<= l0 \< r0 \<= n - 1`
* `1 \<= q \<= 5 × 105` ## Solutions
### Solution 1
Python3JavaC++Go
|
```
1
```
|
```
``
```
|
|
```
1
```
|
```
``
```
|
|
```
1
```
|
```
``
```
|
|
```
1
```
|
```
``
```
|
GitHub Was this page helpful?
Thanks for your feedback!
Thanks for your feedback! Help us improve this page.
## Comments
Back to top