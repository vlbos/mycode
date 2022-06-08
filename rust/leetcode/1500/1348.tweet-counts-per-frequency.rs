/*
 * @lc app=leetcode id=1348 lang=rust
 *
 * [1348] Tweet Counts Per Frequency
 */

// @lc code=start
struct TweetCounts {
  users:std::collections::HashMap<String,Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TweetCounts {

    fn new() -> Self {
        Self{users:std::collections::HashMap::new()}
    }
    
    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        self.users.entry(tweet_name.clone()).or_default().push(time);
    }
    
    fn get_tweet_counts_per_frequency(&self, freq: String, tweet_name: String, start_time: i32, end_time: i32) -> Vec<i32> {
         let mut len = 60;
        if freq=="hour"{
            len = 60*60;
        }else if freq=="day"{
                len =  60*60*24;
        }
        let n = ((end_time-start_time)/len+1) as usize;
        let end_time=end_time+1;
        let mut ans = vec![0;n];
        if let Some(t)=self.users.get(&tweet_name){
             for &f in t{
                if  f>= start_time && f<end_time{
                    ans[((f-start_time)/len) as usize]+=1;
                }
             }
        }
        ans
    }
}

/**
 * Your TweetCounts object will be instantiated and called as such:
 * let obj = TweetCounts::new();
 * obj.record_tweet(tweetName, time);
 * let ret_2: Vec<i32> = obj.get_tweet_counts_per_frequency(freq, tweetName, startTime, endTime);
 */
// @lc code=end

