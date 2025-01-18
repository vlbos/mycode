# Easy

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Teams`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | team\_name   | varchar |
# +-------------+---------+
# team\_name is the column with unique values of this table.
# Each row of this table shows the name of a team.

# Write a solution to report all the possible matches of the league. Note that every two teams play two matches with each other, with one team being the `home_team` once and the other time being the `away_team`.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Teams table:
# +-------------+
# | team\_name   |
# +-------------+
# | Leetcode FC |
# | Ahly SC     |
# | Real Madrid |
# +-------------+
# **Output:** 
# +-------------+-------------+
# | home\_team   | away\_team   |
# +-------------+-------------+
# | Real Madrid | Leetcode FC |
# | Real Madrid | Ahly SC     |
# | Leetcode FC | Real Madrid |
# | Leetcode FC | Ahly SC     |
# | Ahly SC     | Real Madrid |
# | Ahly SC     | Leetcode FC |
# +-------------+-------------+
# **Explanation:** All the matches of the league are shown in the table.


import pandas as pd

def find_all_matches(teams: pd.DataFrame) -> pd.DataFrame:
    return teams.merge(teams, how='cross').query('team_name_x != team_name_y').rename(columns={'team_name_x':'home_team', 'team_name_y':'away_team'})