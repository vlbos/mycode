# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `TeamStats`

# +------------------+---------+
# | Column Name      | Type    |
# +------------------+---------+
# | team\_id          | int     |
# | team\_name        | varchar |
# | matches\_played   | int     |
# | wins             | int     |
# | draws            | int     |
# | losses           | int     |
# +------------------+---------+
# team\_id is the unique key for this table.
# This table contains team id, team name, matches\_played, wins, draws, and losses.

# Write a solution to calculate the **points** and **rank** for each team in the league. Points are calculated as follows:

# *   `3` points for a **win**
# *   `1` point for a **draw**
# *   `0` points for a **loss**

# **Note:** Teams with the same points must be assigned the same rank.

# Return _the result table ordered by_ `points` _in **descending**, and then by_ `team_name` _in **ascending** order._

# The query result format is in the following example.

# **Example:**

# **Input:**

# `TeamStats` table:

# +---------+-----------------+----------------+------+-------+--------+
# | team\_id | team\_name       | matches\_played | wins | draws | losses |
# +---------+-----------------+----------------+------+-------+--------+
# | 1       | Manchester City | 10             | 6    | 2     | 2      |
# | 2       | Liverpool       | 10             | 6    | 2     | 2      |
# | 3       | Chelsea         | 10             | 5    | 3     | 2      |
# | 4       | Arsenal         | 10             | 4    | 4     | 2      |
# | 5       | Tottenham       | 10             | 3    | 5     | 2      |
# +---------+-----------------+----------------+------+-------+--------+

# **Output:**

# +---------+-----------------+--------+----------+
# | team\_id | team\_name       | points | position |
# +---------+-----------------+--------+----------+
# | 2       | Liverpool       | 20     | 1        |
# | 1       | Manchester City | 20     | 1        |
# | 3       | Chelsea         | 18     | 3        |
# | 4       | Arsenal         | 16     | 4        |
# | 5       | Tottenham       | 14     | 5        |
# +---------+-----------------+--------+----------+

# **Explanation:**

# *   Manchester City and Liverpool both have 20 points (6 wins \* 3 points + 2 draws \* 1 point), so they share position 1.
# *   Chelsea has 18 points (5 wins \* 3 points + 3 draws \* 1 point) and is position 3rd.
# *   Arsenal has 16 points (4 wins \* 3 points + 4 draws \* 1 point) and is position 4th.
# *   Tottenham has 14 points (3 wins \* 3 points + 5 draws \* 1 point) and is position 5th.

# The output table is ordered by points in descending order, then by team\_name in ascending order.



import pandas as pd

def calculate_team_standings(team_stats: pd.DataFrame) -> pd.DataFrame:
    return (
        team_stats.assign(
            points=team_stats["wins"] * 3 + team_stats["draws"],
            position=lambda df: df["points"].rank(method="min", ascending=False),
        ).sort_values(["points", "team_name"], ascending=[False, True])
    )[["team_id", "team_name", "points", "position"]]