# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Playback`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | session\_id  | int  |
# | customer\_id | int  |
# | start\_time  | int  |
# | end\_time    | int  |
# +-------------+------+
# session\_id is the column with unique values for this table.
# customer\_id is the ID of the customer watching this session.
# The session runs during the **inclusive** interval between start\_time and end\_time.
# It is guaranteed that start\_time <= end\_time and that two sessions for the same customer do not intersect.

# Table: `Ads`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | ad\_id       | int  |
# | customer\_id | int  |
# | timestamp   | int  |
# +-------------+------+
# ad\_id is the column with unique values for this table.
# customer\_id is the ID of the customer viewing this ad.
# timestamp is the moment of time at which the ad was shown.

# Write a solution to report all the sessions that did not get shown any ads.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Playback table:
# +------------+-------------+------------+----------+
# | session\_id | customer\_id | start\_time | end\_time |
# +------------+-------------+------------+----------+
# | 1          | 1           | 1          | 5        |
# | 2          | 1           | 15         | 23       |
# | 3          | 2           | 10         | 12       |
# | 4          | 2           | 17         | 28       |
# | 5          | 2           | 2          | 8        |
# +------------+-------------+------------+----------+
# Ads table:
# +-------+-------------+-----------+
# | ad\_id | customer\_id | timestamp |
# +-------+-------------+-----------+
# | 1     | 1           | 5         |
# | 2     | 2           | 17        |
# | 3     | 2           | 20        |
# +-------+-------------+-----------+
# **Output:** 
# +------------+
# | session\_id |
# +------------+
# | 2          |
# | 3          |
# | 5          |
# +------------+
# **Explanation:** 
# The ad with ID 1 was shown to user 1 at time 5 while they were in session 1.
# The ad with ID 2 was shown to user 2 at time 17 while they were in session 4.
# The ad with ID 3 was shown to user 2 at time 20 while they were in session 4.
# We can see that sessions 1 and 4 had at least one ad. Sessions 2, 3, and 5 did not have any ads, so we return them.


import pandas as pd

def ad_free_sessions(playback: pd.DataFrame, ads: pd.DataFrame) -> pd.DataFrame:
    df = playback.merge(ads,on="customer_id").query("timestamp>=start_time & timestamp<=end_time")
    return playback.loc[~playback["session_id"].isin(set(df["session_id"]))][["session_id"]]