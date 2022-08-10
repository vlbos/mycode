// 635\. Design Log Storage System
// ===============================
// You are given several logs that each log contains a unique id and timestamp.
// Timestamp is a string that has the following format: `Year:Month:Day:Hour:Minute:Second`,
// for example, `2017:01:01:23:59:59`. All domains are zero-padded decimal numbers.
// Design a log storage system to implement the following functions:
// `void Put(int id, string timestamp)`: Given a log's unique id and timestamp, store the log in your storage system.
// `int[] Retrieve(String start, String end, String granularity)`: Return the id of logs whose timestamps are within the range from start to end.
// Start and end all have the same format as timestamp. However, granularity means the time level for consideration.
//  For example, start = "2017:01:01:23:59:59", end = "2017:01:02:23:59:59", granularity = "Day",
// it means that we need to find the logs within the range from Jan. 1st 2017 to Jan. 2nd 2017.
// **Example 1:**

// put(1, "2017:01:01:23:59:59");
// put(2, "2017:01:01:22:59:59");
// put(3, "2016:01:01:00:00:00");
// retrieve("2016:01:01:01:01:01","2017:01:01:23:00:00","Year"); // return \[1,2,3\], because you need to return all logs within 2016 and 2017.
// retrieve("2016:01:01:01:01:01","2017:01:01:23:00:00","Hour"); // return \[1,2\], because you need to return all logs start from 2016:01:01:01 to 2017:01:01:23, where log 3 is left outside the range.

// **Note:**

// 1.  There will be at most 300 operations of Put or Retrieve.
// 2.  Year ranges from \[2000,2017\]. Hour ranges from \[00,23\].
// 3.  Output for Retrieve has no order required.

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Amazon](https://leetcode.ca/tags/#Amazon) [Google](https://leetcode.ca/tags/#Google) [Snapchat](https://leetcode.ca/tags/#Snapchat) [Twitter](https://leetcode.ca/tags/#Twitter)

// @lc code=start
// use std::collections::BTreeMap;
// use std::ops::Bound::Included;

// const STARTS: [&'static str; 6] = ["0000", "00", "00", "00", "00", "00"];
// const ENDS: [&'static str; 6] = ["9999", "12", "31", "23", "59", "59"];

use std::collections::HashMap;
pub  struct LogSystem {
    // logs: BTreeMap<u64, i32>,
    id2tm: HashMap<i32, String>,
    granularities: HashMap<String, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LogSystem {
    pub fn new() -> Self {
        // Self {
        //     logs: BTreeMap::new(),
        // }
        Self {
            id2tm: HashMap::new(),
            granularities: ["Year", "Month", "Day", "Hour", "Minute", "Second"]
                .iter()
                .enumerate()
                .map(|(i, v)| (v.to_string(), i + 1))
                .collect(),
        }
    }

    pub fn put(&mut self, id: i32, timestamp: String) {
        // self.logs.insert(LogSystem::to_timestamp(&timestamp), id);
        self.id2tm.insert(id, timestamp);
    }

    pub fn retrieve(&self, start: String, end: String, granularity: String) -> Vec<i32> {
        // let level = LogSystem::granularity_to_level(&granularity);
        // let from = LogSystem::to_start_timestamp(&start, level);
        // let to = LogSystem::to_end_timestamp(&end, level);
        // self.logs
        //     .range((Included(&from), Included(&to)))
        //     .map(|(_, v)| *v)
        //     .collect()
        let mut ans = Vec::new();
        let i = *self.granularities.get(&granularity).unwrap() * 3 + 1;
        for (&id, tm) in &self.id2tm {
            let (pre, start, end) = (&tm[..i], &start[..i], &end[..i]);
            if pre >= start && pre <= end {
                ans.push(id);
            }
        }
        ans
    }

    // fn timestamp_to_chunks<'a>(ts: &'a str) -> Vec<&'a str> {
    //     ts.split(":").collect::<Vec<_>>()
    // }

    // fn granularity_to_level(granularity: &str) -> usize {
    //     match granularity {
    //         "Year" => 1,
    //         "Month" => 2,
    //         "Day" => 3,
    //         "Hour" => 4,
    //         "Minute" => 5,
    //         "Second" => 6,
    //         _ => unreachable!(),
    //     }
    // }

    // fn to_timestamp(ts: &str) -> u64 {
    //     ts.replace(":", "").parse::<u64>().unwrap()
    // }

    // fn to_start_timestamp(ts: &str, level: usize) -> u64 {
    //     let chunks = LogSystem::timestamp_to_chunks(ts);
    //     let mut res = String::new();
    //     for i in 0..level {
    //         res += chunks[i];
    //     }
    //     for i in level..chunks.len() {
    //         res += STARTS[i];
    //     }
    //     res.parse::<u64>().unwrap()
    // }

    // fn to_end_timestamp(ts: &str, level: usize) -> u64 {
    //     let chunks = LogSystem::timestamp_to_chunks(ts);
    //     let mut res = String::new();
    //     for i in 0..level {
    //         res += chunks[i];
    //     }
    //     for i in level..chunks.len() {
    //         res += ENDS[i];
    //     }
    //     res.parse::<u64>().unwrap()
    // }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_log_system_1() {
        let mut ls = LogSystem::new();
        ls.put(1, String::from("2017:01:01:23:59:59"));
        ls.put(2, String::from("2017:01:01:22:59:59"));
        ls.put(3, String::from("2016:01:01:00:00:00"));
        assert_eq!(
            ls.retrieve(
                String::from("2016:01:01:01:01:01"),
                String::from("2017:01:01:23:00:00"),
                String::from("Year")
            )
            .into_iter()
            .collect::<HashSet<_>>(),
            vec![3, 2, 1].into_iter().collect::<HashSet<_>>()
        );

        assert_eq!(
            ls.retrieve(
                String::from("2016:01:01:01:01:01"),
                String::from("2017:01:01:23:00:00"),
                String::from("Hour")
            )
            .into_iter()
            .collect::<HashSet<_>>(),
            vec![2, 1].into_iter().collect::<HashSet<_>>()
        )
    }
}
