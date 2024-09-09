# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Purchases`

# +---------------+------+
# | Column Name   | Type |
# +---------------+------+
# | purchase\_id   | int  |
# | user\_id       | int  |
# | purchase\_date | date |
# +---------------+------+
# purchase\_id contains unique values.
# This table contains logs of the dates that users purchased from a certain retailer.

# Write a solution to report the IDs of the users that made any two purchases **at most** `7` days apart.

# Return the result table ordered by `user_id`.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Purchases table:
# +-------------+---------+---------------+
# | purchase\_id | user\_id | purchase\_date |
# +-------------+---------+---------------+
# | 4           | 2       | 2022-03-13    |
# | 1           | 5       | 2022-02-11    |
# | 3           | 7       | 2022-06-19    |
# | 6           | 2       | 2022-03-20    |
# | 5           | 7       | 2022-06-19    |
# | 2           | 2       | 2022-06-08    |
# +-------------+---------+---------------+
# **Output:** 
# +---------+
# | user\_id |
# +---------+
# | 2       |
# | 7       |
# +---------+
# **Explanation:** 
# User 2 had two purchases on 2022-03-13 and 2022-03-20. Since the second purchase is within 7 days of the first purchase, we add their ID.
# User 5 had only 1 purchase.
# User 7 had two purchases on the same day so we add their ID.



import pandas as pd

def find_valid_users(purchases: pd.DataFrame) -> pd.DataFrame:
    s = purchases.sort_values(['user_id', 'purchase_date']).groupby('user_id').apply(lambda g: g['purchase_date']-g['purchase_date'].shift(1))<=pd.Timedelta(7, 'd')
    return s[s].index.get_level_values(0).to_frame().reset_index(drop=True).sort_values('user_id').drop_duplicates()