// 716\. Max Stack
// ===============

// Design a max stack that supports push, pop, top, peekMax and popMax.

// 1.  push(x) -- Push element x onto stack.
// 2.  pop() -- Remove the element on top of the stack and return it.
// 3.  top() -- Get the element on the top.
// 4.  peekMax() -- Retrieve the maximum element in the stack.
// 5.  popMax() -- Retrieve the maximum element in the stack, and remove it. If you find more than one maximum elements, only remove the top-most one.

// **Example 1:**

// MaxStack stack = new MaxStack();
// stack.push(5);
// stack.push(1);
// stack.push(5);
// stack.top(); -> 5
// stack.popMax(); -> 5
// stack.top(); -> 1
// stack.peekMax(); -> 5
// stack.pop(); -> 1
// stack.top(); -> 5

// **Note:**

// 1.  \-1e7 <= x <= 1e7
// 2.  Number of operations won't exceed 10000.
// 3.  The last four operations won't be called when stack is empty.

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Lyft](https://leetcode.ca/tags/#Lyft) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Twitter](https://leetcode.ca/tags/#Twitter)

// @lc code=start
// use std::collections::BTreeMap;

#[derive(Debug)]
struct MaxStack {
    // stack: BTreeMap<usize, i32>,
    // max: BTreeMap<(i32, usize), usize>,
    s: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        // Self {
        //     stack: BTreeMap::new(),
        //     max: BTreeMap::new(),
        // }
        Self { s: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        // let last = self.stack.iter().next_back();
        // let new_index = if let Some((&i, _)) = last { i + 1 } else { 0 };
        // self.stack.insert(new_index, x);
        // self.max.insert((x, new_index), new_index);
        self.s.push(x);
    }

    fn pop(&mut self) -> i32 {
        // let (&i, &v) = self.stack.iter().next_back().unwrap();
        // self.stack.remove(&i);
        // self.max.remove(&(v, i));
        // v
        self.s.pop().unwrap_or(-1)
    }

    fn top(&self) -> i32 {
        // let (_, &v) = self.stack.iter().next_back().unwrap();
        // v
        *self.s.last().unwrap_or(&-1)
    }

    fn peek_max(&self) -> i32 {
        // let (&(v, _), _) = self.max.iter().next_back().unwrap();
        // v
        *self.s.iter().max().unwrap()
    }

    fn pop_max(&mut self) -> i32 {
        // let (&(v, _), &i) = self.max.iter().next_back().unwrap();
        // self.stack.remove(&i);
        // self.max.remove(&(v, i));
        // v
        let max = *self.s.iter().max().unwrap();
        let i = self.s.iter().rposition(|x| *x == max).unwrap();
        self.s.remove(i);
        max
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_stack_1() {
        let mut stk = MaxStack::new();
        stk.push(5);
        stk.push(1);
        stk.push(5);
        assert_eq!(stk.top(), 5);
        assert_eq!(stk.pop_max(), 5);
        assert_eq!(stk.top(), 1);
        assert_eq!(stk.peek_max(), 5);
        assert_eq!(stk.pop(), 1);
        assert_eq!(stk.top(), 5);
    }

    #[test]
    fn test_max_stack_2() {
        let mut stk = MaxStack::new();
        stk.push(5);
        stk.push(1);
        assert_eq!(stk.pop_max(), 5);
        assert_eq!(stk.top(), 1);
    }
}
