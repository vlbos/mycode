# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Student`

# +---------------------+---------+
# | Column Name         | Type    |
# +---------------------+---------+
# | student\_id          | int     |
# | student\_name        | varchar |
# +---------------------+---------+
# student\_id is the primary key (column with unique values) for this table.
# student\_name is the name of the student.

# Table: `Exam`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | exam\_id       | int     |
# | student\_id    | int     |
# | score         | int     |
# +---------------+---------+
# (exam\_id, student\_id) is the primary key (combination of columns with unique values) for this table.
# Each row of this table indicates that the student with student\_id had a score points in the exam with id exam\_id.

# A **quiet student** is the one who took at least one exam and did not score the highest or the lowest score.

# Write a solution to report the students `(student_id, student_name)` being quiet in all exams. Do not return the student who has never taken any exam.

# Return the result table **ordered** by `student_id`.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Student table:
# +-------------+---------------+
# | student\_id  | student\_name  |
# +-------------+---------------+
# | 1           | Daniel        |
# | 2           | Jade          |
# | 3           | Stella        |
# | 4           | Jonathan      |
# | 5           | Will          |
# +-------------+---------------+
# Exam table:
# +------------+--------------+-----------+
# | exam\_id    | student\_id   | score     |
# +------------+--------------+-----------+
# | 10         |     1        |    70     |
# | 10         |     2        |    80     |
# | 10         |     3        |    90     |
# | 20         |     1        |    80     |
# | 30         |     1        |    70     |
# | 30         |     3        |    80     |
# | 30         |     4        |    90     |
# | 40         |     1        |    60     |
# | 40         |     2        |    70     |
# | 40         |     4        |    80     |
# +------------+--------------+-----------+
# **Output:** 
# +-------------+---------------+
# | student\_id  | student\_name  |
# +-------------+---------------+
# | 2           | Jade          |
# +-------------+---------------+
# **Explanation:** 
# For exam 1: Student 1 and 3 hold the lowest and high scores respectively.
# For exam 2: Student 1 hold both highest and lowest score.
# For exam 3 and 4: Studnet 1 and 4 hold the lowest and high scores respectively.
# Student 2 and 5 have never got the highest or lowest in any of the exams.
# Since student 5 is not taking any exam, he is excluded from the result.
# So, we only return the information of Student 2.



import pandas as pd

def find_quiet_students(student: pd.DataFrame, exam: pd.DataFrame) -> pd.DataFrame:
  exam["max_score"] = exam.groupby("exam_id").score.transform(max)
  exam["min_score"] = exam.groupby("exam_id").score.transform(min)
  not_quiet = exam.query("score == max_score | score == min_score").student_id
  return student.loc[~student["student_id"].isin(not_quiet) & student["student_id"].isin(exam.student_id)]