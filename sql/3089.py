# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Posts`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | post\_id     | int     |
# | user\_id     | int     |
# | post\_date   | date    |
# +-------------+---------+
# post\_id is the primary key (column with unique values) for this table.
# Each row of this table contains post\_id, user\_id, and post\_date.

# Write a solution to find users who demonstrate **bursty behavior** in their posting patterns during February `2024`. **Bursty behavior** is defined as **any** period of **7** **consecutive** days where a user's posting frequency is **at least twice** to their **average** weekly posting frequency for February `2024`.

# **Note:** Only include the dates from February `1` to February `28` in your analysis, which means you should count February as having exactly `4` weeks.

# Return _the result table orderd by_ `user_id` _in_ **ascending** _order._

# The result format is in the following example.

# **Example:**

# **Input:**

# Posts table:

# +---------+---------+------------+
# | post\_id | user\_id | post\_date  |
# +---------+---------+------------+
# | 1       | 1       | 2024-02-27 |
# | 2       | 5       | 2024-02-06 |
# | 3       | 3       | 2024-02-25 |
# | 4       | 3       | 2024-02-14 |
# | 5       | 3       | 2024-02-06 |
# | 6       | 2       | 2024-02-25 |
# +---------+---------+------------+

# **Output:**

# +---------+----------------+------------------+
# | user\_id | max\_7day\_posts | avg\_weekly\_posts |
# +---------+----------------+------------------+
# | 1       | 1              | 0.2500           |
# | 2       | 1              | 0.2500           |
# | 5       | 1              | 0.2500           |
# +---------+----------------+------------------+

# **Explanation:**

# *   **User 1:** Made only 1 post in February, resulting in an average of 0.25 posts per week and a max of 1 post in any 7-day period.
# *   **User 2:** Also made just 1 post, with the same average and max 7-day posting frequency as User 1.
# *   **User 5:** Like Users 1 and 2, User 5 made only 1 post throughout February, leading to the same average and max 7-day posting metrics.
# *   **User 3:** Although User 3 made more posts than the others (3 posts), they did not reach twice the average weekly posts in their consecutive 7-day window, so they are not listed in the output.

# **Note:** Output table is ordered by user\_id in ascending order.



import pandas as pd

def find_bursty_behavior(posts: pd.DataFrame) -> pd.DataFrame:
    
    posts = posts[posts['post_date'].astype(str) <= '2024-02-28']
    posts = posts.set_index('post_date').sort_index()

    df = posts.groupby('user_id')['post_id'].rolling('7D').count().reset_index()\
    .rename(columns={'post_id':'weekly_posts'})

    df['max_weekly_posts'] = df.groupby('user_id')['weekly_posts'].transform('max')

    avg_weekly = posts.groupby('user_id')['post_id'].count().reset_index().rename(columns={'post_id':'avg_weekly_posts'})
    avg_weekly['avg_weekly_posts'] = avg_weekly['avg_weekly_posts'] / 4

    df = df.merge(avg_weekly, on='user_id')

    return df[df['max_weekly_posts'] >= df['avg_weekly_posts'] * 2][['user_id','max_weekly_posts','avg_weekly_posts']]\
    .rename(columns={'max_weekly_posts':'max_7day_posts'}).drop_duplicates()