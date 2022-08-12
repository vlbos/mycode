// 362\. Design Hit Counter
// ========================

// Design a hit counter which counts the number of hits received in the past 5 minutes.

// Each function accepts a timestamp parameter (in seconds granularity) and you may assume
// that calls are being made to the system in chronological order (ie, the timestamp is monotonically increasing).
// You may assume that the earliest timestamp starts at 1.

// It is possible that several hits arrive roughly at the same time.

// **Example:**

// HitCounter counter = new HitCounter();

// // hit at timestamp 1.
// counter.hit(1);

// // hit at timestamp 2.
// counter.hit(2);

// // hit at timestamp 3.
// counter.hit(3);

// // get hits at timestamp 4, should return 3.
// counter.getHits(4);

// // hit at timestamp 300.
// counter.hit(300);

// // get hits at timestamp 300, should return 4.
// counter.getHits(300);

// // get hits at timestamp 301, should return 3.
// counter.getHits(301);

// **Follow up:**
// What if the number of hits per second could be very large? Does your design scale?

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Adobe](https://leetcode.ca/tags/#Adobe) [Amazon](https://leetcode.ca/tags/#Amazon) [Apple](https://leetcode.ca/tags/#Apple) [Bloomberg](https://leetcode.ca/tags/#Bloomberg) [Booking.com](https://leetcode.ca/tags/#Booking.com) [Dropbox](https://leetcode.ca/tags/#Dropbox) [Google](https://leetcode.ca/tags/#Google) [Indeed](https://leetcode.ca/tags/#Indeed) [LinkedIn](https://leetcode.ca/tags/#LinkedIn) [Microsoft](https://leetcode.ca/tags/#Microsoft) [Oracle](https://leetcode.ca/tags/#Oracle) [Pinterest](https://leetcode.ca/tags/#Pinterest) [Quip (Salesforce)](https://leetcode.ca/tags/#Quip%20(Salesforce)) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Uber](https://leetcode.ca/tags/#Uber) [Visa](https://leetcode.ca/tags/#Visa) [Yahoo](https://leetcode.ca/tags/#Yahoo) [Zillow](https://leetcode.ca/tags/#Zillow)

// @lc code=start

#[derive(Debug)]
pub struct HitCounter {
    // roll: Vec<i32>,
    // count: i32,
    // last_updated: i32,
    // first_active: i32,
    time: Vec<i32>,
    hits: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl HitCounter {
    /** Initialize your data structure here. */
    pub fn   new() -> Self {
        // Self {
        //     roll: vec![0; 300],
        //     count: 0,
        //     last_updated: 0,
        //     first_active: 0,
        // }
        Self {
            time: vec![0; 300],
            hits: vec![0; 300],
        }
    }

    /** Record a hit.
    @param timestamp - The current timestamp (in seconds granularity). */
    pub fn   hit(&mut self, timestamp: i32) {
        // self.clear(timestamp);
        // self.roll[((timestamp - 1) % 300) as usize] += 1;
        // self.count += 1;
        // self.last_updated = timestamp;
        let idx = (timestamp % 300) as usize;
        if self.time[idx] == timestamp {
            self.hits[idx] += 1;
        } else {
            self.time[idx] = timestamp;
            self.hits[idx] = 1;
        }
    }

    /** Return the number of hits in the past 5 minutes.
    @param timestamp - The current timestamp (in seconds granularity). */
    pub fn   get_hits(&self, timestamp: i32) -> i32 {
        // self.clear(timestamp);
        // return self.count;
        self.time
            .iter()
            .enumerate()
            .filter(|(_, &t)| timestamp - t < 300)
            .map(|(i, _)| self.hits[i])
            .sum::<i32>()
    }

    //pub fn  clear(&mut self, timestamp: i32) {
    //     // let last_updated = self.last_updated;
    //     // let first_active = self.first_active;
    //     // if timestamp >= last_updated + 300 {
    //     //     self.roll = vec![0; 300];
    //     //     self.count = 0;
    //     // } else if timestamp - 300 >= first_active + 1 {
    //     //     for t in (first_active + 1)..=(timestamp - 300) {
    //     //         let index = ((t - 1 + 300) % 300) as usize;
    //     //         self.count -= self.roll[index];
    //     //         self.roll[index] = 0;
    //     //     }
    //     // }
    //     // self.first_active = i32::max(0, timestamp - 300);
    // }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
   pub fn  test_hit_counter_1() {
        let mut counter = HitCounter::new();
        counter.hit(1);
        counter.hit(2);
        counter.hit(3);
        assert_eq!(counter.get_hits(4), 3);
        counter.hit(300);
        assert_eq!(counter.get_hits(300), 4);
        assert_eq!(counter.get_hits(301), 3);
    }

    #[test]
   pub fn  test_hit_counter_2() {
        let mut counter = HitCounter::new();
        counter.hit(100);
        counter.hit(101);
        counter.hit(202);
        assert_eq!(counter.get_hits(310), 3);
        assert_eq!(counter.get_hits(400), 2);
        assert_eq!(counter.get_hits(401), 1);
    }
}
