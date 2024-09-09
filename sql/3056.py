# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Activities`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | activity\_id   | int     |
# | user\_id       | int     |
# | activity\_type | enum    |
# | time\_spent    | decimal |
# +---------------+---------+
# activity\_id is column of unique values for this table.
# activity\_type is an ENUM (category) type of ('send', 'open'). 
# This table contains activity id, user id, activity type and time spent.

# Table: `Age`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | user\_id     | int  |
# | age\_bucket  | enum |
# +-------------+------+
# user\_id is the column of unique values for this table.
# age\_bucket is an ENUM (category) type of ('21-25', '26-30', '31-35'). 
# This table contains user id and age group.

# Write a solution to calculate the **percentage** of the total time spent on **sending** and **opening snaps** for **each age group**. Precentage should be **rounded** to `2` decimal places.

# Return _the result table_ _in **any** order._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Activities table:
# +-------------+---------+---------------+------------+
# | activity\_id | user\_id | activity\_type | time\_spent |
# +-------------+---------+---------------+------------+
# | 7274        | 123     | open          | 4.50       | 
# | 2425        | 123     | send          | 3.50       | 
# | 1413        | 456     | send          | 5.67       | 
# | 2536        | 456     | open          | 3.00       | 
# | 8564        | 456     | send          | 8.24       | 
# | 5235        | 789     | send          | 6.24       | 
# | 4251        | 123     | open          | 1.25       | 
# | 1435        | 789     | open          | 5.25       | 
# +-------------+---------+---------------+------------+
# Age table:
# +---------+------------+
# | user\_id | age\_bucket | 
# +---------+------------+
# | 123     | 31-35      | 
# | 789     | 21-25      | 
# | 456     | 26-30      | 
# +---------+------------+
# **Output:** 
# +------------+-----------+-----------+
# | age\_bucket | send\_perc | open\_perc |
# +------------+-----------+-----------+
# | 31-35      | 37.84     | 62.16     |
# | 26-30      | 82.26     | 17.74     |
# | 21-25      | 54.31     | 45.69     |
# +------------+-----------+-----------+
# **Explanation:** 
# For age group 31-35:
#   - There is only one user belonging to this group with the user ID 123.
#   - The total time spent on sending snaps by this user is 3.50, and the time spent on opening snaps is 4.50 + 1.25 = 5.75.
#   - The overall time spent by this user is 3.50 + 5.75 = 9.25.
#   - Therefore, the sending snap percentage will be (3.50 / 9.25) \* 100 = 37.84, and the opening snap percentage will be (5.75 / 9.25) \* 100 = 62.16.
# For age group 26-30: 
#   - There is only one user belonging to this group with the user ID 456. 
#   - The total time spent on sending snaps by this user is 5.67 + 8.24 = 13.91, and the time spent on opening snaps is 3.00. 
#   - The overall time spent by this user is 13.91 + 3.00 = 16.91. 
#   - Therefore, the sending snap percentage will be (13.91 / 16.91) \* 100 = 82.26, and the opening snap percentage will be (3.00 / 16.91) \* 100 = 17.74.
# For age group 21-25: 
#   - There is only one user belonging to this group with the user ID 789. 
#   - The total time spent on sending snaps by this user is 6.24, and the time spent on opening snaps is 5.25. 
#   - The overall time spent by this user is 6.24 + 5.25 = 11.49. 
#   - Therefore, the sending snap percentage will be (6.24 / 11.49) \* 100 = 54.31, and the opening snap percentage will be (5.25 / 11.49) \* 100 = 45.69.
# All percentages in output table rounded to the two decimal places.




import pandas as pd

def snap_analysis(activities: pd.DataFrame, age: pd.DataFrame) -> pd.DataFrame:

    df = activities.merge(age, how = 'left'
                  ).pivot_table(index=["age_bucket"], columns = 'activity_type', 
                                values='time_spent', aggfunc={'time_spent': "sum"}
                  ).reset_index().fillna(0)

    df['sm'] =  df.open + df.send 
    df['send_perc'] = round(df.send/df.sm*100,2)
    df['open_perc'] = round(df.open/df.sm*100,2)

    return df.iloc[:,[0,4,5]]