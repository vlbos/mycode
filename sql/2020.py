# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Subscriptions`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | account\_id  | int  |
# | start\_date  | date |
# | end\_date    | date |
# +-------------+------+
# account\_id is the primary key column for this table.
# Each row of this table indicates the start and end dates of an account's subscription.
# Note that always start\_date < end\_date.

# Table: `Streams`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | session\_id  | int  |
# | account\_id  | int  |
# | stream\_date | date |
# +-------------+------+
# session\_id is the primary key column for this table.
# account\_id is a foreign key from the Subscriptions table.
# Each row of this table contains information about the account and the date associated with a stream session.

# Write an SQL query to report the number of accounts that bought a subscription in `2021` but did not have any stream session.

# The query result format is in the following example.

# **Example 1:**

# **Input:** 
# Subscriptions table:
# +------------+------------+------------+
# | account\_id | start\_date | end\_date   |
# +------------+------------+------------+
# | 9          | 2020-02-18 | 2021-10-30 |
# | 3          | 2021-09-21 | 2021-11-13 |
# | 11         | 2020-02-28 | 2020-08-18 |
# | 13         | 2021-04-20 | 2021-09-22 |
# | 4          | 2020-10-26 | 2021-05-08 |
# | 5          | 2020-09-11 | 2021-01-17 |
# +------------+------------+------------+
# Streams table:
# +------------+------------+-------------+
# | session\_id | account\_id | stream\_date |
# +------------+------------+-------------+
# | 14         | 9          | 2020-05-16  |
# | 16         | 3          | 2021-10-27  |
# | 18         | 11         | 2020-04-29  |
# | 17         | 13         | 2021-08-08  |
# | 19         | 4          | 2020-12-31  |
# | 13         | 5          | 2021-01-05  |
# +------------+------------+-------------+
# **Output:** 
# +----------------+
# | accounts\_count |
# +----------------+
# | 2              |
# +----------------+
# **Explanation:** Users 4 and 9 did not stream in 2021.
# User 11 did not subscribe in 2021.



import pandas as pd

def find_target_accounts(subscriptions: pd.DataFrame, streams: pd.DataFrame) -> pd.DataFrame:

    st = streams[streams.stream_date.dt.year == 2021]

    df = subscriptions[(subscriptions.start_date <= '2021-12-31') &
                       (subscriptions.end_date.dt.year >= 2021) &
                       (~subscriptions.account_id.isin(st.account_id))]

    return pd.DataFrame({'accounts_count': [len(df)]})