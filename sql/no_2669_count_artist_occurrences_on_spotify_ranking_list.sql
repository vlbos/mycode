# [2669. Count Artist Occurrences On Spotify Ranking List](https://leetcode.com/problems/count-artist-occurrences-on-spotify-ranking-list)

 

## Description

Table: <font face="monospace">Spotify


+-------------+---------+ 
| Column Name | Type    | 
+-------------+---------+ 
| id          | int     | 
| track_name  | varchar |
| artist      | varchar |
+-------------+---------+
id is the primary Key for this table.
Each row contains an id, track_name, and artist.


Write an SQL query to find how many times each artist appeared on the spotify ranking list.

Return the result table having the artist&#39;s name along with the corresponding number of occurrences ordered by occurrence count in descending order. If the occurrences are equal, then it&rsquo;s ordered by the artist&rsquo;s name in ascending order.

The query result format is in the following example​​​​​​.

 
 ### Example 1:


Input:
Spotify table: 
+---------+--------------------+------------+ 
| id      | track_name         | artist     |  
+---------+--------------------+------------+
| 303651  | Heart Won&#39;t Forget | Sia        |
| 1046089 | Shape of you       | Ed Sheeran |
| 33445   | I&#39;m the one        | DJ Khalid  |
| 811266  | Young Dumb &amp; Broke | DJ Khalid  | 
| 505727  | Happier            | Ed Sheeran |
+---------+--------------------+------------+ 
Output:
+------------+-------------+
| artist     | occurrences | 
+------------+-------------+
| DJ Khalid  | 2           |
| Ed Sheeran | 2           |
| Sia        | 1           | 
+------------+-------------+ 

Explanation: The count of occurrences is listed in descending order under the column name "occurrences". If the number of occurrences is the same, the artist&#39;s names are sorted in ascending order.


## Solutions

<!-- tabs:start -->

### **SQL**

```sql
# Write your MySQL query statement below
SELECT
    artist,
    count(1) AS occurrences
FROM Spotify
GROUP BY artist
ORDER BY occurrences DESC, artist;
```

<!-- tabs:end -->
