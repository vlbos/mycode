# Hard

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

# Write a solution to find the **top** `3` trending **hashtags** in **February** `2024`. Every tweet may contain **several** **hashtags**.

# Return _the result table ordered by count of hashtag, hashtag in_ **descending** _order._

# The result format is in the following example.

# **Example 1:**

# **Input:**

# Tweets table:

# +---------+----------+------------------------------------------------------------+------------+
# | user\_id | tweet\_id | tweet                                                      | tweet\_date |
# +---------+----------+------------------------------------------------------------+------------+
# | 135     | 13       | Enjoying a great start to the day. #HappyDay #MorningVibes | 2024-02-01 |
# | 136     | 14       | Another #HappyDay with good vibes! #FeelGood               | 2024-02-03 |
# | 137     | 15       | Productivity peaks! #WorkLife #ProductiveDay               | 2024-02-04 |
# | 138     | 16       | Exploring new tech frontiers. #TechLife #Innovation        | 2024-02-04 |
# | 139     | 17       | Gratitude for today's moments. #HappyDay #Thankful         | 2024-02-05 |
# | 140     | 18       | Innovation drives us. #TechLife #FutureTech                | 2024-02-07 |
# | 141     | 19       | Connecting with nature's serenity. #Nature #Peaceful       | 2024-02-09 |
# +---------+----------+------------------------------------------------------------+------------+
 

# **Output:**

# +-----------+-------+
# | hashtag   | count |
# +-----------+-------+
# | #HappyDay | 3     |
# | #TechLife | 2     |
# | #WorkLife | 1     |
# +-----------+-------+

# **Explanation:**

# *   **#HappyDay:** Appeared in tweet IDs 13, 14, and 17, with a total count of 3 mentions.
# *   **#TechLife:** Appeared in tweet IDs 16 and 18, with a total count of 2 mentions.
# *   **#WorkLife:** Appeared in tweet ID 15, with a total count of 1 mention.

# **Note:** Output table is sorted in descending order by count and hashtag respectively.


import pandas as pd


def find_trending_hashtags(tweets: pd.DataFrame) -> pd.DataFrame:
    return (
        tweets.query("(tweet_date >= '2024-02-01') & (tweet_date < '2024-03-01')")
        .assign(
            hashtag=tweets["tweet"].apply(
                lambda x: [w for w in x.split(" ") if w.startswith("#")]
            )
        )
        .explode("hashtag")
        .groupby("hashtag", as_index=False)
        .agg(count=("hashtag", "count"))
        .sort_values(by=["count", "hashtag"], ascending=[False, False])
        .head(3)
    )