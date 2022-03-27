/*
 * @lc app=leetcode id=1093 lang=rust
 *
 * [1093] Statistics from a Large Sample
 */

// @lc code=start
impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let min = count.iter().position(|&x|x>0).unwrap();
        let max = count.iter().rposition(|&x|x>0).unwrap();
        let modei = *count.iter().max().unwrap();
        let mode = count.iter().position(|&x|x==modei).unwrap();
        let cnt = count.iter().sum::<i32>();
        let sum = count.iter().enumerate().map(|(i,&v)| i as f64 * v as f64).sum::<f64>();
        let avg = sum/cnt as f64;
        let mi = cnt>>1;
        let (mut m1,mut m2)= if cnt&1==1 {(mi,mi)}else{(mi-1,mi)};
        let (mut i1,mut i2)=(0f64,0f64);
        for (i,&v)in count.iter().enumerate(){
            if v==0{
            continue;
            }
            if m1>=0{
                m1-=v;
                if m1<0{
                    i1=i as f64;
                }
            }
            if m2>=0{
                m2-=v;
                if m2<0{
                    i2=i as f64;
                }
            }
        }
        vec![min as f64,max as f64,avg,(i1+i2)/2f64,mode as f64]
    }
}
// @lc code=end

