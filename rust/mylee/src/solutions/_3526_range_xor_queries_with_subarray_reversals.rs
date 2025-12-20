// ## [3526\. Range XOR Queries with Subarray Reversals 🔒](https://leetcode.com/problems/range-xor-queries-with-subarray-reversals)

// [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

// ## Description

// You are given an integer array `nums` of length `n` and a 2D integer array `queries` of length `q`,
// where each query is one of the following three types:

// 1.  **Update**: `queries[i] = [1, index, value]`
//     Set `nums[index] = value`.

// 2.  **Range XOR Query**: `queries[i] = [2, left, right]`
//     Compute the bitwise XOR of all elements in the subarray `nums[left...right]`, and record this result.

// 3.  **Reverse Subarray**: `queries[i] = [3, left, right]`
//     Reverse the subarray `nums[left...right]` in place.

// Return *an array of the results of all range XOR queries* in the order they were encountered.

// **Example 1:**

// **Input:** nums = \[1,2,3,4,5\], queries = \[\[2,1,3\],\[1,2,10\],\[3,0,4\],\[2,0,4\]\]

// **Output:** \[5,8\]

// **Explanation:**

// +   **Query** **1****:** `[2, 1, 3]` – Compute XOR of subarray `[2, 3, 4]` resulting in 5.

// +   **Query 2:** `[1, 2, 10]` – Update `nums[2]` to 10, updating the array to `[1, 2, 10, 4, 5]`.

// +   **Query 3:** `[3, 0, 4]` – Reverse the entire array to get `[5, 4, 10, 2, 1]`.

// +   **Query 4:** `[2, 0, 4]` – Compute XOR of subarray `[5, 4, 10, 2, 1]` resulting in 8.

// **Example 2:**

// **Input:** nums = \[7,8,9\], queries = \[\[1,0,3\],\[2,0,2\],\[3,1,2\]\]

// **Output:** \[2\]

// **Explanation:**

// +   **Query 1:** `[1, 0, 3]` – Update `nums[0]` to 3, updating the array to `[3, 8, 9]`.

// +   **Query 2:** `[2, 0, 2]` – Compute XOR of subarray `[3, 8, 9]` resulting in 2.

// +   **Query 3:** `[3, 1, 2]` – Reverse the subarray `[8, 9]` to get `[9, 8]`.

// **Constraints:**

// +   `1 <= nums.length <= 105`
// +   `0 <= nums[i] <= 109`
// +   `1 <= queries.length <= 105`
// +   `queries[i].length == 3​`
// +   `queries[i][0] ∈ {1, 2, 3}​`
// +   If `queries[i][0] == 1`:`​`
//     +   `0 <= index < nums.length​`
//     +   `0 <= value <= 109`
// +   If `queries[i][0] == 2` or `queries[i][0] == 3`:`​`
//     +   `0 <= left <= right < nums.length​`

