/*
 * @lc app=leetcode.cn id=492 lang=rust
 *
 * [492] 构造矩形
 */

// @lc code=start
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut w = (area as f64).sqrt() as i32;
        while area % w != 0 {
            w -= 1;
        }
        vec![area / w, w].to_vec()
        //  fn is_div(n1:i32,n2:i32)->bool{
        //     if  n1%n2==0 {
        // 	    println!("{}={}*{}",n1,n2,n1/n2);
        // 	    return true;
        // 	}
        // 	else{
        // 	    return false;
        // 	}

        // }

        //  fn is_prime_number(num:i32)->bool{
        //     let y1=num as f32;
        // 	let y2=y1.sqrt() as i32+1;
        // 	let mut is_prime=true;
        //     for x in 2..y2 {
        //         if is_div(num,x){
        // 		   is_prime=false;
        // 		}
        // 	}
        //     return is_prime;
        // }
        // if is_prime_number(area){
        //             return vec![area,1].to_vec();
        //     }
        //         if area==4{
        // return vec![2,2].to_vec();
        //         }
        //   if area==6{
        // return vec![3,2].to_vec();
        //         }
        //             let area = area as i64;
        //             let mut a = area/2;
        //             let mut i=area/2;
        //             let mut start = 2i64;
        //             let mut end = (area/2) as i64;
        //             let mut mid =(start+end)/2;
        //             let mut min = area as i64;
        //             while start<end {
        //                 mid = (start+end)/2;
        //                 println!("{},{},{}",mid,start,end);
        //                 i = 1;
        //                 if (area/mid)*mid !=area {
        //                 while (mid+i)<=end && mid-i>=start{
        //                     let midi = mid+i;
        //                      if (area/midi)*midi ==area{
        //                     mid = midi;
        //                     break;
        //                     }
        //                     let midi = mid-i;
        //                      if (area/midi)*midi ==area{
        //                     mid = midi;
        //                     break;
        //                     }
        //                     i+=1;
        //                 }
        //                  if (area/mid)*mid !=area {
        //                     break;
        //                     }
        //                 }
        //                 if   mid >= area/mid{
        //                         println!("min/area:{}:{}:{}",mid,(area/mid),(area/mid)*mid);
        //                     if (area/mid)*mid ==area {
        //                         println!("minarea:{}",mid);
        //                     }
        //                     if (mid - area/mid)<(min- area/min){
        //                         println!("min:{}",mid);
        //                         min = mid;
        //                     }else if min<area{
        //                     break;
        //                     }

        //                     end=mid-1;
        //                 } else  {
        //                     start=mid+1;
        //                 }

        //             }
        //             vec![min as i32,(area/min) as i32].to_vec()
    }
}
// @lc code=end
