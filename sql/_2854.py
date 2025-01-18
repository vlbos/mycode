# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Steps`

# +-------------+------+ 
# | Column Name | Type | 
# +-------------+------+ 
# | user\_id     | int  | 
# | steps\_count | int  |
# | steps\_date  | date |
# +-------------+------+
# (user\_id, steps\_date) is the primary key for this table.
# Each row of this table contains user\_id, steps\_count, and steps\_date.

# Write a solution to calculate `3-day` **rolling averages** of steps for each user.

# We calculate the `n-day` **rolling average** this way:

# *   For each day, we calculate the average of `n` consecutive days of step counts ending on that day if available, otherwise, `n-day` rolling average is not defined for it.

# Output the `user_id`, `steps_date`, and rolling average. Round the rolling average to **two decimal places**.

# Return _the result table ordered by_ `user_id`_,_ `steps_date` _in **ascending** order._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Steps table:
# +---------+-------------+------------+
# | user\_id | steps\_count | steps\_date |
# +---------+-------------+------------+
# | 1       | 687         | 2021-09-02 |
# | 1       | 395         | 2021-09-04 |
# | 1       | 499         | 2021-09-05 |
# | 1       | 712         | 2021-09-06 |
# | 1       | 576         | 2021-09-07 |
# | 2       | 153         | 2021-09-06 |
# | 2       | 171         | 2021-09-07 |
# | 2       | 530         | 2021-09-08 |
# | 3       | 945         | 2021-09-04 |
# | 3       | 120         | 2021-09-07 |
# | 3       | 557         | 2021-09-08 |
# | 3       | 840         | 2021-09-09 |
# | 3       | 627         | 2021-09-10 |
# | 5       | 382         | 2021-09-05 |
# | 6       | 480         | 2021-09-01 |
# | 6       | 191         | 2021-09-02 |
# | 6       | 303         | 2021-09-05 |
# +---------+-------------+------------+
# **Output:** 
# +---------+------------+-----------------+
# | user\_id | steps\_date | rolling\_average | 
# +---------+------------+-----------------+
# | 1       | 2021-09-06 | 535.33          | 
# | 1       | 2021-09-07 | 595.67          | 
# | 2       | 2021-09-08 | 284.67          |
# | 3       | 2021-09-09 | 505.67          |
# | 3       | 2021-09-10 | 674.67          |    
# +---------+------------+-----------------+
# **Explanation:** 
# - For user id 1, the step counts for the three consecutive days up to 2021-09-06 are available. Consequently, the rolling average for this particular date is computed as (395 + 499 + 712) / 3 = 535.33.
# - For user id 1, the step counts for the three consecutive days up to 2021-09-07 are available. Consequently, the rolling average for this particular date is computed as (499 + 712 + 576) / 3 = 595.67.
# - For user id 2, the step counts for the three consecutive days up to 2021-09-08 are available. Consequently, the rolling average for this particular date is computed as (153 + 171 + 530) / 3 = 284.67.
# - For user id 3, the step counts for the three consecutive days up to 2021-09-09 are available. Consequently, the rolling average for this particular date is computed as (120 + 557 + 840) / 3 = 505.67.
# - For user id 3, the step counts for the three consecutive days up to 2021-09-10 are available. Consequently, the rolling average for this particular date is computed as (557 + 840 + 627) / 3 = 674.67.
# - For user id 4 and 5, the calculation of the rolling average is not viable as there is insufficient data for the consecutive three days. Output table ordered by user\_id and steps\_date in ascending order.



import pandas as pd

def rolling_average(steps: pd.DataFrame) -> pd.DataFrame:
    return (
        steps.sort_values("steps_date")
        .groupby("user_id", as_index=False)
        .rolling("3D", on="steps_date", min_periods=3)
        .mean()
        .round(2)
        .dropna()
        .rename(columns={"steps_count": "rolling_average"})[
            ["user_id", "steps_date", "rolling_average"]
        ]
    )