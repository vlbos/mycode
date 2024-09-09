# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Ads`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | ad\_id         | int     |
# | user\_id       | int     |
# | action        | enum    |
# +---------------+---------+
# (ad\_id, user\_id) is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the ID of an Ad, the ID of a user, and the action taken by this user regarding this Ad.
# The action column is an ENUM (category) type of ('Clicked', 'Viewed', 'Ignored').

# A company is running Ads and wants to calculate the performance of each Ad.

# Performance of the Ad is measured using Click-Through Rate (CTR) where:

# ![](https://assets.leetcode.com/uploads/2020/01/17/sql1.png)

# Write a solution to find the `ctr` of each Ad. **Round** `ctr` to **two decimal points**.

# Return the result table ordered by `ctr` in **descending order** and by `ad_id` in **ascending order** in case of a tie.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Ads table:
# +-------+---------+---------+
# | ad\_id | user\_id | action  |
# +-------+---------+---------+
# | 1     | 1       | Clicked |
# | 2     | 2       | Clicked |
# | 3     | 3       | Viewed  |
# | 5     | 5       | Ignored |
# | 1     | 7       | Ignored |
# | 2     | 7       | Viewed  |
# | 3     | 5       | Clicked |
# | 1     | 4       | Viewed  |
# | 2     | 11      | Viewed  |
# | 1     | 2       | Clicked |
# +-------+---------+---------+
# **Output:** 
# +-------+-------+
# | ad\_id | ctr   |
# +-------+-------+
# | 1     | 66.67 |
# | 3     | 50.00 |
# | 2     | 33.33 |
# | 5     | 0.00  |
# +-------+-------+
# **Explanation:** 
# for ad\_id = 1, ctr = (2/(2+1)) \* 100 = 66.67
# for ad\_id = 2, ctr = (1/(1+2)) \* 100 = 33.33
# for ad\_id = 3, ctr = (1/(1+1)) \* 100 = 50.00
# for ad\_id = 5, ctr = 0.00, Note that ad\_id = 5 has no clicks or views.
# Note that we do not care about Ignored Ads.


import pandas as pd

def ads_performance(ads: pd.DataFrame) -> pd.DataFrame:
    df = ads.query("action == 'Clicked' or action == 'Viewed'").assign(action = ads.action == "Clicked")
    df = df.groupby("ad_id").agg(clicked = ("action","sum"), total = ("action","count"))
    df = df.assign(ctr = df["clicked"] / df["total"] * 100 )[["ctr"]]
    df = ads[["ad_id"]].drop_duplicates().merge(df, on ="ad_id",how = "left")
    return df.round(2).fillna(0).sort_values(["ctr","ad_id"],ascending = [0,1])