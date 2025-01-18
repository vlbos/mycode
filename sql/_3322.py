# [3322\. Premier League Table Ranking III 🔒](https://leetcode.com/problems/premier-league-table-ranking-iii)
# ============================================================================================================

# [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

# Description
# -----------

# Table: `SeasonStats`

# +------------------+---------+
# | Column Name      | Type    |
# +------------------+---------+
# | season\_id        | int     |
# | team\_id          | int     |
# | team\_name        | varchar |
# | matches\_played   | int     |
# | wins             | int     |
# | draws            | int     |
# | losses           | int     |
# | goals\_for        | int     |
# | goals\_against    | int     |
# +------------------+---------+
# (season\_id, team\_id) is the unique key for this table.
# This table contains season id, team id, team name, matches played, wins, draws, losses, goals scored (goals\_for), and goals conceded (goals\_against) for each team in each season.

# Write a solution to calculate the **points**, **goal difference**, and **rank** for **each team** in **each season**. The ranking should be determined as follows:

# *   Teams are first ranked by their total points (highest to lowest)
# *   If points are tied, teams are then ranked by their goal difference (highest to lowest)
# *   If goal difference is also tied, teams are then ranked alphabetically by team name

# Points are calculated as follows:

# *   `3` points for a **win**
# *   `1` point for a **draw**
# *   `0` points for a **loss**

# Goal difference is calculated as: `goals_for - goals_against`

# Return _the result table ordered by_ `season_id` _in **ascending** order, then by_ `rank` _in **ascending** order, and finally by_ `team_name` _in **ascending** order._

# The query result format is in the following example.

# **Example:**

# **Input:**

# `SeasonStats` table:

# +------------+---------+-------------------+----------------+------+-------+--------+-----------+---------------+
# | season\_id  | team\_id | team\_name         | matches\_played | wins | draws | losses | goals\_for | goals\_against |
# +------------+---------+-------------------+----------------+------+-------+--------+-----------+---------------+
# | 2021       | 1       | Manchester City   | 38             | 29   | 6     | 3      | 99        | 26            |
# | 2021       | 2       | Liverpool         | 38             | 28   | 8     | 2      | 94        | 26            |
# | 2021       | 3       | Chelsea           | 38             | 21   | 11    | 6      | 76        | 33            |
# | 2021       | 4       | Tottenham         | 38             | 22   | 5     | 11     | 69        | 40            |
# | 2021       | 5       | Arsenal           | 38             | 22   | 3     | 13     | 61        | 48            |
# | 2022       | 1       | Manchester City   | 38             | 28   | 5     | 5      | 94        | 33            |
# | 2022       | 2       | Arsenal           | 38             | 26   | 6     | 6      | 88        | 43            |
# | 2022       | 3       | Manchester United | 38             | 23   | 6     | 9      | 58        | 43            |
# | 2022       | 4       | Newcastle         | 38             | 19   | 14    | 5      | 68        | 33            |
# | 2022       | 5       | Liverpool         | 38             | 19   | 10    | 9      | 75        | 47            |
# +------------+---------+-------------------+----------------+------+-------+--------+-----------+---------------+

# **Output:**

# +------------+---------+-------------------+--------+-----------------+----------+
# | season\_id  | team\_id | team\_name         | points | goal\_difference | position |
# +------------+---------+-------------------+--------+-----------------+----------+
# | 2021       | 1       | Manchester City   | 93     | 73              | 1        |
# | 2021       | 2       | Liverpool         | 92     | 68              | 2        |
# | 2021       | 3       | Chelsea           | 74     | 43              | 3        |
# | 2021       | 4       | Tottenham         | 71     | 29              | 4        |
# | 2021       | 5       | Arsenal           | 69     | 13              | 5        |
# | 2022       | 1       | Manchester City   | 89     | 61              | 1        |
# | 2022       | 2       | Arsenal           | 84     | 45              | 2        |
# | 2022       | 3       | Manchester United | 75     | 15              | 3        |
# | 2022       | 4       | Newcastle         | 71     | 35              | 4        |
# | 2022       | 5       | Liverpool         | 67     | 28              | 5        | 
# +------------+---------+-------------------+--------+-----------------+----------+

# **Explanation:**

# *   For the 2021 season:
#     *   Manchester City has 93 points (29 \* 3 + 6 \* 1) and a goal difference of 73 (99 - 26).
#     *   Liverpool has 92 points (28 \* 3 + 8 \* 1) and a goal difference of 68 (94 - 26).
#     *   Chelsea has 74 points (21 \* 3 + 11 \* 1) and a goal difference of 43 (76 - 33).
#     *   Tottenham has 71 points (22 \* 3 + 5 \* 1) and a goal difference of 29 (69 - 40).
#     *   Arsenal has 69 points (22 \* 3 + 3 \* 1) and a goal difference of 13 (61 - 48).
# *   For the 2022 season:
#     *   Manchester City has 89 points (28 \* 3 + 5 \* 1) and a goal difference of 61 (94 - 33).
#     *   Arsenal has 84 points (26 \* 3 + 6 \* 1) and a goal difference of 45 (88 - 43).
#     *   Manchester United has 75 points (23 \* 3 + 6 \* 1) and a goal difference of 15 (58 - 43).
#     *   Newcastle has 71 points (19 \* 3 + 14 \* 1) and a goal difference of 35 (68 - 33).
#     *   Liverpool has 67 points (19 \* 3 + 10 \* 1) and a goal difference of 28 (75 - 47).
# *   The teams are ranked first by points, then by goal difference, and finally by team name.
# *   The output is ordered by season\_id ascending, then by rank ascending, and finally by team\_name ascending.



import pandas as pd


def process_team_standings(season_stats: pd.DataFrame) -> pd.DataFrame:
    season_stats["points"] = season_stats["wins"] * 3 + season_stats["draws"]
    season_stats["goal_difference"] = (
        season_stats["goals_for"] - season_stats["goals_against"]
    )

    season_stats = season_stats.sort_values(
        ["season_id", "points", "goal_difference", "team_name"],
        ascending=[True, False, False, True],
    )

    season_stats["position"] = season_stats.groupby("season_id").cumcount() + 1

    return season_stats[
        ["season_id", "team_id", "team_name", "points", "goal_difference", "position"]
    ]

