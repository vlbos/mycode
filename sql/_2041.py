# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Candidates`

# +--------------+----------+
# | Column Name  | Type     |
# +--------------+----------+
# | candidate\_id | int      |
# | name         | varchar  |
# | years\_of\_exp | int      |
# | interview\_id | int      |
# +--------------+----------+
# candidate\_id is the primary key (column with unique values) for this table.
# Each row of this table indicates the name of a candidate, their number of years of experience, and their interview ID.

# Table: `Rounds`

# +--------------+------+
# | Column Name  | Type |
# +--------------+------+
# | interview\_id | int  |
# | round\_id     | int  |
# | score        | int  |
# +--------------+------+
# (interview\_id, round\_id) is the primary key (combination of columns with unique values) for this table.
# Each row of this table indicates the score of one round of an interview.

# Write a solution to report the IDs of the candidates who have **at least two** years of experience and the sum of the score of their interview rounds is **strictly greater than `15`**.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Candidates table:
# +--------------+---------+--------------+--------------+
# | candidate\_id | name    | years\_of\_exp | interview\_id |
# +--------------+---------+--------------+--------------+
# | 11           | Atticus | 1            | 101          |
# | 9            | Ruben   | 6            | 104          |
# | 6            | Aliza   | 10           | 109          |
# | 8            | Alfredo | 0            | 107          |
# +--------------+---------+--------------+--------------+
# Rounds table:
# +--------------+----------+-------+
# | interview\_id | round\_id | score |
# +--------------+----------+-------+
# | 109          | 3        | 4     |
# | 101          | 2        | 8     |
# | 109          | 4        | 1     |
# | 107          | 1        | 3     |
# | 104          | 3        | 6     |
# | 109          | 1        | 4     |
# | 104          | 4        | 7     |
# | 104          | 1        | 2     |
# | 109          | 2        | 1     |
# | 104          | 2        | 7     |
# | 107          | 2        | 3     |
# | 101          | 1        | 8     |
# +--------------+----------+-------+
# **Output:** 
# +--------------+
# | candidate\_id |
# +--------------+
# | 9            |
# +--------------+
# **Explanation:** 
# - Candidate 11: The total score is 16, and they have one year of experience. We do not include them in the result table because of their years of experience.
# - Candidate 9: The total score is 22, and they have six years of experience. We include them in the result table.
# - Candidate 6: The total score is 10, and they have ten years of experience. We do not include them in the result table because the score is not good enough.
# - Candidate 8: The total score is 6, and they have zero years of experience. We do not include them in the result table because of their years of experience and the score.



import pandas as pd

def accepted_candidates(candidates: pd.DataFrame, rounds: pd.DataFrame) -> pd.DataFrame:
    return rounds.groupby("interview_id")[["score"]].sum().merge(candidates,on="interview_id").query("score>15 & years_of_exp>=2")[["candidate_id"]]