# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Activity`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | user\_id       | int     |
# | session\_id    | int     |
# | activity\_date | date    |
# | activity\_type | enum    |
# +---------------+---------+
# This table may have duplicate rows.
# The activity\_type column is an ENUM (category) of type ('open\_session', 'end\_session', 'scroll\_down', 'send\_message').
# The table shows the user activities for a social media website. 
# Note that each session belongs to exactly one user.

# Write a solution to find the average number of sessions per user for a period of `30` days ending `2019-07-27` inclusively, **rounded to 2 decimal places**. The sessions we want to count for a user are those with at least one activity in that time period.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Activity table:
# +---------+------------+---------------+---------------+
# | user\_id | session\_id | activity\_date | activity\_type |
# +---------+------------+---------------+---------------+
# | 1       | 1          | 2019-07-20    | open\_session  |
# | 1       | 1          | 2019-07-20    | scroll\_down   |
# | 1       | 1          | 2019-07-20    | end\_session   |
# | 2       | 4          | 2019-07-20    | open\_session  |
# | 2       | 4          | 2019-07-21    | send\_message  |
# | 2       | 4          | 2019-07-21    | end\_session   |
# | 3       | 2          | 2019-07-21    | open\_session  |
# | 3       | 2          | 2019-07-21    | send\_message  |
# | 3       | 2          | 2019-07-21    | end\_session   |
# | 3       | 5          | 2019-07-21    | open\_session  |
# | 3       | 5          | 2019-07-21    | scroll\_down   |
# | 3       | 5          | 2019-07-21    | end\_session   |
# | 4       | 3          | 2019-06-25    | open\_session  |
# | 4       | 3          | 2019-06-25    | end\_session   |
# +---------+------------+---------------+---------------+
# **Output:** 
# +---------------------------+ 
# | average\_sessions\_per\_user |
# +---------------------------+ 
# | 1.33                      |
# +---------------------------+
# **Explanation:** User 1 and 2 each had 1 session in the past 30 days while user 3 had 2 sessions so the average is (1 + 1 + 2) / 3 = 1.33.



import pandas as pd

def user_activity(activity: pd.DataFrame) -> pd.DataFrame:
   return (
        (
            activity[activity["activity_date"].between("2019-06-28", "2019-07-27")]
            .groupby("user_id", as_index=False)
            .agg({"session_id": "nunique"})
            .agg(average_sessions_per_user=("session_id", "mean"))
            .round(2)
            .transpose()
        )
        if activity["activity_date"].between("2019-06-28", "2019-07-27").any()
        else pd.DataFrame({"average_sessions_per_user": [0.00]})
    )


