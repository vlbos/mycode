# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Users`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | user\_id     | int     |
# | name        | varchar |
# +-------------+---------+
# `user_id` is the column with unique values for this table.
# Each row of this table contains user id and name.

# Table: `Rides`

# +--------------+------+
# | Column Name  | Type |
# +--------------+------+
# | ride\_id      | int  |
# | user\_id      | int  | 
# | distance     | int  |
# +--------------+------+
# ride\_id is the column of unique values for this table.
# Each row of this table contains ride id, user id, and traveled distance.

# Write a solution to calculate the `distance` traveled by **each user**. If there is a user who hasn't completed any rides, then their `distance` should be considered as `0`. Output the `user_id`, `name` and total traveled `distance`.

# Return _the result table ordered by_ `user_id` _in **ascending** order._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Users table:
# +---------+---------+
# | user\_id | name    |
# +---------+---------+
# | 17      | Addison |
# | 14      | Ethan   |
# | 4       | Michael |
# | 2       | Avery   |
# | 10      | Eleanor |
# +---------+---------+
# Rides table:
# +---------+---------+----------+
# | ride\_id | user\_id | distance |
# +---------+---------+----------+
# | 72      | 17      | 160      |
# | 42      | 14      | 161      |
# | 45      | 4       | 59       |
# | 32      | 2       | 197      |
# | 15      | 4       | 357      |
# | 56      | 2       | 196      |
# | 10      | 14      | 25       |
# +---------+---------+----------+
# **Output:** 
# +---------+---------+-------------------+
# | user\_id | name    | traveled distance |
# +---------+---------+-------------------+
# | 2       | Avery   | 393               |
# | 4       | Michael | 416               |
# | 10      | Eleanor | 0                 |
# | 14      | Ethan   | 186               |
# | 17      | Addison | 160               |
# +---------+---------+-------------------+
# **Explanation:** 
# -  User id 2 completed two journeys of 197 and 196, resulting in a combined travel distance of 393.
# -  User id 4 completed two journeys of 59 and 357, resulting in a combined travel distance of 416.
# -  User id 14 completed two journeys of 161 and 25, resulting in a combined travel distance of 186.
# -  User id 16 completed only one journey of 160.
# -  User id 10 did not complete any journeys, thus the total travel distance remains at 0.
# Returning the table orderd by user\_id in ascending order.



import pandas as pd

def get_total_distance(users: pd.DataFrame, rides: pd.DataFrame) -> pd.DataFrame:
    grouped = rides.groupby(by="user_id", as_index=False).agg(distance=("distance", "sum"))
    return (pd.merge(
        left=users,
        right=grouped,
        on="user_id",
        how="left"
    )
            .sort_values(by="user_id", ascending=True)
            .fillna(0)
            .rename(columns={"distance": "traveled distance"}))