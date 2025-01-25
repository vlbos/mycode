205\. 斐波那契

*    [题目](https://www.acwing.com/problem/content/description/207/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/207/1/)
*    [题解](https://www.acwing.com/problem/content/solution/207/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/207/)

  

在斐波那契数列中，Fib0\=0,Fib1\=1,Fibn\=Fibn−1+Fibn−2(n\>1)Fib0\=0,Fib1\=1,Fibn\=Fibn−1+Fibn−2(n\>1)。

给定整数 nn，求 Fibnmod10000Fibnmod10000。

#### 输入格式

输入包含不超过 100100 组测试用例。

每个测试用例占一行，包含一个整数 nn。

当输入用例 n\=−1n\=−1 时，表示输入终止，且该用例无需处理。

#### 输出格式

每个测试用例输出一个整数表示结果。

每个结果占一行。

#### 数据范围

0≤n≤2×1090≤n≤2×109

#### 输入样例：

    0
    9
    999999999
    1000000000
    -1
    

#### 输出样例：

    0
    34
    626
    6875
    

难度：中等

时/空限制：1s / 64MB

总通过数：3670

总尝试数：5371

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3756&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3756&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3756&show_algorithm_tags=1)[矩阵乘法](https://www.acwing.com/problem/search/1/?search_content=%E7%9F%A9%E9%98%B5%E4%B9%98%E6%B3%95&source_file_id=3756&show_algorithm_tags=1)[快速幂](https://www.acwing.com/problem/search/1/?search_content=%E5%BF%AB%E9%80%9F%E5%B9%82&source_file_id=3756&show_algorithm_tags=1)