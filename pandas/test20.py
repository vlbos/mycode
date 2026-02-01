# 20. Inserting a New Column

# Write a Pandas program to insert a new column in existing DataFrame.

# Sample DataFrame:
# Sample Python dictionary data and list labels:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
# df = pd.DataFrame(exam_data , index=labels)
# print("Original rows:")
# print(df)
# color = ['Red','Blue','Orange','Red','White','White','Blue','Green','Green','Red']
# df['color'] = color
# print("\nNew DataFrame after inserting the 'color' column")
# print(df)
# Sample Output:

# Original rows:                                                          
#    attempts       name qualify  score                                  
# a         1  Anastasia     yes   12.5                                  
# b         3       Dima      no    9.0                                  
# c         2  Katherine     yes   16.5                                  
# d         3      James      no    NaN                                  
# e         2      Emily      no    9.0                                  
# f         3    Michael     yes   20.0                                  
# g         1    Matthew     yes   14.5                                  
# h         1      Laura      no    NaN                                  
# i         2      Kevin      no    8.0                                  
# j         1      Jonas     yes   19.0                                  
                                                                       
# New DataFrame after inserting the 'color' column                       
#    attempts       name qualify  score   color                          
# a         1  Anastasia     yes   12.5     Red                          
# b         3       Dima      no    9.0    Blue                          
# c         2  Katherine     yes   16.5  Orange                          
# d         3      James      no    NaN     Red
# e         2      Emily      no    9.0   White                          
# f         3    Michael     yes   20.0   White                          
# g         1    Matthew     yes   14.5    Blue                          
# h         1      Laura      no    NaN   Green                          
# i         2      Kevin      no    8.0   Green                          
# j         1      Jonas     yes   19.0     Red                    
# Explanation:

# The above code creates a Pandas DataFrame called ‘df’ with columns 'name', 'score', 'attempts', and 'qualify' using the provided ‘exam_data’ dictionary and ‘labels’ list as the index.

# df['color'] = color: This line creates a new column called 'color' using the ‘color’ list. The 'color' column is appended to the end of the DataFrame and contains the colors assigned to each name. The colors are assigned based on the order of the names in the ‘exam_data’ dictionary.

# Finally print() function prints the updated DataFrame.

# 21. Iterating Over DataFrame Rows

# Write a Pandas program to iterate over rows in a DataFrame.

# Sample Python dictionary data and list labels:
# exam_data = [{'name':'Anastasia', 'score':12.5}, {'name':'Dima','score':9}, {'name':'Katherine','score':16.5}]

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data = [{'name':'Anastasia', 'score':12.5}, {'name':'Dima','score':9}, {'name':'Katherine','score':16.5}]
# df = pd.DataFrame(exam_data)
# for index, row in df.iterrows():
#     print(row['name'], row['score'])
# Sample Output:

# Anastasia 12.5                                                         
# Dima 9.0                                                               
# Katherine 16.5                   
# Explanation:

# The above code first creates a Pandas DataFrame called ‘df’ using a list of dictionaries containing information about exam scores for three people.

# It then iterates over the rows of the DataFrame using the iterrows() method, which returns an iterator yielding index and row data as tuples. For each row, the code prints the values of the name and score columns using print() function.


# 22. Getting List from Column Headers

# Write a Pandas program to get list from DataFrame column headers.

# Sample data:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
# df = pd.DataFrame(exam_data , index=labels)
# print(list(df.columns.values))
# Sample Output:

# ['attempts', 'name', 'qualify', 'score']                  
# Explanation:

# The above code defines a dictionary 'exam_data' with keys 'name', 'score', 'attempts', and 'qualify' and their corresponding values as lists. It then defines a list of 'labels' and creates a Pandas DataFrame 'df' from the dictionary with the index set to the list of labels.

# print(list(df.columns.values)): This line prints the list of column names in the DataFrame df.

# 23. Renaming DataFrame Columns

# Write a Pandas program to rename columns of a given DataFrame.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 2 5 8
# 2 3 6 9
# New DataFrame after renaming columns:
# Column1 Column2 Column3
# 0 1 4 7
# 1 2 5 8
# 2 3 6 9

# Sample Solution :

# Python Code :

