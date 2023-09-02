impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
            let n = colors.len();
            let cb=colors.as_bytes();
            let mut i =0;
            let mut ans = 0;
            while i<n{
                let c = cb[i];
                let mut sum = 0;
                let mut max = 0;
                while i<n && cb[i]==c{
                    sum+=needed_time[i];
                    max=max.max(needed_time[i]);
                    i+=1;
                }
                ans+=sum-max;
            }
            ans
    }
}
impl Solution {
    pub fn min_cost(colors: String, mut needed_time: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut dp=vec![0;n];
        for (i,w) in colors.as_bytes().windows(2).enumerate(){
            if w[0]!=w[1]{
                dp[i+1]=dp[i];
                continue
            }
            if needed_time[i]< needed_time[i+1]{
                dp[i+1]=dp[i]+needed_time[i];
            }else{
                dp[i+1]=dp[i]+needed_time[i+1];
                needed_time.swap(i,i+1);
            }
        }  
        dp[n-1] 
    }
}