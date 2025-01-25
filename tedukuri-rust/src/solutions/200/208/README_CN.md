208\. 开关问题

*    [题目](https://www.acwing.com/problem/content/description/210/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/210/1/)
*    [题解](https://www.acwing.com/problem/content/solution/210/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/210/)

  

有 NN 个相同的开关，每个开关都与某些开关有着联系，每当你打开或者关闭某个开关的时候，其他的与此开关相关联的开关也会相应地发生变化，即这些相联系的开关的状态如果原来为开就变为关，如果为关就变为开。

你的目标是经过若干次开关操作后使得最后 NN 个开关达到一个特定的状态。

对于任意一个开关，最多只能进行一次开关操作。

你的任务是，计算有多少种可以达到指定状态的方法。（不计开关操作的顺序）

#### 输入格式

输入第一行有一个数 KK，表示以下有 KK 组测试数据。

每组测试数据的格式如下：

第一行：一个数 NN。

第二行：NN 个 00 或者 11 的数，表示开始时 NN 个开关状态。

第三行：NN 个 00 或者 11 的数，表示操作结束后 NN 个开关的状态。

接下来每行两个数 I,JI,J，表示如果操作第 II 个开关，第 JJ 个开关的状态也会变化。

每组数据以 `0 0` 结束。

#### 输出格式

每组数据输出占一行。

如果有可行方法，输出总数，否则输出 `Oh,it's impossible~!!` 。

#### 数据范围

1≤K≤101≤K≤10,  
0<N<290<N<29

#### 输入样例：

    2
    3
    0 0 0
    1 1 1
    1 2
    1 3
    2 1
    2 3
    3 1
    3 2
    0 0
    3
    0 0 0
    1 0 1
    1 2
    2 1
    0 0
    

#### 输出样例：

    4
    Oh,it's impossible~!!
    

难度：中等

时/空限制：1s / 64MB

总通过数：3105

总尝试数：5004

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3759&show_algorithm_tags=0)

算法标签

[数学知识](https://www.acwing.com/problem/search/1/?search_content=%E6%95%B0%E5%AD%A6%E7%9F%A5%E8%AF%86&source_file_id=3759&show_algorithm_tags=1)[高斯消元](https://www.acwing.com/problem/search/1/?search_content=%E9%AB%98%E6%96%AF%E6%B6%88%E5%85%83&source_file_id=3759&show_algorithm_tags=1)