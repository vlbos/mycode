# [3384\. Team Dominance by Pass Success 🔒](https://leetcode.com/problems/team-dominance-by-pass-success)
# ========================================================================================================

# [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

# Description
# -----------

# Table: `Teams`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | player\_id   | int     |
# | team\_name   | varchar | 
# +-------------+---------+
# player\_id is the unique key for this table.
# Each row contains the unique identifier for player and the name of one of the teams participating in that match.

# Table: `Passes`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | pass\_from   | int     |
# | time\_stamp  | varchar |
# | pass\_to     | int     |
# +-------------+---------+
# (pass\_from, time\_stamp) is the primary key for this table.
# pass\_from is a foreign key to player\_id from Teams table.
# Each row represents a pass made during a match, time\_stamp represents the time in minutes (00:00-90:00) when the pass was made,
# pass\_to is the player\_id of the player receiving the pass.

# Write a solution to calculate the **dominance score** for each team in **both halves of the match**. The rules are as follows:

# *   A match is divided into two halves: **first half** (`00:00`\-`45:00` minutes) and **second half** (`45:01`\-`90:00` minutes)
# *   The dominance score is calculated based on successful and intercepted passes:
#     *   When pass\_to is a player from the **same team**: +`1` point
#     *   When pass\_to is a player from the **opposing team** (interception): `-1` point
# *   A higher dominance score indicates better passing performance

# Return _the result table ordered_ _by_ `team_name` and `half_number` _in **ascending** order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# Teams table:

# +------------+-----------+
# | player\_id  | team\_name |
# +------------+-----------+
# | 1          | Arsenal   |
# | 2          | Arsenal   |
# | 3          | Arsenal   |
# | 4          | Chelsea   |
# | 5          | Chelsea   |
# | 6          | Chelsea   |
# +------------+-----------+

# Passes table:

# +-----------+------------+---------+
# | pass\_from | time\_stamp | pass\_to |
# +-----------+------------+---------+
# | 1         | 00:15      | 2       |
# | 2         | 00:45      | 3       |
# | 3         | 01:15      | 1       |
# | 4         | 00:30      | 1       |
# | 2         | 46:00      | 3       |
# | 3         | 46:15      | 4       |
# | 1         | 46:45      | 2       |
# | 5         | 46:30      | 6       |
# +-----------+------------+---------+

# **Output:**

# +-----------+-------------+-----------+
# | team\_name | half\_number | dominance |
# +-----------+-------------+-----------+
# | Arsenal   | 1           | 3         |
# | Arsenal   | 2           | 1         |
# | Chelsea   | 1           | -1        |
# | Chelsea   | 2           | 1         |
# +-----------+-------------+-----------+

# **Explanation:**

# *   **First Half (00:00-45:00):**
#     *   Arsenal's passes:
#         *   1 → 2 (00:15): Successful pass (+1)
#         *   2 → 3 (00:45): Successful pass (+1)
#         *   3 → 1 (01:15): Successful pass (+1)
#     *   Chelsea's passes:
#         *   4 → 1 (00:30): Intercepted by Arsenal (-1)
# *   **Second Half (45:01-90:00):**
#     *   Arsenal's passes:
#         *   2 → 3 (46:00): Successful pass (+1)
#         *   3 → 4 (46:15): Intercepted by Chelsea (-1)
#         *   1 → 2 (46:45): Successful pass (+1)
#     *   Chelsea's passes:
#         *   5 → 6 (46:30): Successful pass (+1)
# *   The results are ordered by team\_name and then half\_number


import pandas as pd


def calculate_team_dominance(teams: pd.DataFrame, passes: pd.DataFrame) -> pd.DataFrame:
    passes_with_teams = passes.merge(
        teams, left_on="pass_from", right_on="player_id", suffixes=("", "_team_from")
    ).merge(
        teams,
        left_on="pass_to",
        right_on="player_id",
        suffixes=("_team_from", "_team_to"),
    )
    passes_with_teams["half_number"] = passes_with_teams["time_stamp"].apply(
        lambda x: 1 if x <= "45:00" else 2
    )
    passes_with_teams["dominance"] = passes_with_teams.apply(
        lambda row: 1 if row["team_name_team_from"] == row["team_name_team_to"] else -1,
        axis=1,
    )
    result = (
        passes_with_teams.groupby(["team_name_team_from", "half_number"])["dominance"]
        .sum()
        .reset_index()
    )
    result.columns = ["team_name", "half_number", "dominance"]
    result = result.sort_values(by=["team_name", "half_number"])
    return result