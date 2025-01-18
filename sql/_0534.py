# Medium

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
# (player\_id, event\_date) is the primary key (column with unique values) of this table.
# This table shows the activity of players of some games.
# Each row is a record of a player who logged in and played a number of games (possibly 0) before logging out on someday using some device.

# Write a solution to report for each player and date, how many games played **so far** by the player. That is, the total number of games played by the player until that date. Check the example for clarity.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Activity table:
# +-----------+-----------+------------+--------------+
# | player\_id | device\_id | event\_date | games\_played |
# +-----------+-----------+------------+--------------+
# | 1         | 2         | 2016-03-01 | 5            |
# | 1         | 2         | 2016-05-02 | 6            |
# | 1         | 3         | 2017-06-25 | 1            |
# | 3         | 1         | 2016-03-02 | 0            |
# | 3         | 4         | 2018-07-03 | 5            |
# +-----------+-----------+------------+--------------+
# **Output:** 
# +-----------+------------+---------------------+
# | player\_id | event\_date | games\_played\_so\_far |
# +-----------+------------+---------------------+
# | 1         | 2016-03-01 | 5                   |
# | 1         | 2016-05-02 | 11                  |
# | 1         | 2017-06-25 | 12                  |
# | 3         | 2016-03-02 | 0                   |
# | 3         | 2018-07-03 | 5                   |
# +-----------+------------+---------------------+
# **Explanation:** 
# For the player with id 1, 5 + 6 = 11 games played by 2016-05-02, and 5 + 6 + 1 = 12 games played by 2017-06-25.
# For the player with id 3, 0 + 5 = 5 games played by 2018-07-03.
# Note that for each player we only care about the days when the player logged in.






import pandas as pd

def gameplay_analysis(activity: pd.DataFrame) -> pd.DataFrame:
    activity =activity.sort_values(by='event_date')
    activity['games_played_so_far']=activity.groupby(['player_id'])['games_played'].transform('cumsum')
    return activity[['player_id','event_date','games_played_so_far']]