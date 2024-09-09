# Easy

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Salaries`

# +-------------+---------+ 
# | Column Name | Type    | 
# +-------------+---------+ 
# | emp\_name    | varchar | 
# | department  | varchar | 
# | salary      | int     |
# +-------------+---------+
# (emp\_name, department) is the primary key (combination of unique values) for this table.
# Each row of this table contains emp\_name, department and salary. There will be **at least one** entry for the engineering and marketing departments.

# Write a solution to calculate the difference between the **highest** salaries in the **marketing** and **engineering** `department`. Output the absolute difference in salaries.

# Return _the result table._

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Salaries table:
# +----------+-------------+--------+
# | emp\_name | department  | salary |
# +----------+-------------+--------+
# | Kathy    | Engineering | 50000  |
# | Roy      | Marketing   | 30000  |
# | Charles  | Engineering | 45000  |
# | Jack     | Engineering | 85000  | 
# | Benjamin | Marketing   | 34000  |
# | Anthony  | Marketing   | 42000  |
# | Edward   | Engineering | 102000 |
# | Terry    | Engineering | 44000  |
# | Evelyn   | Marketing   | 53000  |
# | Arthur   | Engineering | 32000  |
# +----------+-------------+--------+
# **Output:** 
# +-------------------+
# | salary\_difference | 
# +-------------------+
# | 49000             | 
# +-------------------+
# **Explanation:** 
# - The Engineering and Marketing departments have the highest salaries of 102,000 and 53,000, respectively. Resulting in an absolute difference of 49,000.



import pandas as pd

def salaries_difference(salaries: pd.DataFrame) -> pd.DataFrame:
    df = salaries.groupby('department', as_index =False)['salary'].max()
    df['salary_difference'] = abs(df.loc[df.department == 'Engineering', 'salary'].values[0] - df.loc[df.department == 'Marketing', 'salary'].values[0])
    return df[['salary_difference']].drop_duplicates()