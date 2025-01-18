# Medium

# Topics

# Companies

# SQL Schema

# ***

# Pandas Schema

# ***

# Table: `Customers`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | customer\_id   | int     |
# | customer\_name | varchar |
# | email         | varchar |
# +---------------+---------+
# customer\_id is the column of unique values for this table.
# Each row of this table contains the name and the email of a customer of an online shop.

# Table: `Contacts`

# +---------------+---------+
# | Column Name   | Type    |
# +---------------+---------+
# | user\_id       | id      |
# | contact\_name  | varchar |
# | contact\_email | varchar |
# +---------------+---------+
# (user\_id, contact\_email) is the primary key (combination of columns with unique values) for this table.
# Each row of this table contains the name and email of one contact of customer with user\_id.
# This table contains information about people each customer trust. The contact may or may not exist in the Customers table.

# Table: `Invoices`

# +--------------+---------+
# | Column Name  | Type    |
# +--------------+---------+
# | invoice\_id   | int     |
# | price        | int     |
# | user\_id      | int     |
# +--------------+---------+
# invoice\_id is the column of unique values for this table.
# Each row of this table indicates that user\_id has an invoice with invoice\_id and a price.

# Write a solution to find the following for each `invoice_id`:

# *   `customer_name`: The name of the customer the invoice is related to.
# *   `price`: The price of the invoice.
# *   `contacts_cnt`: The number of contacts related to the customer.
# *   `trusted_contacts_cnt`: The number of contacts related to the customer and at the same time they are customers to the shop. (i.e their email exists in the `Customers` table.)

# Return the result table **ordered** by `invoice_id`.

# The result format is in the following example.

# **Example 1:**

# **Input:** 
# Customers table:
# +-------------+---------------+--------------------+
# | customer\_id | customer\_name | email              |
# +-------------+---------------+--------------------+
# | 1           | Alice         | alice@leetcode.com |
# | 2           | Bob           | bob@leetcode.com   |
# | 13          | John          | john@leetcode.com  |
# | 6           | Alex          | alex@leetcode.com  |
# +-------------+---------------+--------------------+
# Contacts table:
# +-------------+--------------+--------------------+
# | user\_id     | contact\_name | contact\_email      |
# +-------------+--------------+--------------------+
# | 1           | Bob          | bob@leetcode.com   |
# | 1           | John         | john@leetcode.com  |
# | 1           | Jal          | jal@leetcode.com   |
# | 2           | Omar         | omar@leetcode.com  |
# | 2           | Meir         | meir@leetcode.com  |
# | 6           | Alice        | alice@leetcode.com |
# +-------------+--------------+--------------------+
# Invoices table:
# +------------+-------+---------+
# | invoice\_id | price | user\_id |
# +------------+-------+---------+
# | 77         | 100   | 1       |
# | 88         | 200   | 1       |
# | 99         | 300   | 2       |
# | 66         | 400   | 2       |
# | 55         | 500   | 13      |
# | 44         | 60    | 6       |
# +------------+-------+---------+
# **Output:** 
# +------------+---------------+-------+--------------+----------------------+
# | invoice\_id | customer\_name | price | contacts\_cnt | trusted\_contacts\_cnt |
# +------------+---------------+-------+--------------+----------------------+
# | 44         | Alex          | 60    | 1            | 1                    |
# | 55         | John          | 500   | 0            | 0                    |
# | 66         | Bob           | 400   | 2            | 0                    |
# | 77         | Alice         | 100   | 3            | 2                    |
# | 88         | Alice         | 200   | 3            | 2                    |
# | 99         | Bob           | 300   | 2            | 0                    |
# +------------+---------------+-------+--------------+----------------------+
# **Explanation:** 
# Alice has three contacts, two of them are trusted contacts (Bob and John).
# Bob has two contacts, none of them is a trusted contact.
# Alex has one contact and it is a trusted contact (Alice).
# John doesn't have any contacts.


import pandas as pd

def count_trusted_contacts(customers: pd.DataFrame, contacts: pd.DataFrame, invoices: pd.DataFrame) -> pd.DataFrame:
    contact_customer = pd.merge(contacts,customers,left_on="contact_email",right_on="email",how="left")
    contact_customer = contact_customer.groupby('user_id').agg(contacts_cnt=('user_id','count'), trusted_contacts_cnt =('email','count') ).reset_index()
    invoice_customer = invoices.merge(customers,left_on="user_id",right_on="customer_id",how="left")
    invoice_customer = invoice_customer.merge(contact_customer,on="user_id",how="left").reset_index()

    return invoice_customer[['invoice_id','customer_name','price','contacts_cnt','trusted_contacts_cnt']].sort_values('invoice_id').fillna(0)