187\. 导弹防御系统

*    [题目](https://www.acwing.com/problem/content/description/189/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/189/1/)
*    [题解](https://www.acwing.com/problem/content/solution/189/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/189/)

  

为了对抗附近恶意国家的威胁，RR 国更新了他们的导弹防御系统。

一套防御系统的导弹拦截高度要么一直 **严格单调** 上升要么一直 **严格单调** 下降。

例如，一套系统先后拦截了高度为 33 和高度为 44 的两发导弹，那么接下来该系统就只能拦截高度大于 44 的导弹。

给定即将袭来的一系列导弹的高度，请你求出至少需要多少套防御系统，就可以将它们全部击落。

#### 输入格式

输入包含多组测试用例。

对于每个测试用例，第一行包含整数 nn，表示来袭导弹数量。

第二行包含 nn 个**不同的**整数，表示每个导弹的高度。

当输入测试用例 n\=0n\=0 时，表示输入终止，且该用例无需处理。

#### 输出格式

对于每个测试用例，输出一个占据一行的整数，表示所需的防御系统数量。

#### 数据范围

1≤n≤501≤n≤50

#### 输入样例：

    5
    3 5 2 4 1
    0 
    

#### 输出样例：

    2
    

#### 样例解释

对于给出样例，最少需要两套防御系统。

一套击落高度为 3,43,4 的导弹，另一套击落高度为 5,2,15,2,1 的导弹。

难度：中等

时/空限制：3s / 64MB

总通过数：22111

总尝试数：42132

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3738&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3738&show_algorithm_tags=1)[深度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%B7%B1%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3738&show_algorithm_tags=1)[DFS](https://www.acwing.com/problem/search/1/?search_content=DFS&source_file_id=3738&show_algorithm_tags=1)[迭代加深](https://www.acwing.com/problem/search/1/?search_content=%E8%BF%AD%E4%BB%A3%E5%8A%A0%E6%B7%B1&source_file_id=3738&show_algorithm_tags=1)[贪心](https://www.acwing.com/problem/search/1/?search_content=%E8%B4%AA%E5%BF%83&source_file_id=3738&show_algorithm_tags=1)