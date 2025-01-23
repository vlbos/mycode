119\. 袭击

*    [题目](https://www.acwing.com/problem/content/description/121/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/121/1/)
*    [题解](https://www.acwing.com/problem/content/solution/121/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/121/)

  

在与联盟的战斗中屡战屡败后，帝国撤退到了最后一个据点。

依靠其强大的防御系统，帝国击退了联盟的六波猛烈进攻。

经过几天的苦思冥想，联盟将军亚瑟终于注意到帝国防御系统唯一的弱点就是能源供应。

该系统由 NN 个核电站供应能源，其中任何一个被摧毁都会使防御系统失效。

将军派出了 NN 个特工进入据点之中，打算对能源站展开一次突袭。

不幸的是，由于受到了帝国空军的袭击，他们未能降落在预期位置。

作为一名经验丰富的将军，亚瑟很快意识到他需要重新安排突袭计划。

他现在最想知道的事情就是哪个特工距离其中任意一个发电站的距离最短。

你能帮他算出来这最短的距离是多少吗？

#### 输入格式

输入中包含多组测试用例。

第一行输入整数 TT，代表测试用例的数量。

对于每个测试用例，第一行输入整数 NN。

接下来 NN 行，每行输入两个整数 XX 和 YY，代表每个核电站的位置的 X，YX，Y 坐标。

在接下来 NN 行，每行输入两个整数 XX 和 YY，代表每名特工的位置的 X，YX，Y 坐标。

#### 输出格式

每个测试用例，输出一个最短距离值，结果保留三位小数。

每个输出结果占一行。

#### 数据范围

1≤N≤1000001≤N≤100000,  
0≤X,Y≤10000000000≤X,Y≤1000000000

#### 输入样例：

    2
    4
    0 0
    0 1
    1 0
    1 1
    2 2
    2 3
    3 2
    3 3
    4
    0 0
    0 0
    0 0
    0 0
    0 0
    0 0
    0 0
    0 0
    

#### 输出样例：

    1.414
    0.000
    

难度：中等

时/空限制：5s / 64MB

总通过数：3421

总尝试数：12650

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3670&show_algorithm_tags=0)

算法标签

[分治](https://www.acwing.com/problem/search/1/?search_content=%E5%88%86%E6%B2%BB&source_file_id=3670&show_algorithm_tags=1)