

// 3279. Maximum Total Area Occupied by Pistons

// Hard

// Hint

// There are several pistons in an old car engine, and we want to calculate the **maximum** possible area **under** the pistons.

// You are given:

// *   An integer `height`, representing the **maximum** height a piston can reach.
// *   An integer array `positions`, where `positions[i]` is the current position of piston `i`, which is equal to the current area **under** it.
// *   A string `directions`, where `directions[i]` is the current moving direction of piston `i`, `'U'` for up, and `'D'` for down.

// Each second:

// *   Every piston moves in its current direction 1 unit. e.g., if the direction is up, `positions[i]` is incremented by 1.
// *   If a piston has reached one of the ends, i.e., `positions[i] == 0` or `positions[i] == height`, its direction will change.

// Return the _maximum possible area_ under all the pistons.

// **Example 1:**

// **Input:** height = 5, positions = \[2,5\], directions = "UD"

// **Output:** 7

// **Explanation:**

// The current position of the pistons has the maximum possible area under it.

// **Example 2:**

// **Input:** height = 6, positions = \[0,0,6,3\], directions = "UUDU"

// **Output:** 15

// **Explanation:**

// After 3 seconds, the pistons will be in positions `[3, 3, 3, 6]`, which has the maximum possible area under it.

// **Constraints:**

// *   `1 <= height <= 106`
// *   `1 <= positions.length == directions.length <= 105`
// *   `0 <= positions[i] <= height`
// *   `directions[i]` is either `'U'` or `'D'`.




#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_area(height: i32, positions: Vec<i32>, directions: String) -> i64 {
         let (mut down,mut up)=(vec![],vec![]);
        let (mut ans,mut cur,total)=(0,positions.iter().map(|&x|x as i64).sum::<i64>(),positions.len() as i64*height as i64);
        for (d,&p) in directions.chars().zip(&positions){
            if d=='D'{
                down.push(p);
            }else{
                up.push(height-p);
            }
        }
        down.sort_unstable();
        up.sort_unstable();
        let (mut i,mut j)=(0,0);
        for t in 0..height{
            while i<down.len() && down[i]==t{
                up.push(down[i]+height);
                i+=1;
            }
            while j<up.len() && up[j]==t{
                down.push(up[j]+height);
                j+=1;
            }
            cur+=(up.len()-j ) as i64-(down.len()-i)  as i64;
            ans=ans.max(cur).max(total-cur);
        }
        ans
    }
}




#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
    pub fn test_max_area_1() {
        assert_eq!(
            7,
            Solution::max_area(
                5,
                vec![2,5],String::from("UD"),
            )
        );
    }

    #[test]
    pub fn test_max_area_2() {
        assert_eq!(
            15,
            Solution::max_area(
               6,
                vec![0,0,6,3],String::from("UUDU"),
            )
        );
    }
}




// // class Solution {
// // public:
// //     #define ll long long int
// //     long long maxArea(int k, vector<int>& a, string s) {
// //         int n = a.size();
// //         map<ll, vector<ll>> m;
// //         ll sum = 0;
// //         ll add = 0;
// //         for(int i = 0;i<n;i++){
// //             sum += a[i];
// //             if(s[i]=='U'){
// //                 int val = k - a[i];
// //                 m[val].push_back(i);
// //                 m[val+k].push_back(i);
// //                 add++;
// //             }else{
// //                 m[a[i]].push_back(i);
// //                 m[a[i]+k].push_back(i);
// //                 add--;
// //             }
// //         }
// //         ll ans = sum;
// //         ll cur = 0;
// //         for(auto [t,j] : m){
// //             sum += (add*(t-cur));
// //             ans = max(sum,ans);
// //             for(auto i : j){
// //                 if(s[i]=='U'){
// //                     s[i] = 'D';
// //                     add -=2;
// //                 }else{
// //                     s[i] = 'U';
// //                     add += 2;
// //                 }
// //             }
// //             cur = t;
// //         }
// //         return ans;
// //     }
// // };




// // class Solution:
// //     def maxArea(self, height: int, positions: List[int], directions: str) -> int:
// //         events = {0:0}
// //         for p,d in zip(positions,directions):
// //             if p == height and d == 'U' or  p == 0 and d == 'D':
// //                 continue
// //             else:
// //                 if d == 'U':
// //                     ft = height - p
// //                     events[0] += 1
// //                     if ft not in events:
// //                         events[ft] = 0
// //                     events[ft]-=2
// //                     fft = ft + height
// //                     if fft not in events:
// //                         events[fft] = 0
// //                     events[fft]+=2
// //                 else:
// //                     ft = p
// //                     events[0] -= 1
// //                     if ft not in events:
// //                         events[ft] = 0
// //                     events[ft]+=2
// //                     fft = ft + height
// //                     if fft not in events:
// //                         events[fft] = 0
// //                     events[fft]-=2
// //         events = sorted([(a,events[a]) for a in events])
// //         currarea = sum(positions)
// //         currpace = 0
// //         currt = 0
// //         res = currarea
// //         for t,e in events:
// //             dt = t - currt
// //             currarea += dt*currpace
// //             currpace += e
// //             currt = t
// //             res = max(res,currarea)
// //         return res

        
//     private long sln1(int h, int[] ps, String ds){
//         int n = ps.length;
//         long[] timeline = new long[h*2+2];
//         long[] presum = new long[h*2+2];
//         long sum = 0, res = 0;
//         int[] map = new int[26];
//         map['U'-'A'] = 1; map['D'-'A'] = -1;
//         for(int i=0;i<n;i++){
//             sum += ps[i];
//             int d = map[ds.charAt(i) - 'A'];
//             timeline[1] += d;
//             timeline[h*(d+1>>1)-d*ps[i]+1] += -2*d;
//             timeline[h+h*(d+1>>1)-d*ps[i]+1] += 2*d;
//         }
//         for(int i= 1;i< timeline.length; i++){
//             timeline[i] += timeline[i-1];
//             presum[i] += presum[i-1]+timeline[i];
//             res = Math.max(res, sum+presum[i]);
//         }
//         return res;
//     }