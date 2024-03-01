# [3062. Winner of the Linked List Game](https://leetcode.com/problems/winner-of-the-linked-list-game)




## Description

You are given the head of a linked list of even length containing integers.

Each odd-indexed node contains an odd integer and each even-indexed node contains an even integer.

We call each even-indexed node and its next node a pair, e.g., the nodes with indices 0 and 1 are a pair, the nodes with indices 2 and 3 are a pair, and so on.

For every pair, we compare the values of the nodes in the pair:


	If the odd-indexed node is higher, the &quot;Odd&quot; team gets a point.
	If the even-indexed node is higher, the &quot;Even&quot; team gets a point.


Return the name of the team with the higher points, if the points are equal, return &quot;Tie&quot;.

&nbsp;
Example 1: 


Input:   head = [2,1] 

Output:   &quot;Even&quot; 

Explanation:  There is only one pair in this linked list and that is (2,1). Since 2 &gt; 1, the Even team gets the point.

Hence, the answer would be &quot;Even&quot;.


Example 2: 


Input:   head = [2,5,4,7,20,5] 

Output:   &quot;Odd&quot; 

Explanation:  There are 3 pairs in this linked list. Let&#39;s investigate each pair individually:

(2,5) -&gt; Since 2 &lt; 5, The Odd team gets the point.

(4,7) -&gt; Since 4 &lt; 7, The Odd team gets the point.

(20,5) -&gt; Since 20 &gt; 5, The Even team gets the point.

The Odd team earned 2 points while the Even team got 1 point and the Odd team has the higher points.

Hence, the answer would be &quot;Odd&quot;.


Example 3: 


Input:   head = [4,5,2,1] 

Output:   &quot;Tie&quot; 

Explanation:  There are 2 pairs in this linked list. Let&#39;s investigate each pair individually:

(4,5) -&gt; Since 4 &lt; 5, the Odd team gets the point.

(2,1) -&gt; Since 2 &gt; 1, the Even team gets the point.

Both teams earned 1 point.

Hence, the answer would be &quot;Tie&quot;.


&nbsp;
Constraints:


	The number of nodes in the list is in the range [2, 100].
	The number of nodes in the list is even.
	1 &lt;= Node.val &lt;= 100
	The value of each odd-indexed node is odd.
	The value of each even-indexed node is even.


## Solutions

### Solution 1: Simulation

Traverse the linked list, each time taking out two nodes, compare their values, and then update the scores of odd and even numbers based on the comparison results. Finally, compare the scores of odd and even numbers and return the result.

The time complexity is $O(n)$, where $n$ is the length of the linked list. The space complexity is $O(1)$.

<!-- tabs:start -->

```python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def gameResult(self, head: Optional[ListNode]) -> str:
        odd = even = 0
        while head:
            a = head.val
            b = head.next.val
            odd += a < b
            even += a > b
            head = head.next.next
        if odd > even:
            return "Odd"
        if odd < even:
            return "Even"
        return "Tie"
```

```java
/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public String gameResult(ListNode head) {
        int odd = 0, even = 0;
        for (; head != null; head = head.next.next) {
            int a = head.val;
            int b = head.next.val;
            odd += a < b ? 1 : 0;
            even += a > b ? 1 : 0;
        }
        if (odd > even) {
            return "Odd";
        }
        if (odd < even) {
            return "Even";
        }
        return "Tie";
    }
}
```

```cpp
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    string gameResult(ListNode* head) {
        int odd = 0, even = 0;
        for (; head != nullptr; head = head->next->next) {
            int a = head->val;
            int b = head->next->val;
            odd += a < b;
            even += a > b;
        }
        if (odd > even) {
            return "Odd";
        }
        if (odd < even) {
            return "Even";
        }
        return "Tie";
    }
};
```

```go
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func gameResult(head *ListNode) string {
	var odd, even int
	for ; head != nil; head = head.Next.Next {
		a, b := head.Val, head.Next.Val
		if a < b {
			odd++
		}
		if a > b {
			even++
		}
	}
	if odd > even {
		return "Odd"
	}
	if odd < even {
		return "Even"
	}
	return "Tie"
}
```

```ts
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function gameResult(head: ListNode | null): string {
    let [odd, even] = [0, 0];
    for (; head; head = head.next.next) {
        const [a, b] = [head.val, head.next.val];
        odd += a < b ? 1 : 0;
        even += a > b ? 1 : 0;
    }
    if (odd > even) {
        return 'Odd';
    }
    if (odd < even) {
        return 'Even';
    }
    return 'Tie';
}
```

<!-- tabs:end -->

<!-- end -->
