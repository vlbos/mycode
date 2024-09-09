# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Traffic`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | user\_id       | int     |
# | activity      | enum    |
# | activity\_date | date    |
# +---------------+---------+
# This table may have duplicate rows.
# The activity column is an ENUM (category) type of ('login', 'logout', 'jobs', 'groups', 'homepage').

# Write a solution to reports for every date within at most `90` days from today, the number of users that logged in for the first time on that date. Assume today is `2019-06-30`.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Traffic table:
# +---------+----------+---------------+
# | user\_id | activity | activity\_date |
# +---------+----------+---------------+
# | 1       | login    | 2019-05-01    |
# | 1       | homepage | 2019-05-01    |
# | 1       | logout   | 2019-05-01    |
# | 2       | login    | 2019-06-21    |
# | 2       | logout   | 2019-06-21    |
# | 3       | login    | 2019-01-01    |
# | 3       | jobs     | 2019-01-01    |
# | 3       | logout   | 2019-01-01    |
# | 4       | login    | 2019-06-21    |
# | 4       | groups   | 2019-06-21    |
# | 4       | logout   | 2019-06-21    |
# | 5       | login    | 2019-03-01    |
# | 5       | logout   | 2019-03-01    |
# | 5       | login    | 2019-06-21    |
# | 5       | logout   | 2019-06-21    |
# +---------+----------+---------------+
# **Output:** 
# +------------+-------------+
# | login\_date | user\_count  |
# +------------+-------------+
# | 2019-05-01 | 1           |
# | 2019-06-21 | 2           |
# +------------+-------------+
# **Explanation:** 
# Note that we only care about dates with non zero user count.
# The user with id 5 first logged in on 2019-03-01 so he's not counted on 2019-06-21.



import pandas as pd

def new_users_daily_count(traffic: pd.DataFrame) -> pd.DataFrame:
        return ( traffic.query("activity=='login'")
                    .groupby("user_id")["activity_date"].min().reset_index()
                    .query("activity_date>'2019-03-31'")
                    .groupby("activity_date").count().reset_index()
                    .rename(columns={"activity_date":"login_date","user_id":"user_count"}) )