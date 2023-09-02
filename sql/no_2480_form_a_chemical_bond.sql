-- # [2480. Form a Chemical Bond](https://leetcode.com/problems/form-a-chemical-bond)


-- ## Description

-- Table: Elements


-- +-------------+---------+
-- | Column Name | Type    |
-- +-------------+---------+
-- | symbol      | varchar |
-- | type        | enum    |
-- | electrons   | int     |
-- +-------------+---------+
-- symbol is the primary key for this table.
-- Each row of this table contains information of one element.
-- type is an ENUM of type (&#39;Metal&#39;, &#39;Nonmetal&#39;, &#39;Noble&#39;)
--  - If type is Noble, electrons is 0.
--  - If type is Metal, electrons is the number of electrons that one atom of this element can give.
--  - If type is Nonmetal, electrons is the number of electrons that one atom of this element needs.


 

-- Two elements can form a bond if one of them is &#39;Metal&#39; and the other is &#39;Nonmetal&#39;.

-- Write an SQL query to find all the pairs of elements that can form a bond.

-- Return the result table in any order.

-- The query result format is in the following example.

 
--  ### Example 1:


-- Input: 
-- Elements table:
-- +--------+----------+-----------+
-- | symbol | type     | electrons |
-- +--------+----------+-----------+
-- | He     | Noble    | 0         |
-- | Na     | Metal    | 1         |
-- | Ca     | Metal    | 2         |
-- | La     | Metal    | 3         |
-- | Cl     | Nonmetal | 1         |
-- | O      | Nonmetal | 2         |
-- | N      | Nonmetal | 3         |
-- +--------+----------+-----------+
-- Output: 
-- +-------+----------+
-- | metal | nonmetal |
-- +-------+----------+
-- | La    | Cl       |
-- | Ca    | Cl       |
-- | Na    | Cl       |
-- | La    | O        |
-- | Ca    | O        |
-- | Na    | O        |
-- | La    | N        |
-- | Ca    | N        |
-- | Na    | N        |
-- +-------+----------+
-- Explanation: 
-- Metal elements are La, Ca, and Na.
-- Nonmeal elements are Cl, O, and N.
-- Each Metal element pairs with a Nonmetal element in the output table.


## Solutions

<!-- tabs:start -->

### **SQL**

```sql
# Write your MySQL query statement below
SELECT a.symbol AS metal, b.symbol AS nonmetal
FROM
    Elements AS a,
    Elements AS b
WHERE a.type = 'Metal' AND b.type = 'Nonmetal';
```

<!-- tabs:end -->
