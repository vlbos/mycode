/*
 * @lc app=leetcode id=355 lang=rust
 *
 * [355] Design Twitter
 */

// @lc code=start
use std::collections::{HashMap,HashSet};
struct Twitter {
       followers:HashMap<i32,HashSet<i32>>,
       tweets:HashMap<i32,Vec<(i32,u64)>>,
       timestamp:u64,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        Self{followers:HashMap::new(),tweets:HashMap::new(),timestamp:0}
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.timestamp+=1;
        self.tweets.entry(user_id).or_insert(Vec::new()).push((tweet_id,self.timestamp));
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut ans:Vec<(i32,u64)> =Vec::new();
        let top = 10;
        ans = self.tweets.iter().filter(|x| *(x.0)==user_id||self.followers.get(&user_id).unwrap_or(&HashSet::new()).contains(x.0)).fold(ans,|mut s,f|{s.extend(f.1);s});
        ans.sort_by(|a,b|(b.1).cmp(&a.1));
        if ans.len()>10{
        ans=ans[..10].to_vec();
        }
        ans.iter().map(|x| x.0).collect()
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.followers.entry(follower_id).or_insert(HashSet::new()).insert(followee_id);
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
         if let Some(followees)=self.followers.get_mut(&follower_id){
                    followees.remove(&followee_id);
         }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
// @lc code=end

