214\. Devu和鲜花

*    [题目](https://www.acwing.com/problem/content/description/216/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/216/1/)
*    [题解](https://www.acwing.com/problem/content/solution/216/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/216/)

  

Devu 有 NN 个盒子，第 ii 个盒子中有 AiAi 枝花。

同一个盒子内的花颜色相同，不同盒子内的花颜色不同。

Devu 要从这些盒子中选出 MM 枝花组成一束，求共有多少种方案。

若两束花每种颜色的花的数量都相同，则认为这两束花是相同的方案。

结果需对 109+7109+7 取模之后方可输出。

#### 输入格式

第一行包含两个整数 NN 和 MM。

第二行包含 NN 个空格隔开的整数，表示 A1,A2,…,ANA1,A2,…,AN。

#### 输出格式

输出一个整数，表示方案数量对 109+7109+7 取模后的结果。

#### 数据范围

1≤N≤201≤N≤20,  
0≤M≤10140≤M≤1014,  
0≤Ai≤10120≤Ai≤1012

#### 输入样例：

    3 5
    1 3 2
    

#### 输出样例：

    3
    

难度：中等

时/空限制：1s / 64MB

总通过数：3960

总尝试数：9011

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3765&show_algorithm_tags=0)[CF451E](https://www.acwing.com/problem/search/1/?search_content=CF451E&source_file_id=3765&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3765&show_algorithm_tags=1)[容斥原理](https://www.acwing.com/problem/search/1/?search_content=%E5%AE%B9%E6%96%A5%E5%8E%9F%E7%90%86&source_file_id=3765&show_algorithm_tags=1)