// 1756\. Design Most Recently Used Queue
// ======================================

// Design a queue-like data structure that moves the most recently used element to the end of the queue.

// Implement the `MRUQueue` class:

// *   `MRUQueue(int n)` constructs the `MRUQueue` with `n` elements: `[1,2,3,...,n]`.
// *   `fetch(int k)` moves the `kth` element **(1-indexed)** to the end of the queue and returns it.

// **Example 1:**

// **Input:**
// \["MRUQueue", "fetch", "fetch", "fetch", "fetch"\]
// \[\[8\], \[3\], \[5\], \[2\], \[8\]\]
// **Output:**
// \[null, 3, 6, 2, 2\]

// **Explanation:**
// MRUQueue mRUQueue = new MRUQueue(8); // Initializes the queue to \[1,2,3,4,5,6,7,8\].
// mRUQueue.fetch(3); // Moves the 3rd element (3) to the end of the queue to become \[1,2,4,5,6,7,8,3\] and returns it.
// mRUQueue.fetch(5); // Moves the 5th element (6) to the end of the queue to become \[1,2,4,5,7,8,3,6\] and returns it.
// mRUQueue.fetch(2); // Moves the 2nd element (2) to the end of the queue to become \[1,4,5,7,8,3,6,2\] and returns it.
// mRUQueue.fetch(8); // The 8th element (2) is already at the end of the queue so just return it.

// **Constraints:**

// *   `1 <= n <= 2000`
// *   `1 <= k <= n`
// *   At most `2000` calls will be made to `fetch`.

// **Follow up:** Finding an `O(n)` algorithm per `fetch` is a bit easy. Can you find an algorithm with a better complexity for each `fetch` call?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Google](https://leetcode.ca/tags/#Google)

//   public MRUQueue(int n) {

//     public int fetch(int k) {

#[allow(dead_code)]
pub struct MRUQueue {
    q: Vec<Vec<i32>>,
    len: i32,
}

impl MRUQueue {
    pub fn new(n: i32) -> Self {
        let len = (n as f32).sqrt() as i32 + 1;
        let mut q = vec![Vec::new(); len as usize];
        for i in 1..=n {
            q[((i - 1) / len) as usize].push(i);
        }
        Self { q, len }
    }
    pub fn fetch(&mut self, k: i32) -> i32 {
        let k = k - 1;
        let len = self.len;
        let j = k % len;
        let ans = self.q[(k / len) as usize].remove(j as usize);
        self.q[len as usize - 1].push(ans);
        for i in k / len..len - 1 {
            let r = self.q[i as usize + 1].remove(0);
            self.q[i as usize].push(r);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_fetch_1() {
        let mut q = MRUQueue::new(8);
        let test = [3, 5, 2, 8];
        let expected_result = [3, 6, 2, 2];
        for (t, e) in test.into_iter().zip(expected_result) {
            assert_eq!(e, q.fetch(t,));
        }
    }
}
