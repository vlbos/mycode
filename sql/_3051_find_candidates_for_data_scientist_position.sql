# [3051. Find Candidates for Data Scientist Position](https://leetcode.com/problems/find-candidates-for-data-scientist-position)

[中文文档](/solution/3000-3099/3051.Find%20Candidates%20for%20Data%20Scientist%20Position/README.md)



## Description

Table: Candidates


+--------------+---------+ 
| Column Name  | Type    | 
+--------------+---------+ 
| candidate_id | int     | 
| skill        | varchar |
+--------------+---------+
(candidate_id, skill) is the primary key (columns with unique values) for this table.
Each row includes candidate_id and skill.


Write a query to find the candidates best suited for a Data Scientist position. The candidate must be proficient in Python, Tableau, and PostgreSQL.

Return the result table ordered by candidate_id in ascending order.

The result format is in the following example.

 
Example 1:


Input: 
Candidates table:
+---------------+--------------+
| candidate_id  | skill        | 
+---------------+--------------+
| 123           | Python       |
| 234           | R            | 
| 123           | Tableau      | 
| 123           | PostgreSQL   | 
| 234           | PowerBI      | 
| 234           | SQL Server   | 
| 147           | Python       | 
| 147           | Tableau      | 
| 147           | Java         |
| 147           | PostgreSQL   |
| 256           | Tableau      |
| 102           | DataAnalysis |
+---------------+--------------+
Output: 
+--------------+
| candidate_id |  
+--------------+
| 123          |  
| 147          | 
+--------------+
Explanation: 
- Candidates 123 and 147 possess the necessary skills in Python, Tableau, and PostgreSQL for the data scientist position.
- Candidates 234 and 102 do not possess any of the required skills for this position.
- Candidate 256 has proficiency in Tableau but is missing skills in Python and PostgreSQL.
The output table is sorted by candidate_id in ascending order.


## Solutions

### Solution 1: Conditional Filtering + Grouping Statistics

First, we filter out candidates who have the skills `Python`, `Tableau`, and `PostgreSQL`. Then, we group by `candidate_id` and count the number of skills each candidate has. Finally, we filter out candidates who have these three skills and sort them in ascending order by `candidate_id`.



```sql
# Write your MySQL query statement below
SELECT candidate_id
FROM Candidates
WHERE skill IN ('Python', 'Tableau', 'PostgreSQL')
GROUP BY 1
HAVING COUNT(1) = 3
ORDER BY 1;
```

<!-- tabs:end -->

<!-- end -->
