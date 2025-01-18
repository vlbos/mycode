# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Calls`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | from\_id     | int     |
# | to\_id       | int     |
# | duration    | int     |
# +-------------+---------+
# This table does not have a primary key (column with unique values), it may contain duplicates.
# This table contains the duration of a phone call between from\_id and to\_id.
# from\_id != to\_id

# Write a solution to report the number of calls and the total call duration between each pair of distinct persons `(person1, person2)` where `person1 < person2`.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Calls table:
# +---------+-------+----------+
# | from\_id | to\_id | duration |
# +---------+-------+----------+
# | 1       | 2     | 59       |
# | 2       | 1     | 11       |
# | 1       | 3     | 20       |
# | 3       | 4     | 100      |
# | 3       | 4     | 200      |
# | 3       | 4     | 200      |
# | 4       | 3     | 499      |
# +---------+-------+----------+
# **Output:** 
# +---------+---------+------------+----------------+
# | person1 | person2 | call\_count | total\_duration |
# +---------+---------+------------+----------------+
# | 1       | 2       | 2          | 70             |
# | 1       | 3       | 1          | 20             |
# | 3       | 4       | 4          | 999            |
# +---------+---------+------------+----------------+
# **Explanation:** 
# Users 1 and 2 had 2 calls and the total duration is 70 (59 + 11).
# Users 1 and 3 had 1 call and the total duration is 20.
# Users 3 and 4 had 4 calls and the total duration is 999 (100 + 200 + 200 + 499).

import pandas as pd

def number_of_calls(calls: pd.DataFrame) -> pd.DataFrame:
    return (
        calls.assign(
            person1=np.where(calls.from_id < calls.to_id, calls.from_id, calls.to_id),
            person2=np.where(calls.from_id > calls.to_id, calls.from_id, calls.to_id),
        )
        .groupby(["person1", "person2"], as_index=False)
        .agg(
            call_count=("duration", "count"),
            total_duration=("duration", "sum"),
        )
    )
