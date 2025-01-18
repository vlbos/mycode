# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Purchases`

# +-------------+----------+
# | Column Name | Type     |
# +-------------+----------+
# | user\_id     | int      |
# | time\_stamp  | datetime |
# | amount      | int      |
# +-------------+----------+
# (user\_id, time\_stamp) is the primary key (combination of columns with unique values) for this table.
# Each row contains information about the purchase time and the amount paid for the user with ID user\_id.

# A user is eligible for a discount if they had a purchase in the inclusive interval of time `[startDate, endDate]` with at least `minAmount` amount. To convert the dates to times, both dates should be considered as the **start** of the day (i.e., `endDate = 2022-03-05` should be considered as the time `2022-03-05 00:00:00`).

# Write a solution to report the IDs of the users that are eligible for a discount.

# Return the result table ordered by `user_id`.

# The result format is in the following example.

# **Example 1:**

# **Input:**
# Purchases table:
# +---------+---------------------+--------+
# | user\_id | time\_stamp          | amount |
# +---------+---------------------+--------+
# | 1       | 2022-04-20 09:03:00 | 4416   |
# | 2       | 2022-03-19 19:24:02 | 678    |
# | 3       | 2022-03-18 12:03:09 | 4523   |
# | 3       | 2022-03-30 09:43:42 | 626    |
# +---------+---------------------+--------+
# startDate = 2022-03-08, endDate = 2022-03-20, minAmount = 1000
# **Output:**
# +---------+
# | user\_id |
# +---------+
# | 3       |
# +---------+
# **Explanation:**
# Out of the three users, only User 3 is eligible for a discount.
#  - User 1 had one purchase with at least minAmount amount, but not within the time interval.
#  - User 2 had one purchase within the time interval, but with less than minAmount amount.
#  - User 3 is the only user who had a purchase that satisfies both conditions.

# **Important Note:** This problem is basically the same as [The Number of Users That Are Eligible for Discount](https://leetcode.com/problems/the-number-of-users-that-are-eligible-for-discount/).



import pandas as pd
from datetime import datetime

def find_valid_users(purchases: pd.DataFrame, start_date: datetime, end_date: datetime, min_amount: int) -> pd.DataFrame:
    df=purchases.query('time_stamp>=@start_date and time_stamp<=@end_date and amount>=@min_amount')
    df=df.groupby("user_id").head(1)["user_id"]
    return  pd.DataFrame(df).sort_values('user_id')


import pandas as pd
from datetime import datetime

def find_valid_users(purchases: pd.DataFrame, start_date: datetime, end_date: datetime, min_amount: int) -> pd.DataFrame:
    valid_purchases = purchases[purchases['amount'] >= min_amount]
    valid_purchases = valid_purchases[(valid_purchases['time_stamp'] >= start_date) & (valid_purchases['time_stamp'] <= end_date)]
    return valid_purchases[['user_id']].drop_duplicates().sort_values('user_id')