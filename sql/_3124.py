# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Contacts`

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | id          | int     |
# | first\_name  | varchar |
# | last\_name   | varchar |
# +-------------+---------+
# id is the primary key (column with unique values) of this table.
# id is a foreign key (reference column) to `Calls` table.
# Each row of this table contains id, first\_name, and last\_name.

# Table: `Calls`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | contact\_id  | int  |
# | type        | enum |
# | duration    | int  |
# +-------------+------+
# (contact\_id, type, duration) is the primary key (column with unique values) of this table.
# type is an ENUM (category) type of ('incoming', 'outgoing').
# Each row of this table contains information about calls, comprising of contact\_id, type, and duration in seconds.

# Write a solution to find the **three longest** **incoming** and **outgoing** calls.

# Return t_he result table ordered by_ `type`, `duration`, and `first_name` _in **descending** order and `duration` must be formatted as **HH:MM:SS**._

# The result format is in the following example.

# **Example 1:**

# **Input:**

# Contacts table:

# +----+------------+-----------+
# | id | first\_name | last\_name |
# +----+------------+-----------+
# | 1  | John       | Doe       |
# | 2  | Jane       | Smith     |
# | 3  | Alice      | Johnson   |
# | 4  | Michael    | Brown     |
# | 5  | Emily      | Davis     |
# +----+------------+-----------+        

# Calls table:

# +------------+----------+----------+
# | contact\_id | type     | duration |
# +------------+----------+----------+
# | 1          | incoming | 120      |
# | 1          | outgoing | 180      |
# | 2          | incoming | 300      |
# | 2          | outgoing | 240      |
# | 3          | incoming | 150      |
# | 3          | outgoing | 360      |
# | 4          | incoming | 420      |
# | 4          | outgoing | 200      |
# | 5          | incoming | 180      |
# | 5          | outgoing | 280      |
# +------------+----------+----------+
        

# **Output:**

# +-----------+----------+-------------------+
# | first\_name| type     | duration\_formatted|
# +-----------+----------+-------------------+
# | Alice     | outgoing | 00:06:00          |
# | Emily     | outgoing | 00:04:40          |
# | Jane      | outgoing | 00:04:00          |
# | Michael   | incoming | 00:07:00          |
# | Jane      | incoming | 00:05:00          |
# | Emily     | incoming | 00:03:00          |
# +-----------+----------+-------------------+
        

# **Explanation:**

# *   Alice had an outgoing call lasting 6 minutes.
# *   Emily had an outgoing call lasting 4 minutes and 40 seconds.
# *   Jane had an outgoing call lasting 4 minutes.
# *   Michael had an incoming call lasting 7 minutes.
# *   Jane had an incoming call lasting 5 minutes.
# *   Emily had an incoming call lasting 3 minutes.

# **Note:** Output table is sorted by type, duration, and first\_name in descending order.



import pandas as pd

def find_longest_calls(contacts: pd.DataFrame, calls: pd.DataFrame) -> pd.DataFrame:
    merged = contacts.merge(calls,left_on='id',right_on='contact_id')
    merged['rank'] = merged.groupby('type')['duration'].rank(ascending=False,method='first')
    result = merged.query('rank<=3')[['first_name','type','duration']]
    result['duration'] = pd.to_datetime(result['duration'], unit='s').dt.time
    return result.rename(columns={'duration':'duration_formatted'}).sort_values(['type','duration_formatted','first_name'],ascending=[False,False,False])

    
