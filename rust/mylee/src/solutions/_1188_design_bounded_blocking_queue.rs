// 1188\. Design Bounded Blocking Queue
// ====================================

// Implement a thread safe bounded blocking queue that has the following methods:

// *   `BoundedBlockingQueue(int capacity)` The constructor initializes the queue with a maximum `capacity`.
// *   `void enqueue(int element)` Adds an `element` to the front of the queue. If the queue is full, the calling thread is blocked until the queue is no longer full.
// *   `int dequeue()` Returns the element at the rear of the queue and removes it. If the queue is empty, the calling thread is blocked until the queue is no longer empty.
// *   `int size()` Returns the number of elements currently in the queue.

// Your implementation will be tested using multiple threads at the same time. Each thread will either be a producer thread that only makes calls to the `enqueue` method or a consumer thread that only makes calls to the `dequeue` method. The `size` method will be called after every test case.

// Please do not use built-in implementations of bounded blocking queue as this will not be accepted in an interview.

// **Example 1:**

// **Input:**
// 1
// 1
// \["BoundedBlockingQueue","enqueue","dequeue","dequeue","enqueue","enqueue","enqueue","enqueue","dequeue"\]
// \[\[2\],\[1\],\[\],\[\],\[0\],\[2\],\[3\],\[4\],\[\]\]

// **Output:**
// \[1,0,2,2\]

// **Explanation:** Number of producer threads = 1
// Number of consumer threads = 1

// BoundedBlockingQueue queue = new BoundedBlockingQueue(2);   // initialize the queue with capacity = 2.

// queue.enqueue(1);   // The producer thread enqueues 1 to the queue.
// queue.dequeue();    // The consumer thread calls dequeue and returns 1 from the queue.
// queue.dequeue();    // Since the queue is empty, the consumer thread is blocked.
// queue.enqueue(0);   // The producer thread enqueues 0 to the queue. The consumer thread is unblocked and returns 0 from the queue.
// queue.enqueue(2);   // The producer thread enqueues 2 to the queue.
// queue.enqueue(3);   // The producer thread enqueues 3 to the queue.
// queue.enqueue(4);   // The producer thread is blocked because the queue's capacity (2) is reached.
// queue.dequeue();    // The consumer thread returns 2 from the queue. The producer thread is unblocked and enqueues 4 to the queue.
// queue.size();       // 2 elements remaining in the queue. size() is always called at the end of each test case.

// **Example 2:**

// **Input:**
// 3
// 4
// \["BoundedBlockingQueue","enqueue","enqueue","enqueue","dequeue","dequeue","dequeue","enqueue"\]
// \[\[3\],\[1\],\[0\],\[2\],\[\],\[\],\[\],\[3\]\]

// **Output:**
// \[1,0,2,1\]

// **Explanation:** Number of producer threads = 3
// Number of consumer threads = 4

// BoundedBlockingQueue queue = new BoundedBlockingQueue(3);   // initialize the queue with capacity = 3.

// queue.enqueue(1);   // Producer thread P1 enqueues 1 to the queue.
// queue.enqueue(0);   // Producer thread P2 enqueues 0 to the queue.
// queue.enqueue(2);   // Producer thread P3 enqueues 2 to the queue.
// queue.dequeue();    // Consumer thread C1 calls dequeue.
// queue.dequeue();    // Consumer thread C2 calls dequeue.
// queue.dequeue();    // Consumer thread C3 calls dequeue.
// queue.enqueue(3);   // One of the producer threads enqueues 3 to the queue.
// queue.size();       // 1 element remaining in the queue.

// Since the number of threads for producer/consumer is greater than 1, we do not know how the threads will be scheduled in the operating system, even though the input seems to imply the ordering. Therefore, any of the output \[1,0,2\] or \[1,2,0\] or \[0,1,2\] or \[0,2,1\] or \[2,0,1\] or \[2,1,0\] will be accepted.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Cloudera](https://leetcode.ca/tags/#Cloudera) [Google](https://leetcode.ca/tags/#Google) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Oracle](https://leetcode.ca/tags/#Oracle) [Uber](https://leetcode.ca/tags/#Uber)
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
pub struct BoundedBlockingQueue {
    capacity: i32,
    q: Mutex<VecDeque<i32>>,
    full_cv: Condvar,
    empty_cv: Condvar,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BoundedBlockingQueue {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity,
            q: Mutex::new(VecDeque::new()),
            full_cv: Condvar::new(),
            empty_cv: Condvar::new(),
        }
    }

    pub fn en_queue(&mut self, element: i32) {
        let mut lq = self.q.lock().unwrap();
        while lq.len() == self.capacity as usize {
            lq = self.full_cv.wait(lq).unwrap();
        }
        lq.push_back(element);
        self.empty_cv.notify_one();
    }

    pub fn de_queue(&mut self) -> i32 {
        let mut lq = self.q.lock().unwrap();
        while lq.is_empty() {
            lq = self.empty_cv.wait(lq).unwrap();
        }
        let ans = lq.pop_front().unwrap();
        self.full_cv.notify_one();
        ans
    }
    pub fn size(&self) -> i32 {
        self.q.lock().unwrap().len() as _
    }
}
// // 此代码可视为 C++ 生产者、消费者模型的一个板子，会在很多地方碰到。
// class BoundedBlockingQueue {
//     int maxSize;
//     mutex mx;
//     condition_variable cv;
//     queue<int> q;
// public:
//     BoundedBlockingQueue(int capacity)
//         : maxSize(capacity){}

//     void enqueue(int element) {
//         unique_lock ul(mx);
//         cv.wait(ul, [&](){return q.size() < maxSize;});
//         q.push(element);
//         ul.unlock();
//         cv.notify_one();
//     }

//     int dequeue() {
//         unique_lock ul(mx);
//         cv.wait(ul, [&](){return q.size() > 0;});
//         int ret = q.front();
//         q.pop();
//         ul.unlock();
//         cv.notify_one();
//         return ret;
//     }

//     int size() {
//         return q.size();
//     }
// };
// from threading import Condition
// import threading

// class BoundedBlockingQueue(object):

//     def __init__(self, capacity: int):
//         self.capacity = capacity
//         self.queue = collections.deque([])
//         self.mutex = threading.Lock()
//         self.not_full = Condition(self.mutex)
//         self.not_empty = Condition(self.mutex)

//     def enqueue(self, element: int) -> None:
//         with self.not_full:
//             while self.size() >= self.capacity:
//                 self.not_full.wait()
//             self.queue.appendleft(element)
//             self.not_empty.notify_all()

//     def dequeue(self) -> int:
//         with self.not_empty:
//             while not self.size():
//                 self.not_empty.wait()
//             ans = self.queue.pop()
//             self.not_full.notify_all()
//             return ans

//     def size(self) -> int:
//         return len(self.queue)
/**
 * Your BoundedBlockingQueue object will be instantiated and called as such:
 * let obj = BoundedBlockingQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_bounded_blocking_queue_1() {
        let k = 1;
        let value = 0;
        let mut obj = BoundedBlockingQueue::new(k);
        obj.en_queue(value);
        let _ret_2: i32 = obj.de_queue();
        let _ret_3: i32 = obj.size();
    }
}
