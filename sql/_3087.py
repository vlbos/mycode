# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Tweets`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | user\_id     | int     |
# | tweet\_id    | int     |
# | tweet\_date  | date    |
# | tweet       | varchar |
# +-------------+---------+
# tweet\_id is the primary key (column with unique values) for this table.
# Each row of this table contains user\_id, tweet\_id, tweet\_date and tweet.

# Write a solution to find the **top** `3` trending **hashtags** in **February** `2024`. Each tweet only contains one hashtag.

# Return _the result table orderd by count of hashtag, hashtag in_ **descending** _order._

# The result format is in the following example.

# **Example 1:**

# **Input:**

# Tweets table:

# +---------+----------+----------------------------------------------+------------+
# | user\_id | tweet\_id | tweet                                        | tweet\_date |
# +---------+----------+----------------------------------------------+------------+
# | 135     | 13       | Enjoying a great start to the day! #HappyDay | 2024-02-01 |
# | 136     | 14       | Another #HappyDay with good vibes!           | 2024-02-03 |
# | 137     | 15       | Productivity peaks! #WorkLife                | 2024-02-04 |
# | 138     | 16       | Exploring new tech frontiers. #TechLife      | 2024-02-04 |
# | 139     | 17       | Gratitude for today's moments. #HappyDay     | 2024-02-05 |
# | 140     | 18       | Innovation drives us. #TechLife              | 2024-02-07 |
# | 141     | 19       | Connecting with nature's serenity. #Nature   | 2024-02-09 |
# +---------+----------+----------------------------------------------+------------+
 

# **Output:**

# +-----------+--------------+
# | hashtag   | hashtag\_count|
# +-----------+--------------+
# | #HappyDay | 3            |
# | #TechLife | 2            |
# | #WorkLife | 1            |
# +-----------+--------------+

# **Explanation:**

# *   **#HappyDay:** Appeared in tweet IDs 13, 14, and 17, with a total count of 3 mentions.
# *   **#TechLife:** Appeared in tweet IDs 16 and 18, with a total count of 2 mentions.
# *   **#WorkLife:** Appeared in tweet ID 15, with a total count of 1 mention.

# **Note:** Output table is sorted in descending order by hashtag\_count and hashtag respectively.



import pandas as pd

def find_trending_hashtags(tweets: pd.DataFrame) -> pd.DataFrame:
    return (
        tweets.assign(hashtag=(tweets["tweet"].str.extract(r"(#\w+)")))
        .groupby("hashtag", as_index=False)
        .agg(hashtag_count=("tweet", "count"))
        .sort_values(["hashtag_count", "hashtag"], ascending=False)
        .head(3)
    )