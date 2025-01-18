# Medium
# Topics
# Companies
# Hint
# SQL Schema
# Pandas Schema
# Table: Student

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | student_id   | int     |
# | student_name | varchar |
# | gender       | varchar |
# | dept_id      | int     |
# +--------------+---------+
# student_id is the primary key (column with unique values) for this table.
# dept_id is a foreign key (reference column) to dept_id in the Department tables.
# Each row of this table indicates the name of a student, their gender, and the id of their department.
 

# Table: Department

# +-------------+---------+
# | Column Name | Type    |
# +-------------+---------+
# | dept_id     | int     |
# | dept_name   | varchar |
# +-------------+---------+
# dept_id is the primary key (column with unique values) for this table.
# Each row of this table contains the id and the name of a department.
 

# Write a solution to report the respective department name and number of students majoring in each department for all departments in the Department table (even ones with no current students).

# Return the result table ordered by student_number in descending order. In case of a tie, order them by dept_name alphabetically.

# The result format is in the following example.

 

# Example 1:

# Input: 
# Student table:
# +------------+--------------+--------+---------+
# | student_id | student_name | gender | dept_id |
# +------------+--------------+--------+---------+
# | 1          | Jack         | M      | 1       |
# | 2          | Jane         | F      | 1       |
# | 3          | Mark         | M      | 2       |
# +------------+--------------+--------+---------+
# Department table:
# +---------+-------------+
# | dept_id | dept_name   |
# +---------+-------------+
# | 1       | Engineering |
# | 2       | Science     |
# | 3       | Law         |
# +---------+-------------+
# Output: 
# +-------------+----------------+
# | dept_name   | student_number |
# +-------------+----------------+
# | Engineering | 2              |
# | Science     | 1              |
# | Law         | 0              |
# +-------------+----------------+



import pandas as pd

def count_students(student: pd.DataFrame, department: pd.DataFrame) -> pd.DataFrame:
    student = student.merge(department, how='left', on='dept_id')
    student = student.drop(['student_name', 'gender'], axis=1)
    counts = student.groupby(by='dept_name').agg(student_number=('student_id', 'nunique')).reset_index()
    counts = department.merge(counts, how='left', on='dept_name').drop('dept_id', axis=1)
    counts['student_number'] = counts['student_number'].fillna(0)
    return counts.sort_values(by=['student_number', 'dept_name'], ascending=[False, True])
