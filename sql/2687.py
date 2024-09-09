# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Bikes`

# +-------------+----------+ 
# | Column Name | Type     | 
# +-------------+----------+ 
# | ride\_id     | int      | 
# | bike\_number | int      | 
# | start\_time  | datetime |
# | end\_time    | datetime |
# +-------------+----------+
# ride\_id column contains unique values.
# Each row contains a ride information that includes ride\_id, bike number, start and end time of the ride.

# Write a solution to find the **last** **time** when each bike was used.

# Return the result table ordered by the bikes that were **most recently used**. 

# The result format is in the following example.

# **Example 1:**

# **Input:** `Bikes` table:
# +---------+-------------+---------------------+---------------------+ 
# | ride\_id | bike\_number | start\_time          | end\_time            |  
# +---------+-------------+---------------------+---------------------+
# | 1       | W00576      | 2012-03-25 11:30:00 | 2012-03-25 12:40:00 |
# | 2       | W00300      | 2012-03-25 10:30:00 | 2012-03-25 10:50:00 |
# | 3       | W00455      | 2012-03-26 14:30:00 | 2012-03-26 17:40:00 |
# | 4       | W00455      | 2012-03-25 12:30:00 | 2012-03-25 13:40:00 |
# | 5       | W00576      | 2012-03-25 08:10:00 | 2012-03-25 09:10:00 |
# | 6       | W00576      | 2012-03-28 02:30:00 | 2012-03-28 02:50:00 |
# +---------+-------------+---------------------+---------------------+ 

# **Output:**
# +-------------+---------------------+ 
# | bike\_number | end\_time            |  
# +-------------+---------------------+
# | W00576      | 2012-03-28 02:50:00 |
# | W00455      | 2012-03-26 17:40:00 |
# | W00300      | 2012-03-25 10:50:00 |
# +-------------+---------------------+ 
# **Explanation:** 
# bike with number W00576 has three rides, out of that, most recent ride is with ride\_id 6 which ended on 2012-03-28 02:50:00.
# bike with number W00300 has only 1 ride so we will include end\_time in output directly. 
# bike with number W00455 has two rides, out of that, most recent ride is with ride\_id 3 which ended on 2012-03-26 17:40:00. 
# Returning output in order by the bike that were most recently used.




import pandas as pd

def last_used_time(bikes: pd.DataFrame) -> pd.DataFrame:
    return bikes.sort_values(by=['bike_number','end_time'],ascending=[1,0]).drop_duplicates(subset=['bike_number']).sort_values('end_time',ascending=False)[['bike_number','end_time']]