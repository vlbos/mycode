# 10. Selecting Rows Where Score is Between 15 and 20

# Write a Pandas program to select the rows the score is between 15 and 20 (inclusive).

# Sample DataFrame:
# Sample Python dictionary data and list labels:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

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
# print("Rows where score between 15 and 20 (inclusive):")
# print(df[df['score'].between(15, 20)])
# Sample Output:

# Rows where score between 15 and 20 (inclusive):                        
#    attempts       name qualify  score                                  
# c         2  Katherine     yes   16.5                                  
# f         3    Michael     yes   20.0                                  
# j         1      Jonas     yes   19.0                                
# Explanation:

# The above Pandas code first creates a dataframe ‘df’ from the dictionary ‘exam_data’ using the list labels as index. It then filters the rows of ‘df’ that have a 'score' value between 15 and 20 (inclusive) using the between() method of a Pandas series.

# Finally print() function prints the resulting filtered dataframe.

# 11. Selecting Rows with Attempts < 2 and Score > 15

# Write a Pandas program to select the rows where number of attempts in the examination is less than 2 and score greater than 15.

# Sample DataFrame:
# Sample Python dictionary data and list labels:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

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
# print("Number of attempts in the examination is less than 2 and score greater than 15 :")
# print(df[(df['attempts'] < 2) & (df['score'] > 15)])
# Sample Output:

# Number of attempts in the examination is less than 2 and score greater than 15 :
#     name  score  attempts qualify
# j  Jonas   19.0         1     yes                              
# Explanation:

# The above code first creates a Pandas DataFrame ‘df’ using the dictionary ‘exam_data’ and a list labels. It then selects the rows where the number of attempts is less than 2 and the score is greater than 15 using the & operator for and condition. Finally, it prints the selected rows of the DataFrame.

# 12. Changing the Score in a Specific Row

# Write a Pandas program to change the score in row 'd' to 11.5.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# Sample Solution :-

# Python Code :

# import pandas as pd
# import numpy as np
# exam_data  = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
#         'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
#         'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
#         'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']

# df = pd.DataFrame(exam_data , index=labels)
# print("\nOriginal data frame:")
# print(df)
# print("\nChange the score in row 'd' to 11.5:")
# df.loc['d', 'score'] = 11.5
# print(df)
# Sample Output:

# Original data frame:                                                   
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
                                                                       
# Change the score in row 'd' to 11.5:                                   
#    attempts       name qualify  score                                  
# a         1  Anastasia     yes   12.5                                  
# b         3       Dima      no    9.0                                  
# c         2  Katherine     yes   16.5
# d         3      James      no   11.5                                  
# e         2      Emily      no    9.0                                  
# f         3    Michael     yes   20.0                                  
# g         1    Matthew     yes   14.5                                  
# h         1      Laura      no    NaN                                  
# i         2      Kevin      no    8.0                                  
# j         1      Jonas     yes   19.0                            
# Explanation:

# The above code first creates a pandas DataFrame 'df' from a dictionary exam_data using a list labels as the index. It then modifies the value in the 'score' column for row 'd' from NaN to 11.5 using the .loc indexer. Finally, it prints the modified DataFrame.

# 13. Summing Examination Attempts

# Write a Pandas program to calculate the sum of the examination attempts by the students.

# Sample DataFrame:
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
# print("\nSum of the examination attempts by the students:")
# print(df['attempts'].sum())
# Sample Output:

# Sum of the examination attempts by the students:                       
# 19                         
# Explanation:

# The above code first creates a Pandas DataFrame ‘df’ from a Python dictionary ‘exam_data’, where the keys of the dictionary represent the column names and the values of the dictionary are lists representing the data in each column. The DataFrame is indexed with the values in the list labels.

# print(df['attempts'].sum()): This line prints the sum of the values in the 'attempts' column using the sum() method of the Pandas Series object representing the 'attempts' column. This gives the total number of attempts made by all students.



# 14. Calculating the Mean of Scores

