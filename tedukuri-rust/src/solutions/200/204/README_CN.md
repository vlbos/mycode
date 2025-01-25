204\. 表达整数的奇怪方式

*    [题目](https://www.acwing.com/problem/content/description/206/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/206/1/)
*    [题解](https://www.acwing.com/problem/content/solution/206/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/206/)

  

给定 2n2n 个整数 a1,a2,…,ana1,a2,…,an 和 m1,m2,…,mnm1,m2,…,mn，求一个最小的非负整数 xx，满足 ∀i∈\[1,n\],x≡mi(mod ai)∀i∈\[1,n\],x≡mi(mod ai)。

#### 输入格式

第 11 行包含整数 nn。

第 2…n+12…n+1 行：每 i+1i+1 行包含两个整数 aiai 和 mimi，数之间用空格隔开。

#### 输出格式

输出最小非负整数 xx，如果 xx 不存在，则输出 −1−1。

#### 数据范围

1≤ai≤231−11≤ai≤231−1,  
0≤mi<ai0≤mi<ai  
1≤n≤251≤n≤25  
所有 mimi 的最小公倍数在 6464 位有符号整数范围内。

#### 输入样例：

    2
    8 7
    11 9
    

#### 输出样例：

    31
    

难度：中等

时/空限制：1s / 64MB

总通过数：20063

总尝试数：36497

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3755&show_algorithm_tags=0)[模板题](https://www.acwing.com/problem/search/1/?search_content=%E6%A8%A1%E6%9D%BF%E9%A2%98&source_file_id=3755&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3755&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3755&show_algorithm_tags=1)[同余方程](https://www.acwing.com/problem/search/1/?search_content=%E5%90%8C%E4%BD%99%E6%96%B9%E7%A8%8B&source_file_id=3755&show_algorithm_tags=1)[扩展中国剩余定理](https://www.acwing.com/problem/search/1/?search_content=%E6%89%A9%E5%B1%95%E4%B8%AD%E5%9B%BD%E5%89%A9%E4%BD%99%E5%AE%9A%E7%90%86&source_file_id=3755&show_algorithm_tags=1)