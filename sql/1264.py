# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Friendship`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | user1\_id      | int     |
# | user2\_id      | int     |
# +---------------+---------+
# (user1\_id, user2\_id) is the primary key (combination of columns with unique values) for this table.
# Each row of this table indicates that there is a friendship relation between user1\_id and user2\_id.

# Table: `Likes`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | user\_id     | int     |
# | page\_id     | int     |
# +-------------+---------+
# (user\_id, page\_id) is the primary key (combination of columns with unique values) for this table.
# Each row of this table indicates that user\_id likes page\_id.

# Write a solution to recommend pages to the user with `user_id = 1` using the pages that your friends liked. It should not recommend pages you already liked.

# Return result table in **any order** without duplicates.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Friendship table:
# +----------+----------+
# | user1\_id | user2\_id |
# +----------+----------+
# | 1        | 2        |
# | 1        | 3        |
# | 1        | 4        |
# | 2        | 3        |
# | 2        | 4        |
# | 2        | 5        |
# | 6        | 1        |
# +----------+----------+
# Likes table:
# +---------+---------+
# | user\_id | page\_id |
# +---------+---------+
# | 1       | 88      |
# | 2       | 23      |
# | 3       | 24      |
# | 4       | 56      |
# | 5       | 11      |
# | 6       | 33      |
# | 2       | 77      |
# | 3       | 77      |
# | 6       | 88      |
# +---------+---------+
# **Output:** 
# +------------------+
# | recommended\_page |
# +------------------+
# | 23               |
# | 24               |
# | 56               |
# | 33               |
# | 77               |
# +------------------+
# **Explanation:** 
# User one is friend with users 2, 3, 4 and 6.
# Suggested pages are 23 from user 2, 24 from user 3, 56 from user 3 and 33 from user 6.
# Page 77 is suggested from both user 2 and user 3.
# Page 88 is not suggested because user 1 already likes it.



import pandas as pd

def page_recommendations(friendship: pd.DataFrame, likes: pd.DataFrame) -> pd.DataFrame:
    fp = likes.loc[likes.user_id.isin(np.union1d(friendship.loc[friendship.user1_id == 1, 'user2_id'], friendship.loc[friendship.user2_id == 1, 'user1_id'])), 'page_id'].unique()
    return pd.DataFrame({'recommended_page':list(np.setdiff1d(fp, likes.loc[likes.user_id == 1,'page_id'].unique()))})