# Write a Pandas program to calculate the mean of all students' scores. Data is stored in a dataframe.

# Sample DataFrame:
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
# print("\nMean score for each different student in data frame:")
# print(df['score'].mean())
# Sample Output:

# Mean score for each different student in data frame:                   
# 13.5625                         
# Explanation:

# The above code first creates a Pandas DataFrame 'df' from a dictionary 'exam_data' with row labels 'labels'. It then computes and prints the mean value of the 'score' column of the DataFrame using the mean() method of Pandas.

# The 'score' column contains numerical values with one missing value represented by numpy.nan. The mean() method automatically excludes the missing value from the computation.

# 15. Appending and Deleting a New Row

# Write a Pandas program to append a new row 'k' to DataFrame with given values for each column. Now delete the new row and return the original data frame.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
# Values for each column will be:
# name : ‘Suresh’, score: 15.5, attempts: 1, qualify: ‘yes’, label: ‘k’

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
# print("\nAppend a new row:")
# df.loc['k'] = [1, 'Suresh', 'yes', 15.5]
# print("Print all records after insert a new record:")
# print(df)
# print("\nDelete the new row and display the original  rows:")
# df = df.drop('k')
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

# Append a new row:
# Print all records after insert a new record:
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
# k         1     Suresh     yes   15.5

# Delete the new row and display the original  rows:
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
# Explanation:

# The above code first creates a Pandas DataFrame 'df' using the dictionary 'exam_data' and index labels 'labels'.

# df.loc['k'] = [1, 'Suresh', 'yes', 15.5]: This line adds a new row to the DataFrame with index label 'k' and values [1, 'Suresh', 'yes', 15.5].

# df = df.drop('k'): This line drops the newly added row using the drop method of DataFrame and assigns the resulting DataFrame back to the same variable ‘df’.

# 16. Sorting the DataFrame by Multiple Columns

# Write a Pandas program to sort the data frame first by 'name' in descending order, then by 'score' in ascending order.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
# Values for each column will be:
# name : 'Suresh', score: 15.5, attempts: 1, qualify: ‘yes’, label: ‘k’

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
# print("Orginal rows:")
# print(df)
# df = df.sort_values(by=['name', 'score'], ascending=[False, True])
# print("Sort the data frame first by ‘name’ in descending order, then by ‘score’ in ascending order:")
# print(df)
# Sample Output:

# Orginal rows:
#         name  score  attempts qualify
# a  Anastasia   12.5         1     yes
# b       Dima    9.0         3      no
# c  Katherine   16.5         2     yes
# d      James    NaN         3      no
# e      Emily    9.0         2      no
# f    Michael   20.0         3     yes
# g    Matthew   14.5         1     yes
# h      Laura    NaN         1      no
# i      Kevin    8.0         2      no
# j      Jonas   19.0         1     yes
# Sort the data frame first by ‘name’ in descending order, then by ‘score’ in ascending order:
#         name  score  attempts qualify
# f    Michael   20.0         3     yes
# g    Matthew   14.5         1     yes
# h      Laura    NaN         1      no
# i      Kevin    8.0         2      no
# c  Katherine   16.5         2     yes
# j      Jonas   19.0         1     yes
# d      James    NaN         3      no
# e      Emily    9.0         2      no
# b       Dima    9.0         3      no
# a  Anastasia   12.5         1     yes   
# Explanation:

# The above code first creates a Pandas DataFrame ‘df’ from the dictionary ‘exam_data’ using the labels labels as the row index.

# df.sort_values(by=['name', 'score'], ascending=[False, True]): This line sorts the DataFrame by the 'name' column in descending order and within each name, it sorts the 'score' column in ascending order.

# 17. Replacing Column Values (qualify)

# Write a Pandas program to replace the ‘qualify' column contains the values 'yes' and 'no' with True and False.

