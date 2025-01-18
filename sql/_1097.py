# Hard

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

# The **install date** of a player is the first login day of that player.

# We define **day one retention** of some date `x` to be the number of players whose **install date** is `x` and they logged back in on the day right after `x`, divided by the number of players whose install date is `x`, rounded to `2` decimal places.

# Write a solution to report for each install date, the number of players that installed the game on that day, and the **day one retention**.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Activity table:
# +-----------+-----------+------------+--------------+
# | player\_id | device\_id | event\_date | games\_played |
# +-----------+-----------+------------+--------------+
# | 1         | 2         | 2016-03-01 | 5            |
# | 1         | 2         | 2016-03-02 | 6            |
# | 2         | 3         | 2017-06-25 | 1            |
# | 3         | 1         | 2016-03-01 | 0            |
# | 3         | 4         | 2016-07-03 | 5            |
# +-----------+-----------+------------+--------------+
# **Output:** 
# +------------+----------+----------------+
# | install\_dt | installs | Day1\_retention |
# +------------+----------+----------------+
# | 2016-03-01 | 2        | 0.50           |
# | 2017-06-25 | 1        | 0.00           |
# +------------+----------+----------------+
# **Explanation:** 
# Player 1 and 3 installed the game on 2016-03-01 but only player 1 logged back in on 2016-03-02 so the day 1 retention of 2016-03-01 is 1 / 2 = 0.50
# Player 2 installed the game on 2017-06-25 but didn't log back in on 2017-06-26 so the day 1 retention of 2017-06-25 is 0 / 1 = 0.00



import pandas as pd

def gameplay_analysis(activity: pd.DataFrame) -> pd.DataFrame:
    activity['install_dt'] = activity.groupby('player_id')['event_date'].transform('min')
    activity['second'] = activity['install_dt'] + pd.DateOffset(1)
    df_second_log = activity.query("second == event_date").groupby('install_dt')['player_id'].count().to_frame('second')
    df_first_log =  activity.query("install_dt == event_date").groupby('install_dt')['player_id'].count().to_frame('installs')
    df = df_first_log.merge(df_second_log, on = 'install_dt', how = 'left').reset_index().fillna(0)
    return df.assign(Day1_retention = (df.second/df.installs*100+0.5).astype(int)/100).drop('second',axis=1)