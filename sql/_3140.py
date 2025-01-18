# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Cinema`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | seat\_id     | int  |
# | free        | bool |
# +-------------+------+
# seat\_id is an auto-increment column for this table.
# Each row of this table indicates whether the ith seat is free or not. 1 means free while 0 means occupied.

# Write a solution to find the **length** of **longest consecutive sequence** of **available** seats in the cinema.

# Note:

# *   There will always be **at most** **one** longest consecutive sequence.
# *   If there are **multiple** consecutive sequences with the **same length**, include all of them in the output.

# Return _the result table **ordered** by_ `first_seat_id` _**in ascending order**_.

# The result format is in the following example.

# **Example:**

# **Input:**

# Cinema table:

# +---------+------+
# | seat\_id | free |
# +---------+------+
# | 1       | 1    |
# | 2       | 0    |
# | 3       | 1    |
# | 4       | 1    |
# | 5       | 1    |
# +---------+------+

# **Output:**

# +-----------------+----------------+-----------------------+
# | first\_seat\_id   | last\_seat\_id   | consecutive\_seats\_len |
# +-----------------+----------------+-----------------------+
# | 3               | 5              | 3                     |
# +-----------------+----------------+-----------------------+

# **Explanation:**

# *   Longest consecutive sequence of available seats starts from seat 3 and ends at seat 5 with a length of 3.

# Output table is ordered by first\_seat\_id in ascending order.



import pandas as pd
import itertools as iter

def consecutive_available_seats(cinema: pd.DataFrame) -> pd.DataFrame:

    cinema = cinema[(cinema.free == 1)].sort_values('seat_id')

    groups = [list(map(lambda x: x[1], g)) for k, g in 
            iter.groupby(enumerate(cinema.seat_id),lambda x: x[0]-x[1])]

    df = pd.DataFrame({'first_seat_id':list(map(lambda x: x[0],groups)),
                       'last_seat_id' :list(map(lambda x: x[-1],groups))})

    df['consecutive_seats_len'] = df.last_seat_id - df.first_seat_id + 1

    return df[df.consecutive_seats_len == df.consecutive_seats_len.max()]