# Sample DataFrame:
# exam_data = {'name': ['Anastasia', 'Dima', 'Katherine', 'James', 'Emily', 'Michael', 'Matthew', 'Laura', 'Kevin', 'Jonas'],
# 'score': [12.5, 9, 16.5, np.nan, 9, 20, 14.5, np.nan, 8, 19],
# 'attempts': [1, 3, 2, 3, 2, 3, 1, 1, 2, 1],
# 'qualify': ['yes', 'no', 'yes', 'no', 'no', 'yes', 'yes', 'no', 'no', 'yes']}
# labels = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']
# Values for each column will be:
# name : ‘Suresh’, score: 15.5, attempts: 1, qualify: ‘yes’, label: ‘k’

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
# print("\nReplace the 'qualify' column contains the values 'yes' and 'no'  with True and  False:")
# df['qualify'] = df['qualify'].map({'yes': True, 'no': False})
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
                                                                       
# Replace the 'qualify' column contains the values 'yes' and 'no'  with T
# rue and  False:                                                        
#    attempts       name  qualify  score                                 
# a         1  Anastasia     True   12.5                                 
# b         3       Dima    False    9.0                                 
# c         2  Katherine     True   16.5 
# d         3      James    False    NaN                                 
# e         2      Emily    False    9.0                                 
# f         3    Michael     True   20.0                                 
# g         1    Matthew     True   14.5                                 
# h         1      Laura    False    NaN                                 
# i         2      Kevin    False    8.0                                 
# j         1      Jonas     True   19.0                      
# Explanation:

# The above code first creates a pandas DataFrame from the ‘exam_data’ dictionary and assigns index labels to each row.

# df['qualify'] = df['qualify'].map({'yes': True, 'no': False}): This line maps the values of the 'qualify' column to True and False based on whether the value is 'yes' or 'no', respectively.

# Finally print(df) function prints the resulting DataFrame.

# 18. Changing a Specific Name Value

# Write a Pandas program to change the name 'James' to 'Suresh' in name column of the data frame.

# Sample DataFrame:
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
# print("\nChange the name 'James' to ‘Suresh’:")
# df['name'] = df['name'].replace('James', 'Suresh')
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
                                                                       
# Change the name 'James' to \‘Suresh\’:                                 
#    attempts       name qualify  score                                  
# a         1  Anastasia     yes   12.5                                  
# b         3       Dima      no    9.0                                  
# c         2  Katherine     yes   16.5                                  
# d         3     Suresh      no    NaN
# e         2      Emily      no    9.0                                  
# f         3    Michael     yes   20.0                                  
# g         1    Matthew     yes   14.5                                  
# h         1      Laura      no    NaN                                  
# i         2      Kevin      no    8.0                                  
# j         1      Jonas     yes   19.0                     
# Explanation:

# The above code first creates a Pandas DataFrame called ‘df’ with the given ‘exam data’, assigns the provided list ‘labels’ as the index.

# df['name'] = df['name'].replace('James', 'Suresh'): This line replaces the name 'James' with 'Suresh' in the name column using the replace() method.

# Finally print() function prints the resulting DataFrame.

# 19. Deleting a Column from the DataFrame

# Write a Pandas program to delete the 'attempts' column from the DataFrame.

# Sample DataFrame:
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
# print("\nDelete the 'attempts' column from the data frame:")
# df.pop('attempts')
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
                                                                       
# Delete the 'attempts' column from the data frame:                      
#         name qualify  score                                            
# a  Anastasia     yes   12.5                                            
# b       Dima      no    9.0                                            
# c  Katherine     yes   16.5                                            
# d      James      no    NaN 
# e      Emily      no    9.0                                            
# f    Michael     yes   20.0                                            
# g    Matthew     yes   14.5                                            
# h      Laura      no    NaN                                            
# i      Kevin      no    8.0                                            
# j      Jonas     yes   19.0                    
# Explanation:

# The above code first creates a Pandas DataFrame named ‘df’ using the provided ‘exam_data’ dictionary and ‘labels’ list as the index.

# df.pop('attempts'): This line removes the 'attempts' column from the DataFrame using the pop() method, which returns the removed column as a Pandas Series.

# Finally print() function prints the updated DataFrame.

