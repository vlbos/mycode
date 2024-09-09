# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Sessions`

# +---------------+----------+
# | Column Name   | Type     |
# +---------------+----------+
# | user\_id       | int      |
# | session\_start | datetime |
# | session\_end   | datetime |
# | session\_id    | int      |
# | session\_type  | enum     |
# +---------------+----------+
# session\_id is column of unique values for this table.
# session\_type is an ENUM (category) type of (Viewer, Streamer).
# This table contains user id, session start, session end, session id and session type.

# Write a solution to find the number of **streaming** sessions for users whose **first session** was as a **viewer**.

# Return _the result table ordered by count of streaming sessions,_ `user_id` _in **descending** order._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Sessions table:
# +---------+---------------------+---------------------+------------+--------------+
# | user\_id | session\_start       | session\_end         | session\_id | session\_type | 
# +---------+---------------------+---------------------+------------+--------------+
# | 101     | 2023-11-06 13:53:42 | 2023-11-06 14:05:42 | 375        | Viewer       |  
# | 101     | 2023-11-22 16:45:21 | 2023-11-22 20:39:21 | 594        | Streamer     |   
# | 102     | 2023-11-16 13:23:09 | 2023-11-16 16:10:09 | 777        | Streamer     | 
# | 102     | 2023-11-17 13:23:09 | 2023-11-17 16:10:09 | 778        | Streamer     | 
# | 101     | 2023-11-20 07:16:06 | 2023-11-20 08:33:06 | 315        | Streamer     | 
# | 104     | 2023-11-27 03:10:49 | 2023-11-27 03:30:49 | 797        | Viewer       | 
# | 103     | 2023-11-27 03:10:49 | 2023-11-27 03:30:49 | 798        | Streamer     |  
# +---------+---------------------+---------------------+------------+--------------+
# **Output:** 
# +---------+----------------+
# | user\_id | sessions\_count | 
# +---------+----------------+
# | 101     | 2              | 
# +---------+----------------+
# **Explanation**
# - user\_id 101, initiated their initial session as a viewer on 2023-11-06 at 13:53:42, followed by two subsequent sessions as a Streamer, the count will be 2.
# - user\_id 102, although there are two sessions, the initial session was as a Streamer, so this user will be excluded.
# - user\_id 103 participated in only one session, which was as a Streamer, hence, it won't be considered.
# - User\_id 104 commenced their first session as a viewer but didn't have any subsequent sessions, therefore, they won't be included in the final count. 
# Output table is ordered by sessions count and user\_id in descending order.


import pandas as pd

def count_turned_streamers(sessions: pd.DataFrame) -> pd.DataFrame:
    sessions['rank'] = sessions.groupby('user_id')['session_start'].rank()
    sessions['boolean'] = np.where(sessions['session_type'] == 'Viewer', 0, 1)
    sessions['sessions_count'] = sessions.groupby('user_id')['boolean'].transform('sum')
    return sessions[(sessions['session_type'] == 'Viewer') & (sessions['rank'] == 1) & (sessions['sessions_count'] >= 1)][['user_id', 'sessions_count']].sort_values(['sessions_count', 'user_id'], ascending=[False, False]).drop_duplicates(['user_id', 'sessions_count'], keep='first')