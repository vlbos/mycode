# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Teams`

# +----------------+---------+
# | Column Name    | Type    |
# +----------------+---------+
# | team\_id        | int     |
# | team\_name      | varchar |
# +----------------+---------+
# team\_id is the column with unique values for this table.
# Each row contains information about one team in the league.

# Table: `Matches`

# +-----------------+---------+
# | Column Name     | Type    |
# +-----------------+---------+
# | home\_team\_id    | int     |
# | away\_team\_id    | int     |
# | home\_team\_goals | int     |
# | away\_team\_goals | int     |
# +-----------------+---------+
# (home\_team\_id, away\_team\_id) is the primary key (combination of columns with unique values) for this table.
# Each row contains information about one match.
# home\_team\_goals is the number of goals scored by the home team.
# away\_team\_goals is the number of goals scored by the away team.
# The winner of the match is the team with the higher number of goals.

# Write a solution to report the statistics of the league. The statistics should be built using the played matches where the **winning** team gets **three points** and the **losing** team gets **no points**. If a match ends with a **draw**, both teams get **one point**.

# Each row of the result table should contain:

# *   `team_name` \- The name of the team in the `Teams` table.
# *   `matches_played` \- The number of matches played as either a home or away team.
# *   `points` \- The total points the team has so far.
# *   `goal_for` \- The total number of goals scored by the team across all matches.
# *   `goal_against` \- The total number of goals scored by opponent teams against this team across all matches.
# *   `goal_diff` \- The result of `goal_for - goal_against`.

# Return the result table ordered by `points` **in descending order**. If two or more teams have the same `points`, order them by `goal_diff` **in descending order**. If there is still a tie, order them by `team_name` in **lexicographical order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Teams table:
# +---------+-----------+
# | team\_id | team\_name |
# +---------+-----------+
# | 1       | Ajax      |
# | 4       | Dortmund  |
# | 6       | Arsenal   |
# +---------+-----------+
# Matches table:
# +--------------+--------------+-----------------+-----------------+
# | home\_team\_id | away\_team\_id | home\_team\_goals | away\_team\_goals |
# +--------------+--------------+-----------------+-----------------+
# | 1            | 4            | 0               | 1               |
# | 1            | 6            | 3               | 3               |
# | 4            | 1            | 5               | 2               |
# | 6            | 1            | 0               | 0               |
# +--------------+--------------+-----------------+-----------------+
# **Output:** 
# +-----------+----------------+--------+----------+--------------+-----------+
# | team\_name | matches\_played | points | goal\_for | goal\_against | goal\_diff |
# +-----------+----------------+--------+----------+--------------+-----------+
# | Dortmund  | 2              | 6      | 6        | 2            | 4         |
# | Arsenal   | 2              | 2      | 3        | 3            | 0         |
# | Ajax      | 4              | 2      | 5        | 9            | -4        |
# +-----------+----------------+--------+----------+--------------+-----------+
# **Explanation:** 
# Ajax (team\_id=1) played 4 matches: 2 losses and 2 draws. Total points = 0 + 0 + 1 + 1 = 2.
# Dortmund (team\_id=4) played 2 matches: 2 wins. Total points = 3 + 3 = 6.
# Arsenal (team\_id=6) played 2 matches: 2 draws. Total points = 1 + 1 = 2.
# Dortmund is the first team in the table. Ajax and Arsenal have the same points, but since Arsenal has a higher goal\_diff than Ajax, Arsenal comes before Ajax in the table.



import pandas as pd

def league_statistics(teams: pd.DataFrame, matches: pd.DataFrame) -> pd.DataFrame:
    df = matches
    df = pd.concat([df, df.rename(columns={df.columns[i]:df.columns[j] for i,j in ((0,1),(1,0),(2,3),(3,2))})])
    df = teams.merge(df, left_on="team_id", right_on="home_team_id")
    df = df.rename(columns = {"home_team_goals":"goal_for","away_team_goals":"goal_against"})
    df["points"] = np.where(df.goal_for > df.goal_against, 3, (df.goal_for==df.goal_against).astype(int))
    df = df.assign(goal_diff = df.goal_for - df.goal_against, matches_played = 1)
    df = df.iloc[:,[1,8,6,4,5,7]].groupby("team_name",as_index=0).sum()
    return df.sort_values(["points","goal_diff","team_name"], ascending = [0,0,1])