//  vector<int> get_results(vector<int>& nums, vector<vector<int>>& queries) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn get_results(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::{cell::RefCell, rc::Rc};
        struct Node {
            pub val: i32,
            pub sub_xor: i32,
            pub sz: i32,
            pub rev: bool,
            pub prior: i32,
            pub l: Option<Rc<RefCell<Node>>>,
            pub r: Option<Rc<RefCell<Node>>>,
        }
        impl Node {
            pub fn new(v: i32) -> Self {
                Self {
                    val: v,
                    sub_xor: v,
                    sz: 1,
                    rev: false,
                    prior: rand::random(),
                    l: None,
                    r: None,
                }
            }
        }
        #[inline]
        fn get_size(t: &Option<Rc<RefCell<Node>>>) -> i32 {
            if let Some(v) = t { v.borrow().sz } else { 0 }
        }
        #[inline]
        fn get_xor(t: &Option<Rc<RefCell<Node>>>) -> i32 {
            if let Some(v) = t {
                v.borrow().sub_xor
            } else {
                0
            }
        }
        fn update(t: &Option<Rc<RefCell<Node>>>) {
            if t.is_none() {
                return;
            }
            let sz = 1
                + get_size(&t.as_ref().unwrap().borrow().l)
                + get_size(&t.as_ref().unwrap().borrow().r);
            t.as_ref().unwrap().borrow_mut().sz = sz;
            let sub_xor = t.as_ref().unwrap().borrow().val
                ^ get_xor(&t.as_ref().unwrap().borrow().l)
                ^ get_xor(&t.as_ref().unwrap().borrow().r);
            t.as_ref().unwrap().borrow_mut().sub_xor = sub_xor;
        }
        fn push(t: &Option<Rc<RefCell<Node>>>) {
            if t.is_none() || !t.as_ref().unwrap().borrow().rev {
                return;
            }
            let ll = t.as_ref().unwrap().borrow_mut().l.take();
            let rr = t.as_ref().unwrap().borrow_mut().r.take();
            (
                t.as_ref().unwrap().borrow_mut().r,
                t.as_ref().unwrap().borrow_mut().l,
            ) = (ll, rr);
            if let Some(v) = &t.as_ref().unwrap().borrow().l {
                v.borrow_mut().rev ^= true;
            }
            if let Some(v) = &t.as_ref().unwrap().borrow().r {
                v.borrow_mut().rev ^= true;
            }
            t.as_ref().unwrap().borrow_mut().rev = false;
        }
        fn merge(
            l: &Option<Rc<RefCell<Node>>>,
            r: &Option<Rc<RefCell<Node>>>,
        ) -> Option<Rc<RefCell<Node>>> {
            push(l);
            push(r);
            if l.is_none() || r.is_none() {
                return if l.is_none() { r.clone() } else { l.clone() };
            }
            if l.as_ref().unwrap().borrow().prior > r.as_ref().unwrap().borrow().prior {
                let rr = merge(&l.as_ref().unwrap().borrow().r, &r);
                l.as_ref().unwrap().borrow_mut().r = rr;
                update(l);
                l.clone()
            } else {
                let ll = merge(&l, &r.as_ref().unwrap().borrow().l);
                r.as_ref().unwrap().borrow_mut().l = ll;
                update(r);
                r.clone()
            }
        }
        fn split(
            t: &Option<Rc<RefCell<Node>>>,
            k: i32,
            l: &mut Option<Rc<RefCell<Node>>>,
            r: &mut Option<Rc<RefCell<Node>>>,
        ) {
            if t.is_none() {
                (*l, *r) = (None, None);
                return;
            }
            push(t);

            if get_size(&t.as_ref().unwrap().borrow().l) >= k {
                let mut ll = t.as_ref().unwrap().borrow().l.clone();
                split(&t.as_ref().unwrap().borrow().l, k, l, &mut ll);
                t.as_ref().unwrap().borrow_mut().l = ll;
                *r = t.clone();
            } else {
                let mut rr = t.as_ref().unwrap().borrow().r.clone();
                split(
                    &t.as_ref().unwrap().borrow().r,
                    k - get_size(&t.as_ref().unwrap().borrow().l) - 1,
                    &mut rr,
                    r,
                );
                t.as_ref().unwrap().borrow_mut().r = rr;
                *l = t.clone();
            }
            update(t);
        }
        fn build(nums: Vec<i32>) -> Option<Rc<RefCell<Node>>> {
            nums.into_iter().fold(None, |root, x| {
                merge(&root, &Some(Rc::new(RefCell::new(Node::new(x)))))
            })
        }
        let mut root = build(nums);
        fn update_value(index: i32, val: i32, root: &mut Option<Rc<RefCell<Node>>>) {
            let (mut l, mut r, mut m) = (None, None, None);
            split(root, index, &mut l, &mut r);
            split(&r.clone(), 1, &mut m, &mut r);
            if let Some(v) = &m {
                v.borrow_mut().val = val;
            }
            update(&m);
            *root = merge(&l, &merge(&m, &r));
        }
        fn range_xor(left: i32, right: i32, root: &mut Option<Rc<RefCell<Node>>>) -> i32 {
            let (mut l, mut r, mut m) = (None, None, None);
            split(root, left, &mut l, &mut r);
            split(&r.clone(), right - left + 1, &mut m, &mut r);
            let ans = get_xor(&m);
            *root = merge(&merge(&l, &m), &r);
            ans
        }
        fn reverse_range(left: i32, right: i32, root: &mut Option<Rc<RefCell<Node>>>) {
            let (mut l, mut r, mut m) = (None, None, None);
            split(root, left, &mut l, &mut r);
            split(&r.clone(), right - left + 1, &mut m, &mut r);
            if let Some(v) = &m {
                v.borrow_mut().rev ^= true;
            }
            *root = merge(&merge(&l, &m), &r);
        }
        let mut ans = vec![];
        for q in queries {
            match q[0] {
                1 => update_value(q[1], q[2], &mut root),
                2 => ans.push(range_xor(q[1], q[2], &mut root)),
                _ => reverse_range(q[1], q[2], &mut root),
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;
    #[test]
    pub fn test_get_results_1() {
        assert_eq!(
            vec![5, 8],
            Solution::get_results(
                vec![1, 2, 3, 4, 5],
                lc_matrix![[2, 1, 3], [1, 2, 10], [3, 0, 4], [2, 0, 4]]
            )
        );
    }
    #[test]
    pub fn test_get_results_2() {
        assert_eq!(
            vec![2],
            Solution::get_results(vec![7, 8, 9], lc_matrix![[1, 0, 3], [2, 0, 2], [3, 1, 2]])
        );
    }
}
