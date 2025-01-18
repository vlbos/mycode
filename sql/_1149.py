# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Views`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | article\_id    | int     |
# | author\_id     | int     |
# | viewer\_id     | int     |
# | view\_date     | date    |
# +---------------+---------+
# This table may have duplicate rows.
# Each row of this table indicates that some viewer viewed an article (written by some author) on some date. 
# Note that equal author\_id and viewer\_id indicate the same person.

# Write a solution to find all the people who viewed more than one article on the same date.

# Return the result table sorted by `id` in ascending order.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Views table:
# +------------+-----------+-----------+------------+
# | article\_id | author\_id | viewer\_id | view\_date  |
# +------------+-----------+-----------+------------+
# | 1          | 3         | 5         | 2019-08-01 |
# | 3          | 4         | 5         | 2019-08-01 |
# | 1          | 3         | 6         | 2019-08-02 |
# | 2          | 7         | 7         | 2019-08-01 |
# | 2          | 7         | 6         | 2019-08-02 |
# | 4          | 7         | 1         | 2019-07-22 |
# | 3          | 4         | 4         | 2019-07-21 |
# | 3          | 4         | 4         | 2019-07-21 |
# +------------+-----------+-----------+------------+
# **Output:** 
# +------+
# | id   |
# +------+
# | 5    |
# | 6    |
# +------+




import pandas as pd

def article_views(views: pd.DataFrame) -> pd.DataFrame:
            return (views.groupby(['viewer_id','view_date']) 
                    .nunique()                            
                    .query("article_id > 1")              
                    .reset_index()['viewer_id']           
                    .drop_duplicates()                    
                    .to_frame('id'))   