# import pandas as pd
# d = {'col1': [1, 2, 3], 'col2': [4, 5, 6], 'col3': [7, 8, 9]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# df.columns = ['Column1', 'Column2', 'Column3']
# df = df.rename(columns={'col1': 'Column1', 'col2': 'Column2', 'col3': 'Column3'})
# print("New DataFrame after renaming columns:")
# print(df)
# Sample Output:

# Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     2     5     8
# 2     3     6     9
# New DataFrame after renaming columns:
#    Column1  Column2  Column3
# 0        1        4        7
# 1        2        5        8
# 2        3        6        9                  
# Explanation:

# The above code first creates a Pandas DataFrame 'df' from a dictionary 'd' with 3 columns 'col1', 'col2', and 'col3'.

# df = df.rename(columns={'col1': 'Column1', 'col2': 'Column2', 'col3': 'Column3'}): This line changes the column names of the DataFrame using two different methods. First, it sets the column names using the .columns attribute, and then it renames the columns using the .rename() method with a dictionary mapping the old column names to the new column names.

# Finally, it prints the DataFrame with the new column names using print() function.

# 24. Selecting Rows Based on Column Values

# Write a Pandas program to select rows from a given DataFrame based on values in some columns.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1
# Rows for colum1 value == 4
# col1 col2 col3
# 1 4 5 8
# 3 4 7 0

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# d = {'col1': [1, 4, 3, 4, 5], 'col2': [4, 5, 6, 7, 8], 'col3': [7, 8, 9, 0, 1]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print('Rows for colum1 value == 4')
# print(df.loc[df['col1'] == 4])
# Sample Output:

#       Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1
# Rows for colum1 value == 4
#    col1  col2  col3
# 1     4     5     8
# 3     4     7     0            
# Explanation:

# The above code first creates a pandas DataFrame called ‘df’ with three columns 'col1', 'col2', and 'col3' and five rows of data.

# df.loc[df['col1'] == 4]: This line selects only the rows of the DataFrame where the 'col1' column is equal to 4. This is done using a boolean mask created by the expression df['col1'] == 4. The .loc method is then used to select only the rows where the boolean mask is True.

# 25. Changing the Order of DataFrame Columns

# Write a Pandas program to change the order of a DataFrame columns.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1
# After altering col1 and col3
# col3 col2 col1
# 0 7 4 1
# 1 8 5 4
# 2 9 6 3
# 3 0 7 4
# 4 1 8 5

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# d = {'col1': [1, 4, 3, 4, 5], 'col2': [4, 5, 6, 7, 8], 'col3': [7, 8, 9, 0, 1]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print('After altering col1 and col3')
# df = df[['col3', 'col2', 'col1']]
# print(df)
# Sample Output:

#  Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1
# After altering col1 and col3
#    col3  col2  col1
# 0     7     4     1
# 1     8     5     4
# 2     9     6     3
# 3     0     7     4
# 4     1     8     5                 
# Explanation:

# The said code creates a Pandas DataFrame called ‘df’ with three columns labeled 'col1', 'col2', and 'col3' and five rows of data.

# df = df[['col3', 'col2', 'col1']]: This code reorders the columns of ‘df’ using double brackets, [['col3', 'col2', 'col1']], to select the columns in the order 'col3', 'col2', and 'col1'. The resulting DataFrame, which has the same data as the original ‘df’ but with columns in a different order, is stored back into the ‘df’ variable.

# 26. Add One Row to a DataFrame

# Write a Pandas program to add one row in an existing DataFrame.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1
# After add one row:
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1
# 5 10 11 12

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# d = {'col1': [1, 4, 3, 4, 5], 'col2': [4, 5, 6, 7, 8], 'col3': [7, 8, 9, 0, 1]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print('After add one row:')
# df2 = {'col1': 10, 'col2': 11, 'col3': 12}
# df = df.append(df2, ignore_index=True)
# print(df)
# Sample Output:

#    Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1
# After add one row:
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1
# 5    10    11    12               
# Explanation:

# The code creates a Pandas DataFrame df from a Python dictionary ‘d’, which has three keys 'col1', 'col2', and 'col3', and corresponding values that are lists of integers. Then, a new Python dictionary df2 is created with the same keys, but with a single set of values.

# df = df.append(df2, ignore_index=True): This code is used to add ‘df2’ as a new row to the DataFrame df. The ignore_index parameter is set to True to reset the index of the appended DataFrame.

# Finally print() function prints the resulting DataFrame.


# 27. Write DataFrame to CSV (Tab Separator)

