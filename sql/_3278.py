  
# Medium

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Candidates`

# +--------------+---------+ 
# | Column Name  | Type    | 
# +--------------+---------+ 
# | candidate\_id | int     | 
# | skill        | varchar |
# | proficiency  | int     |
# +--------------+---------+
# (candidate\_id, skill) is the unique key for this table.
# Each row includes candidate\_id, skill, and proficiency level (1-5).

# Table: `Projects`

# +--------------+---------+ 
# | Column Name  | Type    | 
# +--------------+---------+ 
# | project\_id   | int     | 
# | skill        | varchar |
# | importance   | int     |
# +--------------+---------+
# (project\_id, skill) is the primary key for this table.
# Each row includes project\_id, required skill, and its importance (1-5) for the project.

# Leetcode is staffing for multiple data science projects. Write a solution to find the **best candidate** for **each project** based on the following criteria:

# 1.  Candidates must have **all** the skills required for a project.
# 2.  Calculate a **score** for each candidate-project pair as follows:
#     *   **Start** with `100` points
#     *   **Add** `10` points for each skill where **proficiency > importance**
#     *   **Subtract** `5` points for each skill where **proficiency < importance**

# Include only the top candidate (highest score) for each project. If there’s a **tie**, choose the candidate with the **lower** `candidate_id`. If there is **no suitable candidate** for a project, **do not return** that project.

# Return a result table ordered by `project_id` in ascending order.

# The result format is in the following example.

# **Example:**

# **Input:**

# `Candidates` table:

# +--------------+-----------+-------------+
# | candidate\_id | skill     | proficiency |
# +--------------+-----------+-------------+
# | 101          | Python    | 5           |
# | 101          | Tableau   | 3           |
# | 101          | PostgreSQL| 4           |
# | 101          | TensorFlow| 2           |
# | 102          | Python    | 4           |
# | 102          | Tableau   | 5           |
# | 102          | PostgreSQL| 4           |
# | 102          | R         | 4           |
# | 103          | Python    | 3           |
# | 103          | Tableau   | 5           |
# | 103          | PostgreSQL| 5           |
# | 103          | Spark     | 4           |
# +--------------+-----------+-------------+

# `Projects` table:

# +-------------+-----------+------------+
# | project\_id  | skill     | importance |
# +-------------+-----------+------------+
# | 501         | Python    | 4          |
# | 501         | Tableau   | 3          |
# | 501         | PostgreSQL| 5          |
# | 502         | Python    | 3          |
# | 502         | Tableau   | 4          |
# | 502         | R         | 2          |
# +-------------+-----------+------------+

# **Output:**

# +-------------+--------------+-------+
# | project\_id  | candidate\_id | score |
# +-------------+--------------+-------+
# | 501         | 101          | 105   |
# | 502         | 102          | 130   |
# +-------------+--------------+-------+

# **Explanation:**

# *   For Project 501, Candidate 101 has the highest score of 105. All other candidates have the same score but Candidate 101 has the lowest candidate\_id among them.
# *   For Project 502, Candidate 102 has the highest score of 130.

# The output table is ordered by project\_id in ascending order.


import pandas as pd

def find_best_candidates(candidates: pd.DataFrame, projects: pd.DataFrame) -> pd.DataFrame:
    merged = pd.merge(candidates, projects, on='skill')
    
    merged['score'] = (10 * (merged['proficiency'] > merged['importance'])) - (5 * (merged['proficiency'] < merged['importance']))
    
    candidate_scores = merged.groupby(['candidate_id', 'project_id']).agg(
        total_score=('score', 'sum'),
        num_skills=('skill', 'count'),
        candidate_skills=('skill', set)
    ).reset_index()
    
    required_skills = projects.groupby('project_id').agg(
    required_skills=('skill', set)
    ).reset_index()
    
    merged_scores = pd.merge(candidate_scores, required_skills, on='project_id', how='inner')

    filtered_scores = merged_scores[merged_scores.apply(
        lambda row: row['required_skills'].issubset(row['candidate_skills']), axis=1
        )]
    
    candidate_scores = filtered_scores.sort_values(by=['project_id', 'total_score', 'candidate_id'], ascending=[True, False, True])
    
    best_candidates = candidate_scores.groupby('project_id').first().reset_index()

    best_candidates['score'] = best_candidates['total_score'] + 100
    
    return best_candidates[['project_id', 'candidate_id', 'score']]
    