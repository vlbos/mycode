289\. 环路运输

*    [题目](https://www.acwing.com/problem/content/description/291/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/291/1/)
*    [题解](https://www.acwing.com/problem/content/solution/291/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/291/)

  

在一条环形公路旁均匀地分布着 NN 座仓库，编号为 1∼N1∼N，编号为 ii 的仓库与编号为 jj 的仓库之间的距离定义为 dist(i,j)\=min(|i−j|,N−|i−j|)dist(i,j)\=min⁡(|i−j|,N−|i−j|)，也就是逆时针或顺时针从 ii 到 jj 中较近的一种。

每座仓库都存有货物，其中编号为 ii 的仓库库存量为 AiAi。

在 ii 和 jj 两座仓库之间运送货物需要的代价为 Ai+Aj+dist(i,j)Ai+Aj+dist(i,j)。

求在哪两座仓库之间运送货物需要的代价最大。

#### 输入格式

第一行包含一个整数 NN。

第二行包含 NN 个整数 A1∼ANA1∼AN。

#### 输出格式

输出一个整数，表示最大代价。

#### 数据范围

2≤N≤1062≤N≤106,  
1≤Ai≤1071≤Ai≤107

#### 输入样例：

    5
    1 8 6 2 5
    

#### 输出样例：

    15
    

难度：中等

时/空限制：1s / 64MB

总通过数：2307

总尝试数：4799

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3840&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3840&show_algorithm_tags=1)[环形结构](https://www.acwing.com/problem/search/1/?search_content=%E7%8E%AF%E5%BD%A2%E7%BB%93%E6%9E%84&source_file_id=3840&show_algorithm_tags=1)