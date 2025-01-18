# [3401\. Find Circular Gift Exchange Chains 🔒](https://leetcode.com/problems/find-circular-gift-exchange-chains)
# ================================================================================================================

# [![](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Hard-4051B5?style=flat-square)

# Description
# -----------

# Table: `SecretSanta`

# +-------------+------+
# | Column Name | Type |
# +-------------+------+
# | giver\_id    | int  |
# | receiver\_id | int  |
# | gift\_value  | int  |
# +-------------+------+
# (giver\_id, receiver\_id) is the unique key for this table.   
# Each row represents a record of a gift exchange between two employees, giver\_id represents the employee who gives a gift, receiver\_id represents the employee who receives the gift and gift\_value represents the value of the gift given.  

# Write a solution to find the **total gift value** and **length** of **circular chains** of Secret Santa gift exchanges:

# A **circular chain** is defined as a series of exchanges where:

# *   Each employee gives a gift to **exactly one** other employee.
# *   Each employee receives a gift **from exactly** one other employee.
# *   The exchanges form a continuous **loop** (e.g., employee A gives a gift to B, B gives to C, and C gives back to A).

# Return _the result ordered by the chain length and total gift value of the chain in **descending** order_. 

# The result format is in the following example.

# **Example:**

# **Input:**

# SecretSanta table:

# +----------+-------------+------------+
# | giver\_id | receiver\_id | gift\_value |
# +----------+-------------+------------+
# | 1        | 2           | 20         |
# | 2        | 3           | 30         |
# | 3        | 1           | 40         |
# | 4        | 5           | 25         |
# | 5        | 4           | 35         |
# +----------+-------------+------------+

# **Output:**

# +----------+--------------+------------------+
# | chain\_id | chain\_length | total\_gift\_value |
# +----------+--------------+------------------+
# | 1        | 3            | 90               |
# | 2        | 2            | 60               |
# +----------+--------------+------------------+

# **Explanation:**

# *   **Chain 1** involves employees 1, 2, and 3:
#     *   Employee 1 gives a gift to 2, employee 2 gives a gift to 3, and employee 3 gives a gift to 1.
#     *   Total gift value for this chain = 20 + 30 + 40 = 90.
# *   **Chain 2** involves employees 4 and 5:
#     *   Employee 4 gives a gift to 5, and employee 5 gives a gift to 4.
#     *   Total gift value for this chain = 25 + 35 = 60.

# The result table is ordered by the chain length and total gift value of the chain in descending order.



WITH RECURSIVE
  Chains AS (
    SELECT *, giver_id AS start_id
    FROM SecretSanta
    UNION ALL
    SELECT SecretSanta.*, Chains.start_id
    FROM SecretSanta
    INNER JOIN Chains
      ON (
        SecretSanta.giver_id = Chains.receiver_id
        AND SecretSanta.giver_id != Chains.start_id) -- Avoid cycles.
  ),
  ChainSummary AS (
    SELECT
      start_id,
      COUNT(*) AS chain_length,
      SUM(gift_value) AS total_gift_value
    FROM Chains
    GROUP BY 1
  ),
  UniqueChains AS (
    SELECT DISTINCT chain_length, total_gift_value
    FROM ChainSummary
  )
SELECT
  RANK() OVER(
    ORDER BY chain_length DESC, total_gift_value DESC
  ) AS chain_id,
  chain_length,
  total_gift_value
FROM UniqueChains;