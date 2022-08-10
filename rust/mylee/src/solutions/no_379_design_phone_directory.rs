// 379\. Design Phone Directory
// ============================

// Design a Phone Directory which supports the following operations:

// 1.  `get`: Provide a number which is not assigned to anyone.
// 2.  `check`: Check if a number is available or not.
// 3.  `release`: Recycle or release a number.

// **Example:**

// // Init a phone directory containing a total of 3 numbers: 0, 1, and 2.
// PhoneDirectory directory = new PhoneDirectory(3);

// // It can return any available phone number. Here we assume it returns 0.
// directory.get();

// // Assume it returns 1.
// directory.get();

// // The number 2 is available, so return true.
// directory.check(2);

// // It returns 2, the only number that is left.
// directory.get();

// // The number 2 is no longer available, so return false.
// directory.check(2);

// // Release number 2 back to the pool.
// directory.release(2);

// // Number 2 is available again, return true.
// directory.check(2);

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Dropbox](https://leetcode.ca/tags/#Dropbox) [Google](https://leetcode.ca/tags/#Google) [Microsoft](https://leetcode.ca/tags/#Microsoft)

// @lc code=start
use std::collections::HashSet;

pub  struct PhoneDirectory {
    // size: usize,
    // unused: HashSet<i32>,
    numbers: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl PhoneDirectory {
    /** Initialize your data structure here
    @param maxNumbers - The maximum numbers that can be stored in the phone directory. */
    fn new(max_numbers: i32) -> Self {
        // Self {
        //     size: max_numbers as usize,
        //     unused: (0..max_numbers).collect(),
        // }
        Self {
            numbers: (0..max_numbers).collect(),
        }
    }

    /** Provide a number which is not assigned to anyone.
    @return - Return an available number. Return -1 if none is available. */
    fn get(&mut self) -> i32 {
        // let mut res = -1;
        // for i in &self.unused {
        //     res = *i;
        //     break;
        // }
        // if res != -1 {
        //     self.unused.remove(&res);
        // }
        // return res;
        let v = *self.numbers.iter().next().unwrap_or(&-1);
        if v != -1 {
            self.numbers.remove(&v);
        }
        v
    }

    /** Check if a number is available or not. */
    fn check(&self, number: i32) -> bool {
        // self.unused.contains(&number)
        self.numbers.contains(&number)
    }

    /** Recycle or release a number. */
    fn release(&mut self, number: i32) {
        // self.unused.insert(number);
        self.numbers.insert(number);
    }
}

// @lc code=end
