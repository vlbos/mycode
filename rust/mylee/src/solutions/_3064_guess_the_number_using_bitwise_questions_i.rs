# [3064. Guess the Number Using Bitwise Questions I](https://leetcode.com/problems/guess-the-number-using-bitwise-questions-i)



## Description

There is a number n that you have to find.

There is also a pre-defined API int commonSetBits(int num), which returns the number of bits where both n and num are 1 in that position of their binary representation. In other words, it returns the number of set bits in n &amp; num, where &amp; is the bitwise AND operator.

Return the number n.

&nbsp;
Example 1: 


Input:   n = 31 

Output:   31 

Explanation:  It can be proven that it&#39;s possible to find 31 using the provided API.


Example 2: 


Input:   n = 33 

Output:   33 

Explanation:  It can be proven that it&#39;s possible to find 33 using the provided API.


&nbsp;
Constraints:


	1 &lt;= n &lt;= 230 - 1
	0 &lt;= num &lt;= 230 - 1
	If you ask for some num out of the given range, the output wouldn&#39;t be reliable.


## Solutions

### Solution 1: Enumeration

We can enumerate the powers of 2, and then call the `commonSetBits` method. If the return value is greater than 0, it means that the corresponding bit in the binary representation of `n` is 1.

The time complexity is $O(\log n)$, where $n \le 2^{30}$ in this problem. The space complexity is $O(1)$.

<!-- tabs:start -->

```python
# Definition of commonSetBits API.
# def commonSetBits(num: int) -> int:


class Solution:
    def findNumber(self) -> int:
        return sum(1 << i for i in range(32) if commonSetBits(1 << i))
```

```java
/**
 * Definition of commonSetBits API (defined in the parent class Problem).
 * int commonSetBits(int num);
 */

public class Solution extends Problem {
    public int findNumber() {
        int n = 0;
        for (int i = 0; i < 32; ++i) {
            if (commonSetBits(1 << i) > 0) {
                n |= 1 << i;
            }
        }
        return n;
    }
}
```

```cpp
/**
 * Definition of commonSetBits API.
 * int commonSetBits(int num);
 */

class Solution {
public:
    int findNumber() {
        int n = 0;
        for (int i = 0; i < 32; ++i) {
            if (commonSetBits(1 << i)) {
                n |= 1 << i;
            }
        }
        return n;
    }
};
```

```go
/**
 * Definition of commonSetBits API.
 * func commonSetBits(num int) int;
 */

func findNumber() (n int) {
	for i := 0; i < 32; i++ {
		if commonSetBits(1<<i) > 0 {
			n |= 1 << i
		}
	}
	return
}
```

```ts
/**
 * Definition of commonSetBits API.
 * var commonSetBits = function(num: number): number {}
 */

function findNumber(): number {
    let n = 0;
    for (let i = 0; i < 32; ++i) {
        if (commonSetBits(1 << i)) {
            n |= 1 << i;
        }
    }
    return n;
}
```

<!-- tabs:end -->

<!-- end -->
