# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `UserVisits`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | user\_id     | int  |
# | visit\_date  | date |
# +-------------+------+
# This table does not have a primary key, it might contain duplicate rows.
# This table contains logs of the dates that users visited a certain retailer.

# Assume today's date is `'2021-1-1'`.

# Write a solution that will, for each `user_id`, find out the largest `window` of days between each visit and the one right after it (or today if you are considering the last visit).

# Return the result table ordered by `user_id`.

# The query result format is in the following example.

# **Example 1:**

# **Input:** 
# UserVisits table:
# +---------+------------+
# | user\_id | visit\_date |
# +---------+------------+
# | 1       | 2020-11-28 |
# | 1       | 2020-10-20 |
# | 1       | 2020-12-3  |
# | 2       | 2020-10-5  |
# | 2       | 2020-12-9  |
# | 3       | 2020-11-11 |
# +---------+------------+
# **Output:** 
# +---------+---------------+
# | user\_id | biggest\_window|
# +---------+---------------+
# | 1       | 39            |
# | 2       | 65            |
# | 3       | 51            |
# +---------+---------------+
# **Explanation:** 
# For the first user, the windows in question are between dates:
#     - 2020-10-20 and 2020-11-28 with a total of 39 days. 
#     - 2020-11-28 and 2020-12-3 with a total of 5 days. 
#     - 2020-12-3 and 2021-1-1 with a total of 29 days.
# Making the biggest window the one with 39 days.
# For the second user, the windows in question are between dates:
#     - 2020-10-5 and 2020-12-9 with a total of 65 days.
#     - 2020-12-9 and 2021-1-1 with a total of 23 days.
# Making the biggest window the one with 65 days.
# For the third user, the only window in question is between dates 2020-11-11 and 2021-1-1 with a total of 51 days.


 import pandas as pd

def biggest_window(user_visits: pd.DataFrame) -> pd.DataFrame:
  user_visits.sort_values(by=['user_id', 'visit_date'], inplace=True)
  user_visits['next_visit'] = user_visits.groupby(['user_id'])['visit_date'].shift(periods=-1, fill_value='2021-1-1') #fill_value can be supplied with a string value here
  user_visits['window'] = (user_visits['next_visit'] - user_visits['visit_date']).dt.days
  return user_visits.groupby('user_id', as_index=False)[['window']].max().rename(columns={'window': 'biggest_window'})