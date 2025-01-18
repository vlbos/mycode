# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `emails`

# +-------------+----------+
# | Column Name | Type     | 
# +-------------+----------+
# | email\_id    | int      |
# | user\_id     | int      |
# | signup\_date | datetime |
# +-------------+----------+
# (email\_id, user\_id) is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the email ID, user ID, and signup date.

# Table: `texts`

# +---------------+----------+
# | Column Name   | Type     | 
# +---------------+----------+
# | text\_id       | int      |
# | email\_id      | int      |
# | signup\_action | enum     |
# | action\_date   | datetime |
# +---------------+----------+
# (text\_id, email\_id) is the primary key (combination of columns with unique values) for this table. 
# signup\_action is an enum type of ('Verified', 'Not Verified'). 
# Each row of this table contains the text ID, email ID, signup action, and action date.

# Write a Solution to find the user IDs of those who **verified** their **sign-up** on the **second day**.

# Return _the result table ordered by_ `user_id` _in **ascending** order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# emails table:

# +----------+---------+---------------------+
# | email\_id | user\_id | signup\_date         |
# +----------+---------+---------------------+
# | 125      | 7771    | 2022-06-14 09:30:00|
# | 433      | 1052    | 2022-07-09 08:15:00|
# | 234      | 7005    | 2022-08-20 10:00:00|
# +----------+---------+---------------------+

# texts table:

# +---------+----------+--------------+---------------------+
# | text\_id | email\_id | signup\_action| action\_date         |
# +---------+----------+--------------+---------------------+
# | 1       | 125      | Verified     | 2022-06-15 08:30:00|
# | 2       | 433      | Not Verified | 2022-07-10 10:45:00|
# | 4       | 234      | Verified     | 2022-08-21 09:30:00|
# +---------+----------+--------------+---------------------+
    

# **Output:**

# +---------+
# | user\_id |
# +---------+
# | 7005    |
# | 7771    |
# +---------+

# **Explanation:**

# *   User with user\_id 7005 and email\_id 234 signed up on 2022-08-20 10:00:00 and verified on second day of the signup.
# *   User with user\_id 7771 and email\_id 125 signed up on 2022-06-14 09:30:00 and verified on second day of the signup.



import pandas as pd

def find_second_day_signups(emails: pd.DataFrame, texts: pd.DataFrame) -> pd.DataFrame:
    texts = texts[texts['signup_action'] == 'Verified']
    texts['signup_dt'] = (pd.to_datetime(texts['action_date']) - pd.Timedelta(1, unit='D')).dt.floor('d')
    emails['signup_dt'] = emails['signup_date'].dt.floor('d')
    df = pd.merge(emails,texts, on = ['email_id','signup_dt'], how = 'inner')
    return df[['user_id']].sort_values(by='user_id')