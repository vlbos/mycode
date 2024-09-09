# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `EmployeeShifts`

# +------------------+---------+
# | Column Name      | Type    |
# +------------------+---------+
# | employee\_id      | int     |
# | start\_time       | time    |
# | end\_time         | time    |
# +------------------+---------+
# (employee\_id, start\_time) is the unique key for this table.
# This table contains information about the shifts worked by employees, including the start and end times on a specific date.

# Write a solution to count the number of **overlapping shifts** for each employee. Two shifts are considered overlapping if one shift’s `end_time` is **later than** another shift’s `start_time`.

# _Return the result table ordered by_ `employee_id` _in **ascending** order_.

# The query result format is in the following example.

# **Example:**

# **Input:**

# `EmployeeShifts` table:

# +-------------+------------+----------+
# | employee\_id | start\_time | end\_time |
# +-------------+------------+----------+
# | 1           | 08:00:00   | 12:00:00 |
# | 1           | 11:00:00   | 15:00:00 |
# | 1           | 14:00:00   | 18:00:00 |
# | 2           | 09:00:00   | 17:00:00 |
# | 2           | 16:00:00   | 20:00:00 |
# | 3           | 10:00:00   | 12:00:00 |
# | 3           | 13:00:00   | 15:00:00 |
# | 3           | 16:00:00   | 18:00:00 |
# | 4           | 08:00:00   | 10:00:00 |
# | 4           | 09:00:00   | 11:00:00 |
# +-------------+------------+----------+

# **Output:**

# +-------------+--------------------+
# | employee\_id | overlapping\_shifts |
# +-------------+--------------------+
# | 1           | 2                  |
# | 2           | 1                  |
# | 4           | 1                  |
# +-------------+--------------------+

# **Explanation:**

# *   Employee 1 has 3 shifts:
#     *   08:00:00 to 12:00:00
#     *   11:00:00 to 15:00:00
#     *   14:00:00 to 18:00:00The first shift overlaps with the second, and the second overlaps with the third, resulting in 2 overlapping shifts.
# *   Employee 2 has 2 shifts:
#     *   09:00:00 to 17:00:00
#     *   16:00:00 to 20:00:00These shifts overlap with each other, resulting in 1 overlapping shift.
# *   Employee 3 has 3 shifts:
#     *   10:00:00 to 12:00:00
#     *   13:00:00 to 15:00:00
#     *   16:00:00 to 18:00:00None of these shifts overlap, so Employee 3 is not included in the output.
# *   Employee 4 has 2 shifts:
#     *   08:00:00 to 10:00:00
#     *   09:00:00 to 11:00:00These shifts overlap with each other, resulting in 1 overlapping shift.

# The output shows the employee\_id and the count of overlapping shifts for each employee who has at least one overlapping shift, ordered by employee\_id in ascending order.




import pandas as pd

def find_overlapping_shifts(employee_shifts: pd.DataFrame) -> pd.DataFrame:
    merged = employee_shifts.merge(
        employee_shifts,
        on='employee_id',
        suffixes=('_1', '_2')
    )
    
    overlapping = merged[
        (merged['start_time_1'] < merged['start_time_2']) &
        (merged['end_time_1'] > merged['start_time_2'])
    ]
    
    result = overlapping.groupby('employee_id').size().reset_index(name='overlapping_shifts')
    
    result = result.sort_values(by='employee_id').reset_index(drop=True)
    
    return result    