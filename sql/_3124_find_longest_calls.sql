-- ---

-- # [3124. Find Longest Calls 🔒](https://leetcode.com/problems/find-longest-calls)

-- ## Description

-- 

-- Table: Contacts

-- 
-- +-------------+---------+
-- | Column Name | Type    |
-- +-------------+---------+
-- | id          | int     |
-- | first_name  | varchar |
-- | last_name   | varchar |
-- +-------------+---------+
-- id is the primary key (column with unique values) of this table.
-- id is a foreign key (reference column) to Calls table.
-- Each row of this table contains id, first_name, and last_name.
-- 

-- Table: Calls

-- 
-- +-------------+------+
-- | Column Name | Type |
-- +-------------+------+
-- | contact_id  | int  |
-- | type        | enum |
-- | duration    | int  |
-- +-------------+------+
-- (contact_id, type, duration) is the primary key (column with unique values) of this table.
-- type is an ENUM (category) type of (&#39;incoming&#39;, &#39;outgoing&#39;).
-- Each row of this table contains information about calls, comprising of contact_id, type, and duration in seconds.
-- 

-- Write a solution to find the three longest incoming and outgoing calls.

-- Return the result table ordered by type, duration, and first_name in descending order and duration must be formatted as HH:MM:SS.

-- The result format is in the following example.

-- 
-- Example 1:

-- 
-- Input:

-- Contacts table:

-- 
-- +----+------------+-----------+
-- | id | first_name | last_name |
-- +----+------------+-----------+
-- | 1  | John       | Doe       |
-- | 2  | Jane       | Smith     |
-- | 3  | Alice      | Johnson   |
-- | 4  | Michael    | Brown     |
-- | 5  | Emily      | Davis     |
-- +----+------------+-----------+        
-- 

-- Calls table:

-- 
-- +------------+----------+----------+
-- | contact_id | type     | duration |
-- +------------+----------+----------+
-- | 1          | incoming | 120      |
-- | 1          | outgoing | 180      |
-- | 2          | incoming | 300      |
-- | 2          | outgoing | 240      |
-- | 3          | incoming | 150      |
-- | 3          | outgoing | 360      |
-- | 4          | incoming | 420      |
-- | 4          | outgoing | 200      |
-- | 5          | incoming | 180      |
-- | 5          | outgoing | 280      |
-- +------------+----------+----------+
--         

-- Output:

-- 
-- +-----------+----------+-------------------+
-- | first_name| type     | duration_formatted|
-- +-----------+----------+-------------------+
-- | Michael   | incoming | 00:07:00          |
-- | Jane      | incoming | 00:05:00          |
-- | Emily     | incoming | 00:03:00          |
-- | Alice     | outgoing | 00:06:00          |
-- | Emily     | outgoing | 00:04:40          |
-- | Jane      | outgoing | 00:04:00          |
-- +-----------+----------+-------------------+
--         

-- Explanation:

-- 
-- 	Michael had an incoming call lasting 7 minutes.
-- 	Jane had an incoming call lasting 5 minutes.
-- 	Emily had an incoming call lasting 3 minutes.
-- 	Alice had an outgoing call lasting 6 minutes.
-- 	Emily had an outgoing call lasting 4 minutes and 40 seconds.
-- 	Jane had an outgoing call lasting 4 minutes.
-- 

-- Note: Output table is sorted by type, duration, and first_name in descending order.
-- 

-- 

-- ## Solutions

-- 

-- ### Solution 1: Equi-Join + Window Function

-- We can use equi-join to connect the two tables, and then use the window function `RANK()` to calculate the ranking of each type of phone. Finally, we just need to filter out the top three phones.

-- 

-- #### MySQL

-- ```sql
-- WITH
--     T AS (
--         SELECT
--             first_name,
--             type,
--             DATE_FORMAT(SEC_TO_TIME(duration), "%H:%i:%s") AS duration_formatted,
--             RANK() OVER (
--                 PARTITION BY type
--                 ORDER BY duration DESC
--             ) AS rk
--         FROM
--             Calls AS c1
--             JOIN Contacts AS c2 ON c1.contact_id = c2.id
--     )
-- SELECT
--     first_name,
--     type,
--     duration_formatted
-- FROM T
-- WHERE rk <= 3
-- ORDER BY 2, 3 DESC, 1 DESC;
-- ```

-- #### Python3

-- ```python
-- import pandas as pd


-- def find_longest_calls(contacts: pd.DataFrame, calls: pd.DataFrame) -> pd.DataFrame:
--     merged_data = calls.merge(contacts, left_on="contact_id", right_on="id")
--     merged_data["duration_formatted"] = (
--         merged_data["duration"] // 3600 * 10000
--         + merged_data["duration"] % 3600 // 60 * 100
--         + merged_data["duration"] % 60
--     ).apply(lambda x: "{:02}:{:02}:{:02}".format(x // 10000, x // 100 % 100, x % 100))

--     merged_data["rk"] = merged_data.groupby("type")["duration"].rank(
--         method="dense", ascending=False
--     )

--     result = merged_data[merged_data["rk"] <= 3][
--         ["first_name", "type", "duration_formatted"]
--     ]
--     result = result.sort_values(
--         by=["type", "duration_formatted", "first_name"], ascending=[True, False, False]
--     )
--     return result
-- ```
