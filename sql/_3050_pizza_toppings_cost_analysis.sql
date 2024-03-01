# [3050. Pizza Toppings Cost Analysis](https://leetcode.com/problems/pizza-toppings-cost-analysis)

[中文文档](/solution/3000-3099/3050.Pizza%20Toppings%20Cost%20Analysis/README.md)



## Description

Table: Toppings


+--------------+---------+ 
| Column Name  | Type    | 
+--------------+---------+ 
| topping_name | varchar | 
| cost         | decimal |
+--------------+---------+
topping_name is the primary key for this table.
Each row of this table contains topping name and the cost of the topping. 


Write a solution to calculate the total cost of all possible 3-topping pizza combinations from a given list of toppings. The total cost of toppings must be rounded to 2 decimal places.

Note:


	Do not include the pizzas where a topping is repeated. For example, &lsquo;Pepperoni, Pepperoni, Onion Pizza&rsquo;.
	Toppings must be listed in alphabetical order. For example,  'Chicken, Onions, Sausage '.  'Onion, Sausage, Chicken ' is not acceptable.


Return the result table ordered by total cost in descending order and combination of toppings in ascending order.

The result format is in the following example.

 
Example 1:


Input: 
Toppings table:
+--------------+------+
| topping_name | cost |
+--------------+------+
| Pepperoni    | 0.50 |
| Sausage      | 0.70 |
| Chicken      | 0.55 |
| Extra Cheese | 0.40 |
+--------------+------+
Output: 
+--------------------------------+------------+
| pizza                          | total_cost | 
+--------------------------------+------------+
| Chicken,Pepperoni,Sausage      | 1.75       |  
| Chicken,Extra Cheese,Sausage   | 1.65       |
| Extra Cheese,Pepperoni,Sausage | 1.60       |
| Chicken,Extra Cheese,Pepperoni | 1.45       | 
+--------------------------------+------------+
Explanation: 
There are only four different combinations possible with the three topings:
- Chicken, Pepperoni, Sausage: Total cost is $1.75 (Chicken $0.55, Pepperoni $0.50, Sausage $0.70).
- Chicken, Extra Cheese, Sausage: Total cost is $1.65 (Chicken $0.55, Extra Cheese $0.40, Sausage $0.70).
- Extra Cheese, Pepperoni, Sausage: Total cost is $1.60 (Extra Cheese $0.40, Pepperoni $0.50, Sausage $0.70).
- Chicken, Extra Cheese, Pepperoni: Total cost is $1.45 (Chicken $0.55, Extra Cheese $0.40, Pepperoni $0.50).
Output table is ordered by the total cost in descending order.

## Solutions

### Solution 1: Window Function + Conditional Join

First, we use a window function to sort the table by the `topping_name` field and add a `rk` field to each row, representing the ranking of the current row.

Then we use conditional join to join the table `T` three times, named as `t1`, `t2`, `t3` respectively. The join conditions are `t1.rk < t2.rk` and `t2.rk < t3.rk`. After that, we calculate the total price of the three toppings, sort by total price in descending order, and then sort by topping name in ascending order.

<!-- tabs:start -->

```sql
# Write your MySQL query statement below
WITH
    T AS (
        SELECT *, RANK() OVER (ORDER BY topping_name) AS rk
        FROM Toppings
    )
SELECT
    CONCAT(t1.topping_name, ',', t2.topping_name, ',', t3.topping_name) AS pizza,
    t1.cost + t2.cost + t3.cost AS total_cost
FROM
    T AS t1
    JOIN T AS t2 ON t1.rk < t2.rk
    JOIN T AS t3 ON t2.rk < t3.rk
ORDER BY 2 DESC, 1 ASC;
```

<!-- tabs:end -->

<!-- end -->
