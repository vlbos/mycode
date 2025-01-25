393\. 雇佣收银员

*    [题目](https://www.acwing.com/problem/content/description/395/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/395/1/)
*    [题解](https://www.acwing.com/problem/content/solution/395/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/395/)

  

一家超市要每天 2424 小时营业，为了满足营业需求，需要雇佣一大批收银员。

已知不同时间段需要的收银员数量不同，为了能够雇佣尽可能少的人员，从而减少成本，这家超市的经理请你来帮忙出谋划策。

经理为你提供了一个各个时间段收银员最小需求数量的清单 R(0),R(1),R(2),…,R(23)R(0),R(1),R(2),…,R(23)。

R(0)R(0) 表示午夜 00:0000:00 到凌晨 01:0001:00 的最小需求数量，R(1)R(1) 表示凌晨 01:0001:00 到凌晨 02:0002:00 的最小需求数量，以此类推。

一共有 NN 个合格的申请人申请岗位，第 ii 个申请人可以从 titi 时刻开始连续工作 88 小时。

收银员之间不存在替换，一定会完整地工作 88 小时，收银台的数量一定足够。

现在给定你收银员的需求清单，请你计算最少需要雇佣多少名收银员。

#### 输入格式

第一行包含一个不超过 2020 的整数，表示测试数据的组数。

对于每组测试数据，第一行包含 2424 个整数，分别表示 R(0),R(1),R(2),…,R(23)R(0),R(1),R(2),…,R(23)。

第二行包含整数 NN。

接下来 NN 行，每行包含一个整数 titi。

#### 输出格式

每组数据输出一个结果，每个结果占一行。

如果没有满足需求的安排，输出 `No Solution`。

#### 数据范围

0≤R(0)≤10000≤R(0)≤1000,  
0≤N≤10000≤N≤1000,  
0≤ti≤230≤ti≤23

#### 输入样例：

    1
    1 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1
    5
    0
    23
    22
    1
    10
    

#### 输出样例：

    1
    

难度：中等

时/空限制：1s / 10MB

总通过数：4985

总尝试数：10889

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3944&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3944&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3944&show_algorithm_tags=1)[差分约束](https://www.acwing.com/problem/search/1/?search_content=%E5%B7%AE%E5%88%86%E7%BA%A6%E6%9D%9F&source_file_id=3944&show_algorithm_tags=1)