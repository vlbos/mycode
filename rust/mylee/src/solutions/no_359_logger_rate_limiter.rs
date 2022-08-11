// 359\. Logger Rate Limiter
// =========================

// Design a logger system that receive stream of messages along with its timestamps,
// each message should be printed if and only if it is **not printed in the last 10 seconds**.

// Given a message and a timestamp (in seconds granularity), return true if the message should be printed in the given timestamp,
//  otherwise returns false.

// It is possible that several messages arrive roughly at the same time.

// **Example:**

// Logger logger = new Logger();

// // logging string "foo" at timestamp 1
// logger.shouldPrintMessage(1, "foo"); returns true;

// // logging string "bar" at timestamp 2
// logger.shouldPrintMessage(2,"bar"); returns true;

// // logging string "foo" at timestamp 3
// logger.shouldPrintMessage(3,"foo"); returns false;

// // logging string "bar" at timestamp 8
// logger.shouldPrintMessage(8,"bar"); returns false;

// // logging string "foo" at timestamp 10
// logger.shouldPrintMessage(10,"foo"); returns false;

// // logging string "foo" at timestamp 11
// logger.shouldPrintMessage(11,"foo"); returns true;

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google) [Uber](https://leetcode.ca/tags/#Uber)

// @lc code=start
use std::collections::HashMap; //{HashSet, VecDeque};

pub struct Logger {
    // unique: HashSet<String>,
    // log: VecDeque<(i32, String)>,
    m: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {
    /** Initialize your data structure here. */
    fn new() -> Self {
        // Logger {
        //     unique: HashSet::new(),
        //     log: VecDeque::new(),
        // }
        Self { m: HashMap::new() }
    }

    /** Returns true if the message should be printed in the given timestamp, otherwise returns false.
    If this method returns false, the message will not be printed.
    The timestamp is in seconds granularity. */
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        // while let Some((t, s)) = self.log.pop_front() {
        //     if t + 10 > timestamp {
        //         self.log.push_front((t, s));
        //         break;
        //     } else {
        //         self.unique.remove(&s);
        //     }
        // }
        // if self.unique.contains(&message) {
        //     false
        // } else {
        //     self.unique.insert(message.clone());
        //     self.log.push_back((timestamp, message));
        //     true
        // }
        if !self.m.contains_key(&message) || timestamp - *self.m.get(&message).unwrap() >= 10 {
            self.m.insert(message, timestamp);
            true
        } else {
            false
        }
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logger_rate_limiter() {
        let actions: Vec<(i32, &str)> = vec![
            (1, "foo"),
            (2, "bar"),
            (3, "foo"),
            (8, "bar"),
            (10, "foo"),
            (11, "foo"),
        ];
        let results: Vec<bool> = vec![true, true, false, false, false, true];
        let mut logger = Logger::new();

        for i in 0..actions.len() {
            assert_eq!(
                logger.should_print_message(actions[i].0, String::from(actions[i].1)),
                results[i]
            )
        }
    }
}
