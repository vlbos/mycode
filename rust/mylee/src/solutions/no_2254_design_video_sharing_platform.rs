// # [2254. Design Video Sharing Platform](https://leetcode.com/problems/design-video-sharing-platform)

// ## Description

// You have a video sharing platform where users can upload and delete videos.
// Each video is a string of digits, where the ith digit of the string represents the content of the video at minute i.
// For example, the first digit represents the content at minute 0 in the video,
// the second digit represents the content at minute 1 in the video, and so on.
//  Viewers of videos can also like and dislike videos. Internally,
// the platform keeps track of the number of views, likes, and dislikes on each video.

// When a video is uploaded, it is associated with the smallest available integer videoId starting from 0.
//  Once a video is deleted, the videoId associated with that video can be reused for another video.

// Implement the VideoSharingPlatform class:

//
// 	VideoSharingPlatform() Initializes the object.
// 	int upload(String video) The user uploads a video. Return the videoId associated with the video.
// 	void remove(int videoId) If there is a video associated with videoId, remove the video.
// 	String watch(int videoId, int startMinute, int endMinute) If there is a video associated with videoId,
//  increase the number of views on the video by 1 and return the substring of the video string starting at startMinute and ending at min(endMinute, video.length - 1) (inclusive). Otherwise, return "-1".
// 	void like(int videoId) Increases the number of likes on the video associated with videoId by 1 if there is a video associated with videoId.
// 	void dislike(int videoId) Increases the number of dislikes on the video associated with videoId by 1 if there is a video associated with videoId.
// 	int[] getLikesAndDislikes(int videoId) Return a 0-indexed integer array values of length 2 where values[0] is the number of likes and values[1] is the number of dislikes on the video associated with videoId.
// If there is no video associated with videoId, return [-1].
// 	int getViews(int videoId) Return the number of views on the video associated with videoId, if there is no video associated with videoId, return -1.
//

// Example 1:

//
// Input
// ["VideoSharingPlatform", "upload", "upload", "remove", "remove", "upload", "watch", "watch", "like", "dislike", "dislike", "getLikesAndDislikes", "getViews"]
// [[], ["123"], ["456"], [4], [0], ["789"], [1, 0, 5], [1, 0, 1], [1], [1], [1], [1], [1]]
// Output
// [null, 0, 1, null, null, 0, "456", "45", null, null, null, [1, 2], 2]

// Explanation
// VideoSharingPlatform videoSharingPlatform = new VideoSharingPlatform();
// videoSharingPlatform.upload("123");          // The smallest available videoId is 0, so return 0.
// videoSharingPlatform.upload("456");          // The smallest available videoId is 1, so return 1.
// videoSharingPlatform.remove(4);              // There is no video associated with videoId 4, so do nothing.
// videoSharingPlatform.remove(0);              // Remove the video associated with videoId 0.
// videoSharingPlatform.upload("789");          // Since the video associated with videoId 0 was deleted,
//                                              // 0 is the smallest available videoId, so return 0.
// videoSharingPlatform.watch(1, 0, 5);         // The video associated with videoId 1 is "456".
//                                              // The video from minute 0 to min(5, 3 - 1) = 2 is "456", so return "453".
// videoSharingPlatform.watch(1, 0, 1);         // The video associated with videoId 1 is "456".
//                                              // The video from minute 0 to min(1, 3 - 1) = 1 is "45", so return "45".
// videoSharingPlatform.like(1);                // Increase the number of likes on the video associated with videoId 1.
// videoSharingPlatform.dislike(1);             // Increase the number of dislikes on the video associated with videoId 1.
// videoSharingPlatform.dislike(1);             // Increase the number of dislikes on the video associated with videoId 1.
// videoSharingPlatform.getLikesAndDislikes(1); // There is 1 like and 2 dislikes on the video associated with videoId 1, so return [1, 2].
// videoSharingPlatform.getViews(1);            // The video associated with videoId 1 has 2 views, so return 2.
//

// Example 2:

//
// Input
// ["VideoSharingPlatform", "remove", "watch", "like", "dislike", "getLikesAndDislikes", "getViews"]
// [[], [0], [0, 0, 1], [0], [0], [0], [0]]
// Output
// [null, null, "-1", null, null, [-1], -1]

// Explanation
// VideoSharingPlatform videoSharingPlatform = new VideoSharingPlatform();
// videoSharingPlatform.remove(0);              // There is no video associated with videoId 0, so do nothing.
// videoSharingPlatform.watch(0, 0, 1);         // There is no video associated with videoId 0, so return "-1".
// videoSharingPlatform.like(0);                // There is no video associated with videoId 0, so do nothing.
// videoSharingPlatform.dislike(0);             // There is no video associated with videoId 0, so do nothing.
// videoSharingPlatform.getLikesAndDislikes(0); // There is no video associated with videoId 0, so return [-1].
// videoSharingPlatform.getViews(0);            // There is no video associated with videoId 0, so return -1.
//

