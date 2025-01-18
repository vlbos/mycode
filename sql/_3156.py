# Hard

# Topics

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Tasks`

# +---------------+----------+
# | Column Name   | Type     |
# +---------------+----------+
# | task\_id       | int      |
# | employee\_id   | int      |
# | start\_time    | datetime |
# | end\_time      | datetime |
# +---------------+----------+
# (task\_id, employee\_id) is the primary key for this table.
# Each row in this table contains the task identifier, the employee identifier, and the start and end times of each task.

# Write a solution to find the **total duration** of tasks for **each** employee and the **maximum number of concurrent tasks** an employee handled at **any point in time**. The total duration should be **rounded down** to the nearest number of **full hours**.

# Return _the result table ordered by_ `employee_id` **_ascending_** _order_.

# The result format is in the following example.

# **Example:**

# **Input:**

# Tasks table:

# +---------+-------------+---------------------+---------------------+
# | task\_id | employee\_id | start\_time          | end\_time            |
# +---------+-------------+---------------------+---------------------+
# | 1       | 1001        | 2023-05-01 08:00:00 | 2023-05-01 09:00:00 |
# | 2       | 1001        | 2023-05-01 08:30:00 | 2023-05-01 10:30:00 |
# | 3       | 1001        | 2023-05-01 11:00:00 | 2023-05-01 12:00:00 |
# | 7       | 1001        | 2023-05-01 13:00:00 | 2023-05-01 15:30:00 |
# | 4       | 1002        | 2023-05-01 09:00:00 | 2023-05-01 10:00:00 |
# | 5       | 1002        | 2023-05-01 09:30:00 | 2023-05-01 11:30:00 |
# | 6       | 1003        | 2023-05-01 14:00:00 | 2023-05-01 16:00:00 |
# +---------+-------------+---------------------+---------------------+

# **Output:**

# +-------------+------------------+----------------------+
# | employee\_id | total\_task\_hours | max\_concurrent\_tasks |
# +-------------+------------------+----------------------+
# | 1001        | 6                | 2                    |
# | 1002        | 2                | 2                    |
# | 1003        | 2                | 1                    |
# +-------------+------------------+----------------------+

# **Explanation:**

# *   For employee ID 1001:
#     *   Task 1 and Task 2 overlap from 08:30 to 09:00 (30 minutes).
#     *   Task 7 has a duration of 150 minutes (2 hours and 30 minutes).
#     *   Total task time: 60 (Task 1) + 120 (Task 2) + 60 (Task 3) + 150 (Task 7) - 30 (overlap) = 360 minutes = 6 hours.
#     *   Maximum concurrent tasks: 2 (during the overlap period).
# *   For employee ID 1002:
#     *   Task 4 and Task 5 overlap from 09:30 to 10:00 (30 minutes).
#     *   Total task time: 60 (Task 4) + 120 (Task 5) - 30 (overlap) = 150 minutes = 2 hours and 30 minutes.
#     *   Total task hours (rounded down): 2 hours.
#     *   Maximum concurrent tasks: 2 (during the overlap period).
# *   For employee ID 1003:
#     *   No overlapping tasks.
#     *   Total task time: 120 minutes = 2 hours.
#     *   Maximum concurrent tasks: 1.

# **Note:** Output table is ordered by employee\_id in ascending order.



def find_total_duration(tasks: pd.DataFrame) -> pd.DataFrame:
    IDX = ["employee_id", "task_id"]
    EMPS = set(tasks.employee_id)

    tasks["task_duration"] = (tasks.end_time - tasks.start_time).dt.total_seconds()

    overlap = (
        tasks.merge(tasks, on="employee_id", suffixes=("_a", "_b"))
        .query("start_time_b > start_time_a & start_time_b < end_time_a")
        .rename(columns={"task_id_a": "task_id"})
    )
    overlap["end"] = np.minimum(overlap.end_time_a, overlap.end_time_b)
    overlap["overlap_duration"] = (
        overlap.end - overlap.start_time_b
    ).dt.total_seconds()

    max_concurrent = (
        (overlap.groupby(["employee_id", "task_id"]).size() + 1)
        .reset_index()
        .groupby("employee_id")[0]
        .max()
        .reindex(EMPS, fill_value=1)
    )
    max_concurrent.name = "max_concurrent_tasks"

    out = (
        pd.concat((tasks.set_index(IDX), overlap.set_index(IDX)), axis=1)
        .fillna(0)
        .groupby("employee_id")
        .apply(
            lambda grp: int(
                (grp.task_duration - grp.overlap_duration).sum() / (60 * 60)
            )
        )
    )
    out.name = "total_task_hours"
    
    return (
        pd.concat((out, max_concurrent), axis=1)
        .reset_index()
        .sort_values("employee_id")
    )