# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Calls`

# +--------------+----------+
# | Column Name  | Type     |
# +--------------+----------+
# | caller\_id    | int      |
# | recipient\_id | int      |
# | call\_time    | datetime |
# | city         | varchar  |
# +--------------+----------+
# (caller\_id, recipient\_id, call\_time) is the primary key (combination of columns with unique values) for this table.
# Each row contains caller id, recipient id, call time, and city.

# Write a solution to find the **peak** calling **hour** for each `city`. If **multiple hours** have the **same** number of calls, all of those hours will be recognized as **peak hours** for that specific city.

# Return _the result table ordered by **peak calling hour** and_ `city` _in **descending**_ _order._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Calls table:
# +-----------+--------------+---------------------+----------+
# | caller\_id | recipient\_id | call\_time           | city     |
# +-----------+--------------+---------------------+----------+
# | 8         | 4            | 2021-08-24 22:46:07 | Houston  |
# | 4         | 8            | 2021-08-24 22:57:13 | Houston  |  
# | 5         | 1            | 2021-08-11 21:28:44 | Houston  |  
# | 8         | 3            | 2021-08-17 22:04:15 | Houston  |
# | 11        | 3            | 2021-08-17 13:07:00 | New York |
# | 8         | 11           | 2021-08-17 14:22:22 | New York |
# +-----------+--------------+---------------------+----------+
# **Output:** 
# +----------+-------------------+-----------------+
# | city     | peak\_calling\_hour | number\_of\_calls |
# +----------+-------------------+-----------------+
# | Houston  | 22                | 3               |
# | New York | 14                | 1               |
# | New York | 13                | 1               |
# +----------+-------------------+-----------------+
# **Explanation:** 
# For Houston:
#   - The peak time is 22:00, with a total of 3 calls recorded. 
# For New York:
#   - Both 13:00 and 14:00 hours have equal call counts of 1, so both times are considered peak hours.
# Output table is ordered by peak\_calling\_hour and city in descending order.



import pandas as pd

def peak_calling_hours(calls: pd.DataFrame) -> pd.DataFrame:
    calls['peak_calling_hour'] = calls['call_time'].astype(str).str[11:13].astype(int)
    x = calls.groupby(["city", "peak_calling_hour"]).size().reset_index().rename(columns={0:"number_of_calls"})
    x2 = x.groupby(["city"]).agg({"number_of_calls":"max"}).reset_index()
    return x.merge(x2, how="inner", on=["city", "number_of_calls"]).sort_values(["peak_calling_hour", "city"], ascending=False)
    