// Constraints:

//
// 	1 <= video.length <= 10^5
// 	The sum of video.length over all calls to upload does not exceed 10^5
// 	video consists of digits.
// 	0 <= videoId <= 10^5
// 	0 <= startMinute < endMinute < 10^5
// 	startMinute < video.length
// 	The sum of endMinute - startMinute over all calls to watch does not exceed 10^5.
// 	At most 10^5 calls in total will be made to all functions.
//

use std::collections::{BTreeSet, HashMap};
#[allow(dead_code)]
#[derive(Default)]
pub struct VideoSharingPlatform {
    used_id: BTreeSet<i32>,
    video_id2video: HashMap<i32, String>,
    video_id2views: HashMap<i32, i32>,
    video_id2likes: HashMap<i32, i32>,
    video_id2dislikes: HashMap<i32, i32>,
    curr_id: i32,
}
impl VideoSharingPlatform {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn upload(&mut self, video: String) -> i32 {
        let id = *self.used_id.iter().next().unwrap_or(&-1);
        let video_id = if id != -1 {
            self.used_id.remove(&id);
            id
        } else {
            let id = self.curr_id;
            self.curr_id += 1;
            id
        };
        self.video_id2video.insert(video_id, video);
        video_id
    }
    pub fn remove(&mut self, video_id: i32) {
        if self.video_id2video.get(&video_id).is_none() {
            return;
        }
        self.used_id.insert(video_id);
        self.video_id2video.remove(&video_id);
        self.video_id2views.remove(&video_id);
        self.video_id2likes.remove(&video_id);
        self.video_id2dislikes.remove(&video_id);
    }
    pub fn watch(&mut self, video_id: i32, start_minute: i32, end_minute: i32) -> String {
        let video = if let Some(v) = self.video_id2video.get(&video_id) {
            v.clone()
        } else {
            return "-1".to_string();
        };
        *self.video_id2views.entry(video_id).or_insert(0) += 1;

        let (s, e) = (
            if start_minute < 0 { 0 } else { start_minute } as usize,
            if end_minute > video.len() as i32 - 1 {
                video.len() as i32 - 1
            } else {
                end_minute
            } as usize,
        );
        video[s..=e].to_string()
    }
    pub fn like(&mut self, video_id: i32) {
        if self.video_id2video.get(&video_id).is_none() {
            return;
        }
        *self.video_id2likes.entry(video_id).or_insert(0) += 1;
    }
    pub fn dislike(&mut self, video_id: i32) {
        if self.video_id2video.get(&video_id).is_none() {
            return;
        }
        *self.video_id2dislikes.entry(video_id).or_insert(0) += 1;
    }
    pub fn get_likes_and_dislikes(&self, video_id: i32) -> Vec<i32> {
        if self.video_id2video.get(&video_id).is_none() {
            vec![-1]
        } else {
            vec![
                *self.video_id2likes.get(&video_id).unwrap_or(&0),
                *self.video_id2dislikes.get(&video_id).unwrap_or(&0),
            ]
        }
    }
    pub fn get_views(&self, video_id: i32) -> i32 {
        if self.video_id2video.get(&video_id).is_none() {
            -1
        } else {
            *self.video_id2views.get(&video_id).unwrap_or(&0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 输入：
    // ["VideoSharingPlatform","upload","upload","upload","getLikesAndDislikes","like","dislike"]
    // [[],["28"],["27"],["9728"],[0],[1],[0]]
    // 输出：
    // [null,0,1,2,[-1],null,null]
    // 预期结果：
    // [null,0,1,2,[0,0],null,null]
    #[test]
    pub fn test_video_sharing_platform_1() {
        let mut vsp = VideoSharingPlatform::new();
        assert_eq!(0, vsp.upload("123".to_string()));
        assert_eq!(1, vsp.upload("456".to_string()));
        vsp.remove(4);
        vsp.remove(0);
        assert_eq!(0, vsp.upload("789".to_string()));
        assert_eq!("456".to_string(), vsp.watch(1, 0, 5));
        assert_eq!("45".to_string(), vsp.watch(1, 0, 1));
        vsp.like(1);
        vsp.dislike(1);
        vsp.dislike(1);
        assert_eq!(vec![1, 2], vsp.get_likes_and_dislikes(1));
        assert_eq!(2, vsp.get_views(1));
    }
    #[test]
    pub fn test_video_sharing_platform_2() {
        let mut vsp = VideoSharingPlatform::new();
        vsp.remove(0);
        assert_eq!("-1".to_string(), vsp.watch(0, 0, 1));
        vsp.like(0);
        vsp.dislike(0);
        vsp.dislike(1);
        assert_eq!(vec![-1], vsp.get_likes_and_dislikes(0));
        assert_eq!(-1, vsp.get_views(0));
    }
}
