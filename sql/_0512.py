
# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Activity`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | player\_id    | int     |
# | device\_id    | int     |
# | event\_date   | date    |
# | games\_played | int     |
# +--------------+---------+
# (player\_id, event\_date) is the primary key (combination of columns with unique values) of this table.
# This table shows the activity of players of some games.
# Each row is a record of a player who logged in and played a number of games (possibly 0) before logging out on someday using some device.

# Write a solution to report the **device** that is first logged in for each player.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Activity table:
# +-----------+-----------+------------+--------------+
# | player\_id | device\_id | event\_date | games\_played |
# +-----------+-----------+------------+--------------+
# | 1         | 2         | 2016-03-01 | 5            |
# | 1         | 2         | 2016-05-02 | 6            |
# | 2         | 3         | 2017-06-25 | 1            |
# | 3         | 1         | 2016-03-02 | 0            |
# | 3         | 4         | 2018-07-03 | 5            |
# +-----------+-----------+------------+--------------+
# **Output:** 
# +-----------+-----------+
# | player\_id | device\_id |
# +-----------+-----------+
# | 1         | 2         |
# | 2         | 3         |
# | 3         | 1         |
# +-----------+-----------+

import pandas as pd

def game_analysis(activity: pd.DataFrame) -> pd.DataFrame:
    return activity.loc[activity.groupby("player_id")["event_date"].idxmin()][
        ["player_id", "device_id"]
    ]