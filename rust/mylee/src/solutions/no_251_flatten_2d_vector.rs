// 251\. Flatten 2D Vector
// =======================

// Design and implement an iterator to flatten a 2d vector. It should support the following operations: `next` and `hasNext`.

// **Example:**

// Vector2D iterator = new Vector2D(\[\[1,2\],\[3\],\[4\]\]);

// iterator.next(); // return 1
// iterator.next(); // return 2
// iterator.next(); // return 3
// iterator.hasNext(); // return true
// iterator.hasNext(); // return true
// iterator.next(); // return 4
// iterator.hasNext(); // return false

// **Notes:**

// 1.  Please remember to **RESET** your class variables declared in Vector2D, as static/class variables are **persisted across multiple test cases**. Please see [here](https://leetcode.com/faq/) for more details.
// 2.  You may assume that `next()` call will always be valid, that is, there will be at least a next element in the 2d vector when `next()` is called.

// **Follow up:**

// As an added challenge, try to code it using only [iterators in C++](http://www.cplusplus.com/reference/iterator/iterator/) or [iterators in Java](http://docs.oracle.com/javase/7/docs/api/java/util/Iterator.html).

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Airbnb](https://leetcode.ca/tags/#Airbnb) [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Facebook](https://leetcode.ca/tags/#Facebook) [Google](https://leetcode.ca/tags/#Google) [Lyft](https://leetcode.ca/tags/#Lyft) [Salesforce](https://leetcode.ca/tags/#Salesforce) [Twitter](https://leetcode.ca/tags/#Twitter) [Uber](https://leetcode.ca/tags/#Uber) [Zenefits](https://leetcode.ca/tags/#Zenefits)

// @lc code=start
pub struct Vector2D {
    // values: Vec<Vec<i32>>,
    // i: usize,
    // j: usize,
    vec: Vec<Vec<i32>>,
    row: usize,
    col: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Vector2D {
    pub fn new(vec: Vec<Vec<i32>>) -> Self {
        // Vector2D {
        //     values: v,
        //     i: 0,
        //     j: 0,
        // }
        Self { vec, row: 0, col: 0 }
    }

    pub fn next(&mut self) -> i32 {
        let has_next = self.has_next();
        if has_next {
            let ans = self.vec[self.row][self.col];
            self.col += 1;
            ans
        } else {
            -1
        }
        // let ans = self.v[self.row][self.col];
        // if self.col + 1 == self.v[self.row].len() {
        //     self.row = self.row + 1;
        //     self.col = 0;
        // } else {
        //     self.col += 1;
        // }
        // ans
    }

    pub fn has_next(&mut self) -> bool {
        loop {
            if self.row >= self.vec.len() {
                return false;
            } else if self.col < self.vec[self.row].len() {
                return true;
            } else {
                self.row += 1;
                self.col = 0;
            }
        }
        // self.row < self.v.len() && self.col < self.v[self.row].len()
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    // ["Vector2D","hasNext","next","hasNext"]
    // [[[[],[3]]],[],[],[]]
    #[test]
    pub fn test_vector_2d() {
        let vv = vec![vec![1, 2], vec![3], vec![4]];
        let mut v2d = Vector2D::new(vv);
        assert_eq!(v2d.next(), 1);
        assert_eq!(v2d.next(), 2);
        assert_eq!(v2d.next(), 3);
        assert!(v2d.has_next());
        assert!(v2d.has_next());
        assert_eq!(v2d.next(), 4);
        assert!(!v2d.has_next());
    }
}
