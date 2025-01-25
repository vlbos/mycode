391\. 聚会

*    [题目](https://www.acwing.com/problem/content/description/393/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/393/1/)
*    [题解](https://www.acwing.com/problem/content/solution/393/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/393/)

  

YY 岛风景美丽宜人，气候温和，物产丰富。

YY 岛上有 NN 个城市（编号 1,2,…,N1,2,…,N），有 N−1N−1 条城市间的道路连接着它们。

每一条道路都连接某两个城市。

幸运的是，小可可通过这些道路可以走遍 YY 岛的所有城市。

神奇的是，乘车经过每条道路所需要的费用都是一样的。

小可可，小卡卡和小 YYYY 经常想聚会，每次聚会，他们都会选择一个城市，使得 33 个人到达这个城市的总费用最小。

由于他们计划中还会有很多次聚会，每次都选择一个地点是很烦人的事情，所以他们决定把这件事情交给你来完成。

他们会提供给你地图以及若干次聚会前他们所处的位置，希望你为他们的每一次聚会选择一个合适的地点。

#### 输入格式

第一行两个正整数，NN 和 MM，分别表示城市个数和聚会次数。

后面有 N−1N−1 行，每行用两个正整数 AA 和 BB 表示编号为 AA 和编号为 BB 的城市之间有一条路。

再后面有 MM 行，每行用三个正整数表示一次聚会的情况：小可可所在的城市编号，小卡卡所在的城市编号以及小 YYYY 所在的城市编号。

#### 输出格式

一共有 MM 行，每行两个数 PosPos 和 CostCost，用一个空格隔开，表示第 ii 次聚会的地点选择在编号为 PosPos 的城市，总共的费用是经过 CostCost 条道路所花费的费用。

#### 数据范围

N≤500000,M≤500000N≤500000,M≤500000

#### 输入样例：

    6 4
    1 2
    2 3
    2 4
    4 5
    5 6
    4 5 6
    6 3 1
    2 4 4
    6 6 6
    

#### 输出样例：

    5 2
    2 5
    4 1
    6 0
    

难度：中等

时/空限制：2s / 256MB

总通过数：822

总尝试数：2226

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3942&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3942&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3942&show_algorithm_tags=1)[最近公共祖先](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E8%BF%91%E5%85%AC%E5%85%B1%E7%A5%96%E5%85%88&source_file_id=3942&show_algorithm_tags=1)