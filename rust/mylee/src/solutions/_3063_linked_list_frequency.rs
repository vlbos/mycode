# [3063. Linked List Frequency](https://leetcode.com/problems/linked-list-frequency)

[

## Description

Given the head of a linked list containing k distinct elements, return a linked list of length k containing the frequency of each distinct element in the given linked list in any order.

&nbsp;
Example 1: 


Input:   head = [1,1,1,2,2,3] 

Output:   [3,2,1] 

Explanation:  There are 3 distinct elements in the list. The frequency of 1 is 3, the frequency of 2 is 2 and the frequency of 3 is 1. Hence, we return 3 -&gt; 2 -&gt; 1.

Note that 1 -&gt; 2 -&gt; 3, 1 -&gt; 3 -&gt; 2, 2 -&gt; 1 -&gt; 3, 2 -&gt; 3 -&gt; 1, and 3 -&gt; 1 -&gt; 2 are also valid answers.


Example 2: 


Input:   head = [1,1,2,2,2] 

Output:   [2,3] 

Explanation:  There are 2 distinct elements in the list. The frequency of 1 is 2 and the frequency of 2 is 3. Hence, we return 2 -&gt; 3.


Example 3: 


Input:   head = [6,5,4,3,2,1] 

Output:   [1,1,1,1,1,1] 

Explanation:  There are 6 distinct elements in the list. The frequency of each of them is 1. Hence, we return 1 -&gt; 1 -&gt; 1 -&gt; 1 -&gt; 1 -&gt; 1.


&nbsp;
Constraints:


	The number of nodes in the list is in the range [1, 105].
	1 &lt;= Node.val &lt;= 105


## Solutions

### Solution 1: Hash Table

We use a hash table `cnt` to record the occurrence times of each element value in the linked list, then traverse the values of the hash table to construct a new linked list.

The time complexity is $O(n)$, and the space complexity is $O(n)$, where $n$ is the length of the linked list.

<!-- tabs:start -->

```python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def frequenciesOfElements(self, head: Optional[ListNode]) -> Optional[ListNode]:
        cnt = Counter()
        while head:
            cnt[head.val] += 1
            head = head.next
        dummy = ListNode()
        for val in cnt.values():
            dummy.next = ListNode(val, dummy.next)
        return dummy.next
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
    public ListNode frequenciesOfElements(ListNode head) {
        Map<Integer, Integer> cnt = new HashMap<>();
        for (; head != null; head = head.next) {
            cnt.merge(head.val, 1, Integer::sum);
        }
        ListNode dummy = new ListNode();
        for (int val : cnt.values()) {
            dummy.next = new ListNode(val, dummy.next);
        }
        return dummy.next;
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
    ListNode* frequenciesOfElements(ListNode* head) {
        unordered_map<int, int> cnt;
        for (; head; head = head->next) {
            cnt[head->val]++;
        }
        ListNode* dummy = new ListNode();
        for (auto& [_, val] : cnt) {
            dummy->next = new ListNode(val, dummy->next);
        }
        return dummy->next;
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
func frequenciesOfElements(head *ListNode) *ListNode {
	cnt := map[int]int{}
	for ; head != nil; head = head.Next {
		cnt[head.Val]++
	}
	dummy := &ListNode{}
	for _, val := range cnt {
		dummy.Next = &ListNode{val, dummy.Next}
	}
	return dummy.Next
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

function frequenciesOfElements(head: ListNode | null): ListNode | null {
    const cnt: Map<number, number> = new Map();
    for (; head; head = head.next) {
        cnt.set(head.val, (cnt.get(head.val) || 0) + 1);
    }
    const dummy = new ListNode();
    for (const val of cnt.values()) {
        dummy.next = new ListNode(val, dummy.next);
    }
    return dummy.next;
}
```

<!-- tabs:end -->

<!-- end -->
