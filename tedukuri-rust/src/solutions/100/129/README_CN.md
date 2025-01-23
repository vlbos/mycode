129\. 火车进栈

*    [题目](https://www.acwing.com/problem/content/description/131/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/131/1/)
*    [题解](https://www.acwing.com/problem/content/solution/131/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/131/)

  

这里有 nn 列火车将要进站再出站，但是，每列火车只有 11 节，那就是车头。

这 nn 列火车按 11 到 nn 的顺序从东方左转进站，这个车站是南北方向的，它虽然无限长，只可惜是一个死胡同，而且站台只有一条股道，火车只能倒着从西方出去，而且每列火车必须进站，先进后出。

也就是说这个火车站其实就相当于一个栈，每次可以让右侧头火车进栈，或者让栈顶火车出站。

车站示意如图：

                出站<——    <——进站
                         |车|
                         |站|
                         |__|
    

现在请你按《字典序》输出前 2020 种可能的出栈方案。

#### 输入格式

输入一个整数 nn，代表火车数量。

#### 输出格式

按照《字典序》输出前 2020 种答案，每行一种，不要空格。

#### 数据范围

1≤n≤201≤n≤20

#### 输入样例：

    3
    

#### 输出样例：

    123
    132
    213
    231
    321
    

难度：简单

时/空限制：1s / 64MB

总通过数：6058

总尝试数：10781

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3680&show_algorithm_tags=0)

算法标签

[栈](https://www.acwing.com/problem/search/1/?search_content=%E6%A0%88&source_file_id=3680&show_algorithm_tags=1)[dfs](https://www.acwing.com/problem/search/1/?search_content=dfs&source_file_id=3680&show_algorithm_tags=1)