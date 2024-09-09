# Hard

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Players`

# +-------------+-------+
# | Column Name | Type  |
# +-------------+-------+
# | player\_id   | int   |
# | group\_id    | int   |
# +-------------+-------+
# player\_id is the primary key (column with unique values) of this table.
# Each row of this table indicates the group of each player.

# Table: `Matches`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | match\_id      | int     |
# | first\_player  | int     |
# | second\_player | int     | 
# | first\_score   | int     |
# | second\_score  | int     |
# +---------------+---------+
# match\_id is the primary key (column with unique values) of this table.
# Each row is a record of a match, first\_player and second\_player contain the player\_id of each match.
# first\_score and second\_score contain the number of points of the first\_player and second\_player respectively.
# You may assume that, in each match, players belong to the same group.

# The winner in each group is the player who scored the maximum total points within the group. In the case of a tie, the **lowest** `player_id` wins.

# Write a solution to find the winner in each group.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Players table:
# +-----------+------------+
# | player\_id | group\_id   |
# +-----------+------------+
# | 15        | 1          |
# | 25        | 1          |
# | 30        | 1          |
# | 45        | 1          |
# | 10        | 2          |
# | 35        | 2          |
# | 50        | 2          |
# | 20        | 3          |
# | 40        | 3          |
# +-----------+------------+
# Matches table:
# +------------+--------------+---------------+-------------+--------------+
# | match\_id   | first\_player | second\_player | first\_score | second\_score |
# +------------+--------------+---------------+-------------+--------------+
# | 1          | 15           | 45            | 3           | 0            |
# | 2          | 30           | 25            | 1           | 2            |
# | 3          | 30           | 15            | 2           | 0            |
# | 4          | 40           | 20            | 5           | 2            |
# | 5          | 35           | 50            | 1           | 1            |
# +------------+--------------+---------------+-------------+--------------+
# **Output:** 
# +-----------+------------+
# | group\_id  | player\_id  |
# +-----------+------------+ 
# | 1         | 15         |
# | 2         | 35         |
# | 3         | 40         |
# +-----------+------------+


import pandas as pd

def tournament_winners(players: pd.DataFrame, matches: pd.DataFrame) -> pd.DataFrame:
    df = (pd.concat([matches.iloc[:,[1,3]]
                       .rename(columns = {'first_player' :'player_id'}),

                     matches.iloc[:,[2,4]]
                       .rename(columns = {'second_player':'player_id',
                                          'second_score' :'first_score'})])

          .groupby(['player_id'])['first_score'].sum().reset_index()
          .merge(players, on= 'player_id', how = 'left'))

    df['rnk'] = (df.groupby(['group_id'])['first_score']
                   .rank(method = 'first', ascending = False))
        
    return df[df.rnk == 1].iloc[:,[2,0]]