# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Members`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | member\_id   | int     |
# | name        | varchar |
# +-------------+---------+
# member\_id is the column with unique values for this table.
# Each row of this table indicates the name and the ID of a member.

# Table: `Visits`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | visit\_id    | int  |
# | member\_id   | int  |
# | visit\_date  | date |
# +-------------+------+
# visit\_id is the column with unique values for this table.
# member\_id is a foreign key (reference column) to member\_id from the Members table.
# Each row of this table contains information about the date of a visit to the store and the member who visited it.

# Table: `Purchases`

# +----------------+------+
# | Column Name    | Type |
# +----------------+------+
# | visit\_id       | int  |
# | charged\_amount | int  |
# +----------------+------+
# visit\_id is the column with unique values for this table.
# visit\_id is a foreign key (reference column) to visit\_id from the Visits table.
# Each row of this table contains information about the amount charged in a visit to the store.

# A store wants to categorize its members. There are three tiers:

# *   **"Diamond"**: if the conversion rate is **greater than or equal to** `80`.
# *   **"Gold"**: if the conversion rate is **greater than or equal to** `50` and less than `80`.
# *   **"Silver"**: if the conversion rate is **less than** `50`.
# *   **"Bronze"**: if the member never visited the store.

# The **conversion rate** of a member is `(100 * total number of purchases for the member) / total number of visits for the member`.

# Write a solution to report the id, the name, and the category of each member.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Members table:
# +-----------+---------+
# | member\_id | name    |
# +-----------+---------+
# | 9         | Alice   |
# | 11        | Bob     |
# | 3         | Winston |
# | 8         | Hercy   |
# | 1         | Narihan |
# +-----------+---------+
# Visits table:
# +----------+-----------+------------+
# | visit\_id | member\_id | visit\_date |
# +----------+-----------+------------+
# | 22       | 11        | 2021-10-28 |
# | 16       | 11        | 2021-01-12 |
# | 18       | 9         | 2021-12-10 |
# | 19       | 3         | 2021-10-19 |
# | 12       | 11        | 2021-03-01 |
# | 17       | 8         | 2021-05-07 |
# | 21       | 9         | 2021-05-12 |
# +----------+-----------+------------+
# Purchases table:
# +----------+----------------+
# | visit\_id | charged\_amount |
# +----------+----------------+
# | 12       | 2000           |
# | 18       | 9000           |
# | 17       | 7000           |
# +----------+----------------+
# **Output:** 
# +-----------+---------+----------+
# | member\_id | name    | category |
# +-----------+---------+----------+
# | 1         | Narihan | Bronze   |
# | 3         | Winston | Silver   |
# | 8         | Hercy   | Diamond  |
# | 9         | Alice   | Gold     |
# | 11        | Bob     | Silver   |
# +-----------+---------+----------+
# **Explanation:** 
# - User Narihan with id = 1 did not make any visits to the store. She gets a Bronze category.
# - User Winston with id = 3 visited the store one time and did not purchase anything. The conversion rate = (100 \* 0) / 1 = 0. He gets a Silver category.
# - User Hercy with id = 8 visited the store one time and purchased one time. The conversion rate = (100 \* 1) / 1 = 1. He gets a Diamond category.
# - User Alice with id = 9 visited the store two times and purchased one time. The conversion rate = (100 \* 1) / 2 = 50. She gets a Gold category.
# - User Bob with id = 11 visited the store three times and purchased one time. The conversion rate = (100 \* 1) / 3 = 33.33. He gets a Silver category.


import pandas as pd

def find_categories(members: pd.DataFrame, visits: pd.DataFrame, purchases: pd.DataFrame) -> pd.DataFrame:
    df = visits.merge(purchases,"left","visit_id")
    df = df.groupby("member_id").agg(P =("charged_amount","count"),V =("visit_id","count"))
    df = df.assign(ratio = 1.0 * df.P / df.V).reset_index()
    df["category"] = np.where(df.ratio < 0.5,   "Silver",
                     np.where(df.ratio < 0.8,   "Gold",
                                                "Diamond"))
    return members.merge(df,"left","member_id")[["member_id","name","category"]].fillna("Bronze")