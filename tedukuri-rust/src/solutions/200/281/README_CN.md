281\. 硬币

*    [题目](https://www.acwing.com/problem/content/description/283/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/283/1/)
*    [题解](https://www.acwing.com/problem/content/solution/283/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/283/)

  

给定 NN 种硬币，其中第 ii 种硬币的面值为 AiAi，共有 CiCi 个。

从中选出若干个硬币，把面值相加，若结果为 SS，则称“面值 SS 能被拼成”。

求 1∼M1∼M 之间能被拼成的面值有多少个。

#### 输入格式

输入包含多组测试用例。

每组测试用例第一行包含两个整数 NN 和 MM。

第二行包含 2N2N 个整数，分别表示 A1,A2,…,ANA1,A2,…,AN 和 C1,C2,…,CNC1,C2,…,CN。

当输入用例 N\=0，M\=0N\=0，M\=0 时，表示输入终止，且该用例无需处理。

#### 输出格式

每组用例输出一个结果，每个结果占一行。

#### 数据范围

1≤N≤1001≤N≤100,  
1≤M≤1051≤M≤105,  
1≤Ai≤1051≤Ai≤105,  
1≤Ci≤10001≤Ci≤1000

#### 输入用例：

    3 10
    1 2 4 2 1 1
    2 5
    1 4 2 1
    0 0
    

#### 输出用例：

    8
    4
    

难度：困难

时/空限制：1s / 64MB

总通过数：2901

总尝试数：7661

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3832&show_algorithm_tags=0)[POJ1742](https://www.acwing.com/problem/search/1/?search_content=POJ1742&source_file_id=3832&show_algorithm_tags=0)[男人八题](https://www.acwing.com/problem/search/1/?search_content=%E7%94%B7%E4%BA%BA%E5%85%AB%E9%A2%98&source_file_id=3832&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3832&show_algorithm_tags=1)[多重背包](https://www.acwing.com/problem/search/1/?search_content=%E5%A4%9A%E9%87%8D%E8%83%8C%E5%8C%85&source_file_id=3832&show_algorithm_tags=1)