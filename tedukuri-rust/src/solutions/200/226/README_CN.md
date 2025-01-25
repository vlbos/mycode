226\. 233矩阵

*    [题目](https://www.acwing.com/problem/content/description/228/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/228/1/)
*    [题解](https://www.acwing.com/problem/content/solution/228/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/228/)

  

在我们的日常生活中，我们经常使用 233233 来表达我们的感受。

实际上，我们可能会说 2333,233332333,23333 或 233333......233333...... 意思相同。

假设我们有一个名为 233233 矩阵的矩阵。

在第一行，它将包含 233,2333,23333…233,2333,23333…（这意味着 a0,1\=233，a0,2\=2333，a0,3\=23333…a0,1\=233，a0,2\=2333，a0,3\=23333…）。

此外，在 233233 矩阵中，满足 ai,j\=ai−1,j+ai,j−1（i,j≠0）ai,j\=ai−1,j+ai,j−1（i,j≠0）。

现在给定 a1,0,a2,0,…,an,0a1,0,a2,0,…,an,0，请求出在 233233 矩阵中 an,man,m 的值。

#### 输入格式

输入包含多组数据，请处理至文件末尾。

每组数据包括两行，第一行包含两个整数 n，mn，m。

第二行包含 nn 个整数，表示 a1,0,a2,0,…,an,0a1,0,a2,0,…,an,0。

#### 输出格式

每组数据输出一个整数，表示 an,mmod10000007an,mmod10000007 的值。

每个结果占一行。

#### 数据范围

1≤n≤101≤n≤10,  
1≤m≤1091≤m≤109,  
0≤ai,0<2310≤ai,0<231

#### 输入样例：

    1 1
    1
    2 2
    0 0
    3 7
    23 47 16
    

#### 输出样例：

    234
    2799
    72937
    

#### 样例解释：

![C534-1009-1.jpg](https://cdn.acwing.com/media/article/image/2019/01/19/19_a9d848521b-C534-1009-1.jpg)

难度：简单

时/空限制：1s / 64MB

总通过数：532

总尝试数：838

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3777&show_algorithm_tags=0)[HDU5015](https://www.acwing.com/problem/search/1/?search_content=HDU5015&source_file_id=3777&show_algorithm_tags=0)[kuangbin专题](https://www.acwing.com/problem/search/1/?search_content=kuangbin%E4%B8%93%E9%A2%98&source_file_id=3777&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3777&show_algorithm_tags=1)[矩阵乘法](https://www.acwing.com/problem/search/1/?search_content=%E7%9F%A9%E9%98%B5%E4%B9%98%E6%B3%95&source_file_id=3777&show_algorithm_tags=1)[递推](https://www.acwing.com/problem/search/1/?search_content=%E9%80%92%E6%8E%A8&source_file_id=3777&show_algorithm_tags=1)