# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Players`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | player\_id      | int     |
# | player\_name    | varchar |
# +----------------+---------+
# player\_id is the primary key (column with unique values) for this table.
# Each row in this table contains the name and the ID of a tennis player.

# Table: `Championships`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | year          | int     |
# | Wimbledon     | int     |
# | Fr\_open       | int     |
# | US\_open       | int     |
# | Au\_open       | int     |
# +---------------+---------+
# year is the primary key (column with unique values) for this table.
# Each row of this table contains the IDs of the players who won one each tennis tournament of the grand slam.

# Write a solution to report the number of grand slam tournaments won by each player. Do not include the players who did not win any tournament.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Players table:
# +-----------+-------------+
# | player\_id | player\_name |
# +-----------+-------------+
# | 1         | Nadal       |
# | 2         | Federer     |
# | 3         | Novak       |
# +-----------+-------------+
# Championships table:
# +------+-----------+---------+---------+---------+
# | year | Wimbledon | Fr\_open | US\_open | Au\_open |
# +------+-----------+---------+---------+---------+
# | 2018 | 1         | 1       | 1       | 1       |
# | 2019 | 1         | 1       | 2       | 2       |
# | 2020 | 2         | 1       | 2       | 2       |
# +------+-----------+---------+---------+---------+
# **Output:** 
# +-----------+-------------+-------------------+
# | player\_id | player\_name | grand\_slams\_count |
# +-----------+-------------+-------------------+
# | 2         | Federer     | 5                 |
# | 1         | Nadal       | 7                 |
# +-----------+-------------+-------------------+
# **Explanation:** 
# Player 1 (Nadal) won 7 titles: Wimbledon (2018, 2019), Fr\_open (2018, 2019, 2020), US\_open (2018), and Au\_open (2018).
# Player 2 (Federer) won 5 titles: Wimbledon (2020), US\_open (2019, 2020), and Au\_open (2019, 2020).
# Player 3 (Novak) did not win anything, we did not include them in the result table.



import pandas as pd

def grand_slam_titles(players: pd.DataFrame, championships: pd.DataFrame) -> pd.DataFrame:
    return players.merge(championships.iloc[:,1:].melt(value_name="player_id",var_name="grand_slams_count").groupby("player_id",0).count(),on="player_id")