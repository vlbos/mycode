# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `students`

# +-------------+----------+
# | Column Name | Type     | 
# +-------------+----------+
# | student\_id  | int      |
# | name        | varchar  |
# | major       | varchar  |
# +-------------+----------+
# student\_id is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the student ID, student name, and their major.

# Table: `courses`

# +-------------+----------+
# | Column Name | Type     | 
# +-------------+----------+
# | course\_id   | int      |
# | name        | varchar  |
# | credits     | int      |
# | major       | varchar  |
# +-------------+----------+
# course\_id is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the course ID, course name, the number of credits for the course, and the major it belongs to.

# Table: `enrollments`

# +-------------+----------+
# | Column Name | Type     | 
# +-------------+----------+
# | student\_id  | int      |
# | course\_id   | int      |
# | semester    | varchar  |
# | grade       | varchar  |
# +-------------+----------+
# (student\_id, course\_id, semester) is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the student ID, course ID, semester, and grade received.

# Write a solution to find the students who have **taken** **all courses** offered in their `major` and have achieved a **grade of A** **in all these courses**.

# Return _the result table ordered by_ `student_id` _in **ascending** order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# students table:

# +------------+------------------+------------------+
# | student\_id | name             | major            |
# +------------+------------------+------------------+
# | 1          | Alice            | Computer Science |
# | 2          | Bob              | Computer Science |
# | 3          | Charlie          | Mathematics      |
# | 4          | David            | Mathematics      |
# +------------+------------------+------------------+

# courses table:

# +-----------+-----------------+---------+------------------+
# | course\_id | name            | credits | major            |
# +-----------+-----------------+---------+------------------+
# | 101       | Algorithms      | 3       | Computer Science |
# | 102       | Data Structures | 3       | Computer Science |
# | 103       | Calculus        | 4       | Mathematics      |
# | 104       | Linear Algebra  | 4       | Mathematics      |
# +-----------+-----------------+---------+------------------+

# enrollments table:

# +------------+-----------+----------+-------+
# | student\_id | course\_id | semester | grade |
# +------------+-----------+----------+-------+
# | 1          | 101       | Fall 2023| A     |
# | 1          | 102       | Fall 2023| A     |
# | 2          | 101       | Fall 2023| B     |
# | 2          | 102       | Fall 2023| A     |
# | 3          | 103       | Fall 2023| A     |
# | 3          | 104       | Fall 2023| A     |
# | 4          | 103       | Fall 2023| A     |
# | 4          | 104       | Fall 2023| B     |
# +------------+-----------+----------+-------+

# **Output:**

# +------------+
# | student\_id |
# +------------+
# | 1          |
# | 3          |
# +------------+

# **Explanation:**

# *   Alice (student\_id 1) is a Computer Science major and has taken both "Algorithms" and "Data Structures", receiving an 'A' in both.
# *   Bob (student\_id 2) is a Computer Science major but did not receive an 'A' in all required courses.
# *   Charlie (student\_id 3) is a Mathematics major and has taken both "Calculus" and "Linear Algebra", receiving an 'A' in both.
# *   David (student\_id 4) is a Mathematics major but did not receive an 'A' in all required courses.

# **Note:** Output table is ordered by student\_id in ascending order.



import pandas as pd

def find_top_scoring_students(enrollments: pd.DataFrame, students: pd.DataFrame, courses: pd.DataFrame) -> pd.DataFrame:
    return (
        students.merge(courses, on="major")
        .merge(enrollments, how="left")
        .assign(isA=lambda df: df["grade"] == "A")
        .groupby("student_id", as_index=False)["isA"]
        .agg(["sum", "size"])[lambda df: df["sum"] == df["size"]][["student_id"]]
        .sort_values("student_id")
    )