# Write a Pandas program to write a DataFrame to CSV file using tab separator.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1
# Data from new_file.csv file:
# col1\tcol2\tcol3
# 0 1\t4\t7
# 1 4\t5\t8
# 2 3\t6\t9
# 3 4\t7\t0
# 4 5\t8\t1

# Sample Solution:

# Python Code :

# import pandas as pd
# import numpy as np
# d = {'col1': [1, 4, 3, 4, 5], 'col2': [4, 5, 6, 7, 8], 'col3': [7, 8, 9, 0, 1]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# print('Data from new_file.csv file:')
# df.to_csv('new_file.csv', sep='\t', index=False)
# new_df = pd.read_csv('new_file.csv')
# print(new_df)
# Sample Output:

#      Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1
# Data from new_file.csv file:
#   col1\tcol2\tcol3
# 0          1\t4\t7
# 1          4\t5\t8
# 2          3\t6\t9
# 3          4\t7\t0
# 4          5\t8\t1             
# Explanation:

# The above code first creates a Pandas DataFrame ‘df’ from a dictionary ‘d’ containing three columns: col1, col2, and col3.

# df.to_csv('new_file.csv', sep='\t', index=False): This code saves this DataFrame as a tab-separated file named new_file.csv using the to_csv() method with the parameters sep='\t' and index=False.

# new_df = pd.read_csv('new_file.csv'): This code loads the saved file into a new DataFrame new_df using the read_csv() function.

# Finally print() function prints the content of 'new_df' to the console.

# 28. City Wise Count

# Write a Pandas program to count city wise number of people from a given of data set (city, name of the person).
# Sample data:
# city Number of people
# 0 California 4
# 1 Georgia 2
# 2 Los Angeles 4

# Sample Solution :

# Python Code :

# import pandas as pd
# df1 = pd.DataFrame({'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'city': ['California', 'Los Angeles', 'California', 'California', 'California', 'Los Angeles', 'Los Angeles', 'Georgia', 'Georgia', 'Los Angeles']})
# g1 = df1.groupby(["city"]).size().reset_index(name='Number of people')
# print(g1)
# Sample Output:

#           city  Number of people
# 0   California                 4
# 1      Georgia                 2
# 2  Los Angeles                 4                  
# Explanation:

# In the above code -

# Creates a Pandas DataFrame called df1 with two columns, "name" and "city", and 10 rows of data.
# Groups the rows of df1 by the "city" column using the groupby() method.
# Applies the size() method to each group to count the number of rows in each group.
# Resets the index of the resulting DataFrame using the reset_index() method and renames the column with the count as "Number of people".
# Stores the resulting DataFrame in a variable called g1.
# The resulting DataFrame has two columns: "city" and "Number of people".
# Prints the contents of g1 to the console.

# 29. Delete Rows by Column Value

# Write a Pandas program to delete DataFrame row(s) based on given column value.

# Sample data:
# Original DataFrame
# col1 col2 col3
# 0 1 4 7
# 1 4 5 8
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1
# New DataFrame
# col1 col2 col3
# 0 1 4 7
# 2 3 6 9
# 3 4 7 0
# 4 5 8 1

# Sample Solution :

# Python Code :

# import pandas as pd
# import numpy as np
# d = {'col1': [1, 4, 3, 4, 5], 'col2': [4, 5, 6, 7, 8], 'col3': [7, 8, 9, 0, 1]}
# df = pd.DataFrame(data=d)
# print("Original DataFrame")
# print(df)
# df = df[df.col2 != 5]
# print("New DataFrame")
# print(df)
# Sample Output:

#     Original DataFrame
#    col1  col2  col3
# 0     1     4     7
# 1     4     5     8
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1
# New DataFrame
#    col1  col2  col3
# 0     1     4     7
# 2     3     6     9
# 3     4     7     0
# 4     5     8     1              
# Explanation:

# The above code first creates a Pandas DataFrame df with columns col1, col2, and col3 using a dictionary ‘d’.

# df = df[df.col2 != 5]: This line filters the rows of the DataFrame where the value in col2 is not equal to 5 using boolean indexing. Specifically, it creates a boolean mask df.col2 != 5, which returns True for all rows where the value in col2 is not 5, and False otherwise. This mask is then passed to the DataFrame to select only the rows where the mask is True, effectively filtering out the row where col2 is equal to 5.

# Finally print() function prints the resulting DataFrame.



