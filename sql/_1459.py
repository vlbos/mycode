# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Points`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | id            | int     |
# | x\_value       | int     |
# | y\_value       | int     |
# +---------------+---------+
# id is the column with unique values for this table.
# Each point is represented as a 2D coordinate (x\_value, y\_value).

# Write a solution to report all possible **axis-aligned** rectangles with a **non-zero area** that can be formed by any two points from the `Points` table.

# Each row in the result should contain three columns `(p1, p2, area)` where:

# *   `p1` and `p2` are the `id`'s of the two points that determine the opposite corners of a rectangle.
# *   `area` is the area of the rectangle and must be **non-zero**.

# Return the result table **ordered** by `area` **in descending order**. If there is a tie, order them by `p1` **in ascending order**. If there is still a tie, order them by `p2` **in ascending order**.

# The result format is in the following table.

# **Example 1:**

# ![](https://assets.leetcode.com/uploads/2021/03/12/rect.png)

# **Input:** 
# Points table:
# +----------+-------------+-------------+
# | id       | x\_value     | y\_value     |
# +----------+-------------+-------------+
# | 1        | 2           | 7           |
# | 2        | 4           | 8           |
# | 3        | 2           | 10          |
# +----------+-------------+-------------+
# **Output:** 
# +----------+-------------+-------------+
# | p1       | p2          | area        |
# +----------+-------------+-------------+
# | 2        | 3           | 4           |
# | 1        | 2           | 2           |
# +----------+-------------+-------------+
# **Explanation:** 
# The rectangle formed by p1 = 2 and p2 = 3 has an area equal to |4-2| \* |8-10| = 4.
# The rectangle formed by p1 = 1 and p2 = 2 has an area equal to |2-4| \* |7-8| = 2.
# Note that the rectangle formed by p1 = 1 and p2 = 3 is invalid because the area is 0.



import pandas as pd

def rectangles_area(points: pd.DataFrame) -> pd.DataFrame:
    df = points.merge(points,"cross").query("id_x<id_y").rename(columns = {"id_x":"P1", "id_y":"P2"})
    df = df.assign(AREA = abs(df.x_value_x - df.x_value_y) * abs(df.y_value_x - df.y_value_y))
    return df.query("AREA > 0")[["P1","P2","AREA"]].sort_values(["AREA","P1","P2"], ascending = [0,1,1])