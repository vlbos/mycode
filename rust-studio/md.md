3958. Minimum Cost to Split into Ones II 
[ Skip to content ](#3958-minimum-cost-to-split-into-ones-ii)
[ ](<https://github.com/doocs/leetcode/edit/main/solution/3900-3999/3958.Minimum Cost to Split into Ones II/README_EN.md>) [ ](<https://github.com/doocs/leetcode/raw/main/solution/3900-3999/3958.Minimum Cost to Split into Ones II/README_EN.md>) # [3958. Minimum Cost to Split into Ones II 🔒](https://leetcode.com/problems/minimum-cost-to-split-into-ones-ii)
[![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)
## Description
You are given an integer `n`.
In one operation, you may split an integer `x` into two positive integers `a` and `b` such that `a + b = x`.
The cost of this operation is `a \* b`.
Return the **minimum** total cost required to split the integer `n` into `n` ones.
 
**Example 1:**
**Input:** n = 3
**Output:** 3
**Explanation:**
One optimal set of operations is:
|`x`|`a`|`b`|`a + b`|`a \* b`|Cost|
|3|1|2|3|2|2|
|2|1|1|2|1|1|
Thus, the minimum total cost is `2 + 1 = 3`.
**Example 2:**
**Input:** n = 4
**Output:** 6
**Explanation:​​​​​​​**
One optimal set of operations is:
|`x`|`a`|`b`|`a + b`|`a \* b`|Cost|
|4|2|2|4|4|4|
|2|1|1|2|1|1|
Thus, the minimum total cost is `4 + 1 + 1 = 6`.
 
**Constraints:**
* `1 \<= n \<= 5 \* 107` ## Solutions
### Solution 1: Mathematics
To minimize the cost, we should first split \\(n\\) into \\(1\\) and \\(n - 1\\), which costs \\(n - 1\\); then split \\(n - 1\\) into \\(1\\) and \\(n - 2\\), which costs \\(n - 2\\). Following this pattern, the total cost is accumulated as \\(1 + 2 + \\dots + (n - 1) = \\frac{n \\times (n - 1)}{2}\\).
The time complexity is \\(O(1)\\), and the space complexity is \\(O(1)\\).
Python3JavaC++GoTypeScript
|
```
123
```
|
```
`classSolution:defminCost(self,n:int)-\>int:returnn\*(n-1)//2`
```
|
|
```
12345
```
|
```
`classSolution{publiclongminCost(intn){return1L\*n\*(n-1)/2;}}`
```
|
|
```
123456
```
|
```
`classSolution{public:longlongminCost(intn){return1LL\*n\*(n-1)/2;}};`
```
|
|
```
123
```
|
```
`funcminCost(nint)int64{returnint64(n\*(n-1)/2)}`
```
|
|
```
123
```
|
```
`functionminCost(n:number):number{return(n\*(n-1))/2;}`
```
|
GitHub Was this page helpful?
Thanks for your feedback!
Thanks for your feedback! Help us improve this page.
## Comments
Back to top