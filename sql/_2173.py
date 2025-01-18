# Hard

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Matches`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | player\_id   | int  |
# | match\_day   | date |
# | result      | enum |
# +-------------+------+
# (player\_id, match\_day) is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the ID of a player, the day of the match they played, and the result of that match.
# The result column is an ENUM (category) type of ('Win', 'Draw', 'Lose').

# The **winning streak** of a player is the number of consecutive wins uninterrupted by draws or losses.

# Write a solution to count the longest winning streak for each player.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Matches table:
# +-----------+------------+--------+
# | player\_id | match\_day  | result |
# +-----------+------------+--------+
# | 1         | 2022-01-17 | Win    |
# | 1         | 2022-01-18 | Win    |
# | 1         | 2022-01-25 | Win    |
# | 1         | 2022-01-31 | Draw   |
# | 1         | 2022-02-08 | Win    |
# | 2         | 2022-02-06 | Lose   |
# | 2         | 2022-02-08 | Lose   |
# | 3         | 2022-03-30 | Win    |
# +-----------+------------+--------+
# **Output:** 
# +-----------+----------------+
# | player\_id | longest\_streak |
# +-----------+----------------+
# | 1         | 3              |
# | 2         | 0              |
# | 3         | 1              |
# +-----------+----------------+
# **Explanation:** 
# Player 1:
# From 2022-01-17 to 2022-01-25, player 1 won 3 consecutive matches.
# On 2022-01-31, player 1 had a draw.
# On 2022-02-08, player 1 won a match.
# The longest winning streak was 3 matches.

# Player 2:
# From 2022-02-06 to 2022-02-08, player 2 lost 2 consecutive matches.
# The longest winning streak was 0 matches.

# Player 3:
# On 2022-03-30, player 3 won a match.
# The longest winning streak was 1 match.

# **Follow up:** If we are interested in calculating the longest streak without losing (i.e., win or draw), how will your solution change?



import pandas as pd

def longest_winning_streak(matches: pd.DataFrame) -> pd.DataFrame:
    matches = matches.sort_values(['player_id', 'match_day'], ascending = [True, True])
    matches['prev_res'] = matches['result'].shift()
    matches['prev_id'] = matches['player_id'].shift()

    matches['streak'] = ((matches['prev_id'] != matches['player_id'])|(matches['prev_res'] != matches['result'])).cumsum()

    group = matches.groupby(['player_id', 'streak', 'result']).agg({'match_day':'nunique'}).reset_index()
    group = group[group['result'] == 'Win']

    group1 = group.groupby(['player_id']).agg({'match_day':'max'}).reset_index()
    
    temp = pd.DataFrame(data = {'player_id':matches['player_id'].unique()})
    output = pd.merge(left = temp, right = group1, on = 'player_id', how = 'left').fillna(0)
    output = output.rename(columns = {'match_day':'longest_streak'})
    return output[['player_id', 'longest_streak']]
    