# Hard

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Spending`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | user\_id     | int     |
# | spend\_date  | date    |
# | platform    | enum    | 
# | amount      | int     |
# +-------------+---------+
# The table logs the history of the spending of users that make purchases from an online shopping website that has a desktop and a mobile application.
# (user\_id, spend\_date, platform) is the primary key (combination of columns with unique values) of this table.
# The platform column is an ENUM (category) type of ('desktop', 'mobile').

# Write a solution to find the total number of users and the total amount spent using the mobile only, the desktop only, and both mobile and desktop together for each date.

# Return the result table in **any order**.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Spending table:
# +---------+------------+----------+--------+
# | user\_id | spend\_date | platform | amount |
# +---------+------------+----------+--------+
# | 1       | 2019-07-01 | mobile   | 100    |
# | 1       | 2019-07-01 | desktop  | 100    |
# | 2       | 2019-07-01 | mobile   | 100    |
# | 2       | 2019-07-02 | mobile   | 100    |
# | 3       | 2019-07-01 | desktop  | 100    |
# | 3       | 2019-07-02 | desktop  | 100    |
# +---------+------------+----------+--------+
# **Output:** 
# +------------+----------+--------------+-------------+
# | spend\_date | platform | total\_amount | total\_users |
# +------------+----------+--------------+-------------+
# | 2019-07-01 | desktop  | 100          | 1           |
# | 2019-07-01 | mobile   | 100          | 1           |
# | 2019-07-01 | both     | 200          | 1           |
# | 2019-07-02 | desktop  | 100          | 1           |
# | 2019-07-02 | mobile   | 100          | 1           |
# | 2019-07-02 | both     | 0            | 0           |
# +------------+----------+--------------+-------------+ 
# **Explanation:** 
# On 2019-07-01, user 1 purchased using **both** desktop and mobile, user 2 purchased using mobile **only** and user 3 purchased using desktop **only**.
# On 2019-07-02, user 2 purchased using mobile **only**, user 3 purchased using desktop **only** and no one purchased using **both** platforms.



import pandas as pd

def user_purchase(spending: pd.DataFrame) -> pd.DataFrame:
    df = spending.groupby(["user_id","spend_date"]).agg(cnt=("platform","count"),total_amount=("amount","sum")).reset_index()
    df = df.merge(spending, on = ["user_id","spend_date"]).drop_duplicates(subset=["user_id","spend_date","cnt"])
    df = df.assign(platform = np.where(df["cnt"] == 2, "both", df["platform"]))
    df = df.groupby(["spend_date","platform"]).agg(total_amount=("total_amount","sum"), total_users=("user_id","count"))

    dates = sorted(list(pd.unique(spending["spend_date"])))
    frame = pd.DataFrame({"spend_date":dates*3, "platform": [x for x in ["desktop","mobile","both"] for _ in range(len(dates))]})
    return frame.merge(df, on = ["spend_date","platform"], how = "left").fillna(0)