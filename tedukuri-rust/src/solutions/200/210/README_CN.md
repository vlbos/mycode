210\. 异或运算

*    [题目](https://www.acwing.com/problem/content/description/212/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/212/1/)
*    [题解](https://www.acwing.com/problem/content/solution/212/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/212/)

  

给定你由 NN 个整数构成的整数序列，你可以从中选取一些（至少一个）进行异或（xorxor）运算，从而得到很多不同的结果。

请问，所有能得到的不同的结果中第 kk 小的结果是多少。

#### 输入格式

第一行包含整数 TT，表示共有 TT 组测试数据。

对于每组测试数据，第一行包含整数 NN。

第二行包含 NN 个整数(均在 11 至 10181018 之间)，表示完整的整数序列。

第三行包含整数 QQ，表示询问的次数。

第四行包含 QQ 个整数 k1,k2,…,kQk1,k2,…,kQ，表示 QQ 个询问对应的 kk。

#### 输出格式

对于每组测试数据，第一行输出 `Case #C:`，其中 CC 为顺序编号（从 11 开始）。

接下来 QQ 行描述 QQ 次询问的结果，每行输出一个整数，表示第 ii 次询问中第 kiki 小的结果。

如果能得到的不同结果的总数少于 kiki，则输出 −1−1。

#### 数据范围

1≤N,Q≤100001≤N,Q≤10000,  
1≤ki≤10181≤ki≤1018

#### 输入样例：

    2
    2
    1 2
    4
    1 2 3 4
    3
    1 2 3
    5
    1 2 3 4 5
    

#### 输出样例：

    Case #1:
    1
    2
    3
    -1
    Case #2:
    0
    1
    2
    3
    -1
    

**注意**:只选取一个数字进行运算，则结果为该数字本身。

难度：中等

时/空限制：1s / 32MB

总通过数：1767

总尝试数：4751

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3761&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3761&show_algorithm_tags=1)[线性空间](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7%E7%A9%BA%E9%97%B4&source_file_id=3761&show_algorithm_tags=1)[线性基](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7%E5%9F%BA&source_file_id=3761&show_algorithm_tags=1)