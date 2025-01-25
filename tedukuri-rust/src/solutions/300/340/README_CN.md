340\. 通信线路

*    [题目](https://www.acwing.com/problem/content/description/342/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/342/1/)
*    [题解](https://www.acwing.com/problem/content/solution/342/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/342/)

  

在郊区有 NN 座通信基站，PP 条 **双向** 电缆，第 ii 条电缆连接基站 AiAi 和 BiBi。

特别地，11 号基站是通信公司的总站，NN 号基站位于一座农场中。

现在，农场主希望对通信线路进行升级，其中升级第 ii 条电缆需要花费 LiLi。

电话公司正在举行优惠活动。

农产主可以指定一条从 11 号基站到 NN 号基站的路径，并指定路径上不超过 KK 条电缆，由电话公司免费提供升级服务。

农场主只需要支付在该路径上剩余的电缆中，升级价格最贵的那条电缆的花费即可。

求至少用多少钱可以完成升级。

#### 输入格式

第 11 行：三个整数 N，P，KN，P，K。

第 2..P+12..P+1 行：第 i+1i+1 行包含三个整数 Ai,Bi,LiAi,Bi,Li。

#### 输出格式

包含一个整数表示最少花费。

若 11 号基站与 NN 号基站之间不存在路径，则输出 −1−1。

#### 数据范围

0≤K<N≤10000≤K<N≤1000,  
1≤P≤100001≤P≤10000,  
1≤Li≤10000001≤Li≤1000000

#### 输入样例：

    5 7 1
    1 2 5
    3 1 4
    2 4 8
    3 2 3
    5 2 9
    3 4 7
    4 5 6
    

#### 输出样例：

    4
    

难度：中等

时/空限制：1s / 64MB

总通过数：14563

总尝试数：28461

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3891&show_algorithm_tags=0)[USACO2008](https://www.acwing.com/problem/search/1/?search_content=USACO2008&source_file_id=3891&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3891&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3891&show_algorithm_tags=1)[SPFA](https://www.acwing.com/problem/search/1/?search_content=SPFA&source_file_id=3891&show_algorithm_tags=1)[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3891&show_algorithm_tags=1)[二分](https://www.acwing.com/problem/search/1/?search_content=%E4%BA%8C%E5%88%86&source_file_id=3891&show_algorithm_tags=1)[双端队列BFS](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%8C%E7%AB%AF%E9%98%9F%E5%88%97BFS&source_file_id=3891&show_algorithm_tags=1)