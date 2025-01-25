225\. 矩阵幂求和

*    [题目](https://www.acwing.com/problem/content/description/227/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/227/1/)
*    [题解](https://www.acwing.com/problem/content/solution/227/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/227/)

  

给定 n×nn×n 矩阵 AA 和正整数 kk，求和 S\=A+A2+A3+…+AkS\=A+A2+A3+…+Ak。

#### 输入格式

输入只包含一个测试用例。

第一行输入包含三个正整数 n，kn，k 和 mm。

接下来 nn 行，每行包含 nn 个非负整数（均不超过 32,76832,768），用以描绘矩阵 AA。

##### 输出格式

按与描述矩阵 AA 相同的方式，输出将 SS 中所有元素对 mm 取模后得到的矩阵。

#### 数据范围

1≤n≤301≤n≤30,  
1≤k≤1091≤k≤109,  
1≤m<1041≤m<104

#### 输入样例:

    2 2 4
    0 1
    1 1
    

#### 输出样例：

    1 2
    2 3
    

难度：简单

时/空限制：1s / 64MB

总通过数：802

总尝试数：1543

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3776&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3776&show_algorithm_tags=1)[矩阵乘法](https://www.acwing.com/problem/search/1/?search_content=%E7%9F%A9%E9%98%B5%E4%B9%98%E6%B3%95&source_file_id=3776&show_algorithm_tags=1)[分治](https://www.acwing.com/problem/search/1/?search_content=%E5%88%86%E6%B2%BB&source_file_id=3776&show_algorithm_tags=1)