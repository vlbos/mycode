# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Teams`

# +---------------+----------+
# | Column Name   | Type     |
# +---------------+----------+
# | team\_id       | int      |
# | team\_name     | varchar  |
# +---------------+----------+
# team\_id is the column with unique values of this table.
# Each row of this table represents a single football team.

# Table: `Matches`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | match\_id      | int     |
# | host\_team     | int     |
# | guest\_team    | int     | 
# | host\_goals    | int     |
# | guest\_goals   | int     |
# +---------------+---------+
# match\_id is the column of unique values of this table.
# Each row is a record of a finished match between two different teams. 
# Teams host\_team and guest\_team are represented by their IDs in the Teams table (team\_id), and they scored host\_goals and guest\_goals goals, respectively.

# You would like to compute the scores of all teams after all matches. Points are awarded as follows:

# *   A team receives **three points** if they win a match (i.e., Scored more goals than the opponent team).
# *   A team receives **one point** if they draw a match (i.e., Scored the same number of goals as the opponent team).
# *   A team receives **no points** if they lose a match (i.e., Scored fewer goals than the opponent team).

# Write a solution that selects the `team_id`, `team_name` and `num_points` of each team in the tournament after all described matches.

# Return the result table ordered by `num_points` **in decreasing order**. In case of a tie, order the records by `team_id` **in increasing order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Teams table:
# +-----------+--------------+
# | team\_id   | team\_name    |
# +-----------+--------------+
# | 10        | Leetcode FC  |
# | 20        | NewYork FC   |
# | 30        | Atlanta FC   |
# | 40        | Chicago FC   |
# | 50        | Toronto FC   |
# +-----------+--------------+
# Matches table:
# +------------+--------------+---------------+-------------+--------------+
# | match\_id   | host\_team    | guest\_team    | host\_goals  | guest\_goals  |
# +------------+--------------+---------------+-------------+--------------+
# | 1          | 10           | 20            | 3           | 0            |
# | 2          | 30           | 10            | 2           | 2            |
# | 3          | 10           | 50            | 5           | 1            |
# | 4          | 20           | 30            | 1           | 0            |
# | 5          | 50           | 30            | 1           | 0            |
# +------------+--------------+---------------+-------------+--------------+
# **Output:** 
# +------------+--------------+---------------+
# | team\_id    | team\_name    | num\_points    |
# +------------+--------------+---------------+
# | 10         | Leetcode FC  | 7             |
# | 20         | NewYork FC   | 3             |
# | 50         | Toronto FC   | 3             |
# | 30         | Atlanta FC   | 1             |
# | 40         | Chicago FC   | 0             |
# +------------+--------------+---------------+


import pandas as pd

def team_scores(teams: pd.DataFrame, matches: pd.DataFrame) -> pd.DataFrame:
    host = matches[['host_team','host_goals','guest_goals' ]].rename(columns = {'host_team':'team_id'})
    guest = matches[['guest_team','guest_goals', 'host_goals']].rename(columns = {'guest_team':'team_id'})
    host['num_points'] = np.where(host['host_goals']>host['guest_goals'], 3,np.where(host['host_goals'] == host['guest_goals'], 1, 0))
    guest['num_points'] = np.where(guest['host_goals']<guest['guest_goals'], 3,np.where(guest['host_goals'] == guest['guest_goals'], 1, 0))
    result = pd.concat([host[['team_id', 'num_points']], guest[['team_id' ,'num_points']]]).groupby(['team_id']).sum().reset_index()
    result = pd.merge(teams, result, how = 'left', on = 'team_id' ).fillna(0)
    return result.sort_values(['num_points','team_id'], ascending = [False, True])