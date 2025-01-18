# Hard

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
# student\_id is the primary key for this table. 
# Each row contains the student ID, student name, and their major.

# Table: `courses`

# +-------------+-------------------+
# | Column Name | Type              |       
# +-------------+-------------------+
# | course\_id   | int               |    
# | name        | varchar           |      
# | credits     | int               |           
# | major       | varchar           |       
# | mandatory   | enum              |      
# +-------------+-------------------+
# course\_id is the primary key for this table. 
# mandatory is an enum type of ('Yes', 'No').
# Each row contains the course ID, course name, credits, major it belongs to, and whether the course is mandatory.

# Table: `enrollments`

# +-------------+----------+
# | Column Name | Type     | 
# +-------------+----------+
# | student\_id  | int      |
# | course\_id   | int      |
# | semester    | varchar  |
# | grade       | varchar  |
# | GPA         | decimal  | 
# +-------------+----------+
# (student\_id, course\_id, semester) is the primary key (combination of columns with unique values) for this table.
# Each row contains the student ID, course ID, semester, and grade received.

# Write a solution to find the students who meet the following criteria:

# *   Have **taken all mandatory courses** and **at least two** elective courses offered in **their major.**
# *   Achieved a grade of **A** in **all mandatory courses** and at least **B** in **elective courses**.
# *   Maintained an average `GPA` of at least `2.5` across all their courses (including those outside their major).

# Return _the result table ordered by_ `student_id` _in **ascending** order_.

# **Example:**

# **Input:**

# students table:

#  +------------+------------------+------------------+
#  | student\_id | name             | major            |
#  +------------+------------------+------------------+
#  | 1          | Alice            | Computer Science |
#  | 2          | Bob              | Computer Science |
#  | 3          | Charlie          | Mathematics      |
#  | 4          | David            | Mathematics      |
#  +------------+------------------+------------------+
 

# courses table:

#  +-----------+-------------------+---------+------------------+----------+
#  | course\_id | name              | credits | major            | mandatory|
#  +-----------+-------------------+---------+------------------+----------+
#  | 101       | Algorithms        | 3       | Computer Science | yes      |
#  | 102       | Data Structures   | 3       | Computer Science | yes      |
#  | 103       | Calculus          | 4       | Mathematics      | yes      |
#  | 104       | Linear Algebra    | 4       | Mathematics      | yes      |
#  | 105       | Machine Learning  | 3       | Computer Science | no       |
#  | 106       | Probability       | 3       | Mathematics      | no       |
#  | 107       | Operating Systems | 3       | Computer Science | no       |
#  | 108       | Statistics        | 3       | Mathematics      | no       |
#  +-----------+-------------------+---------+------------------+----------+
 

# enrollments table:

#  +------------+-----------+-------------+-------+-----+
#  | student\_id | course\_id | semester    | grade | GPA |
#  +------------+-----------+-------------+-------+-----+
#  | 1          | 101       | Fall 2023   | A     | 4.0 |
#  | 1          | 102       | Spring 2023 | A     | 4.0 |
#  | 1          | 105       | Spring 2023 | A     | 4.0 |
#  | 1          | 107       | Fall 2023   | B     | 3.5 |
#  | 2          | 101       | Fall 2023   | A     | 4.0 |
#  | 2          | 102       | Spring 2023 | B     | 3.0 |
#  | 3          | 103       | Fall 2023   | A     | 4.0 |
#  | 3          | 104       | Spring 2023 | A     | 4.0 |
#  | 3          | 106       | Spring 2023 | A     | 4.0 |
#  | 3          | 108       | Fall 2023   | B     | 3.5 |
#  | 4          | 103       | Fall 2023   | B     | 3.0 |
#  | 4          | 104       | Spring 2023 | B     | 3.0 |
#  +------------+-----------+-------------+-------+-----+
 

# **Output:**

#  +------------+
#  | student\_id |
#  +------------+
#  | 1          |
#  | 3          |
#  +------------+
 

# **Explanation:**

# *   Alice (student\_id 1) is a Computer Science major and has taken both Algorithms and Data Structures, receiving an A in both. She has also taken Machine Learning and Operating Systems as electives, receiving an A and B respectively.
# *   Bob (student\_id 2) is a Computer Science major but did not receive an A in all required courses.
# *   Charlie (student\_id 3) is a Mathematics major and has taken both Calculus and Linear Algebra, receiving an A in both. He has also taken Probability and Statistics as electives, receiving an A and B respectively.
# *   David (student\_id 4) is a Mathematics major but did not receive an A in all required courses.

# **Note:** Output table is ordered by student\_id in ascending order.




import pandas as pd

def find_top_scoring_students(students: pd.DataFrame,
                              courses: pd.DataFrame, 
                              enrollments: pd.DataFrame) -> pd.DataFrame:
    
    def isQualified(stu_id, maj):

        stu_courses = df[df.student_id == stu_id]
        man_courses = df_mand[df_mand.major == maj]
        ele_courses = df_elec[df_elec.major == maj]
        
        mandatory_taken = stu_courses[stu_courses.course_id
                                            .isin(man_courses.course_id)]
        if set(man_courses.course_id) != set(mandatory_taken.course_id): return False
        
        elective_taken = stu_courses[stu_courses.course_id.isin(ele_courses.course_id)]

        return len(elective_taken) > 1


    df = enrollments.merge(students, on = 'student_id', how = 'left'
                   ).merge(courses , on = 'course_id' , how = 'left')

    df = df[~df.student_id.isin(set(df[(~df.grade.isin(['A', 'B'])) | 
                      ((df.grade != 'A') & (df.mandatory == 'Yes') & 
                       (df.major_x == df.major_y))].student_id))]

    df = df[df.student_id.isin(set(df.groupby('student_id')
              .filter(lambda x: x.GPA.mean() >= 2.5).student_id))]

    df_mand = courses[courses.mandatory == 'Yes']
    df_elec = courses[courses.mandatory ==  'No']

    return students[students.apply(lambda row: isQualified(row.student_id, 
                 row.major), axis=1)].sort_values('student_id')[['student_id']]

    
    