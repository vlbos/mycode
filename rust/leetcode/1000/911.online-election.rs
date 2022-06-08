/*
 * @lc app=leetcode id=911 lang=rust
 *
 * [911] Online Election
 */

// @lc code=start
struct TopVotedCandidate {
times: Vec<i32>,
tops: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {

    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut tops = Vec::new();
        let mut m = std::collections::HashMap::new();
        let mut top = -1;
        for &p in &persons{
            *m.entry(p).or_insert(0)+=1;
            if m.get(&p).unwrap_or(&0)>=m.get(&top).unwrap_or(&0){
            top=p;
            }
            tops.push(top);
        }
        Self{tops,times}
    }
    
    fn q(&self, t: i32) -> i32 {
        let mut l = 0;
        let mut r = self.times.len()-1;
        while l<r{
             let mid = l+(r-l+1)/2;
            if self.times[mid]<=t{
                l=mid;
            }else{
                r = mid-1;
            }
        }
        self.tops[l]
    }
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
// @lc code=end

