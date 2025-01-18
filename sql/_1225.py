# Hard

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Failed`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | fail\_date    | date    |
# +--------------+---------+
# fail\_date is the primary key (column with unique values) for this table.
# This table contains the days of failed tasks.

# Table: `Succeeded`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | success\_date | date    |
# +--------------+---------+
# success\_date is the primary key (column with unique values) for this table.
# This table contains the days of succeeded tasks.

# A system is running one task **every day**. Every task is independent of the previous tasks. The tasks can fail or succeed.

# Write a solution to report the `period_state` for each continuous interval of days in the period from `2019-01-01` to `2019-12-31`.

# `period_state` is _'_`failed'` if tasks in this interval failed or `'succeeded'` if tasks in this interval succeeded. Interval of days are retrieved as `start_date` and `end_date.`

# Return the result table ordered by `start_date`.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Failed table:
# +-------------------+
# | fail\_date         |
# +-------------------+
# | 2018-12-28        |
# | 2018-12-29        |
# | 2019-01-04        |
# | 2019-01-05        |
# +-------------------+
# Succeeded table:
# +-------------------+
# | success\_date      |
# +-------------------+
# | 2018-12-30        |
# | 2018-12-31        |
# | 2019-01-01        |
# | 2019-01-02        |
# | 2019-01-03        |
# | 2019-01-06        |
# +-------------------+
# **Output:** 
# +--------------+--------------+--------------+
# | period\_state | start\_date   | end\_date     |
# +--------------+--------------+--------------+
# | succeeded    | 2019-01-01   | 2019-01-03   |
# | failed       | 2019-01-04   | 2019-01-05   |
# | succeeded    | 2019-01-06   | 2019-01-06   |
# +--------------+--------------+--------------+
# **Explanation:** 
# The report ignored the system state in 2018 as we care about the system in the period 2019-01-01 to 2019-12-31.
# From 2019-01-01 to 2019-01-03 all tasks succeeded and the system state was "succeeded".
# From 2019-01-04 to 2019-01-05 all tasks failed and the system state was "failed".
# From 2019-01-06 to 2019-01-06 all tasks succeeded and the system state was "succeeded".


import pandas as pd

def report_contiguous_dates(failed: pd.DataFrame, succeeded: pd.DataFrame) -> pd.DataFrame:
    failed = failed[failed['fail_date'].between('2019-01-01','2019-12-31')]
    succeed = succeeded[succeeded['success_date'].between('2019-01-01','2019-12-31')]

    failed['current_state'] = 'failed'
    succeed['current_state'] = 'succeeded'

    data = [
        failed.rename(columns={'fail_date':'date'}),
        succeed.rename(columns={'success_date':'date'})
    ]
    df_1 = pd.concat(data).sort_values(by = 'date', ascending = True)
    
    df_1['prev_state'] = df_1['current_state'].shift(periods = 1)
    df_1['start_date_flag'] = np.where(df_1['prev_state']!=df_1['current_state'],1,0)
    df_1['flag_2'] = df_1['start_date_flag'].cumsum()
    df_2 = df_1.groupby('flag_2').agg(
        period_state = ('current_state','first'),
        start_date = ('date','min'),
        end_date = ('date','max')
    )
    
    df_final= df_2.sort_values(by = 'start_date', ascending = True)
    return df_final