189\. 乳草的入侵

*    [题目](https://www.acwing.com/problem/content/description/191/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/191/1/)
*    [题解](https://www.acwing.com/problem/content/solution/191/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/191/)

  

农民约翰一直努力让他的草地充满鲜美多汁而又健康的牧草。

可惜天不从人愿，他在植物大战人类中败下阵来。

邪恶的乳草已经在他的农场的西北部份占领了一片立足之地。

草地像往常一样，被分割成一个高度为 YY，宽度为 XX 的直角网格。

(1,1)(1,1) 是左下角的格（也就是说坐标排布跟一般的 X,YX,Y 坐标相同）。

乳草一开始占领了格 (Mx,MyMx,My)。

每个星期，乳草传播到已被乳草占领的格子四面八方的每一个没有很多石头的格（包括垂直与水平相邻的和对角线上相邻的格）内。

11 周之后，这些新占领的格又可以把乳草传播到更多的格里面了。

达达想要在草地被乳草完全占领之前尽可能的享用所有的牧草。

她很好奇到底乳草要多久才能占领整个草地。

如果乳草在 00 时刻处于格 (Mx,MyMx,My)，那么几个星期以后它们可以完全占领入侵整片草地呢（对给定的数据总是会发生）？

在草地地图中，`.` 表示草，而 `*` 表示大石。

比如这个 X\=4,Y\=3X\=4,Y\=3 的例子。

    ....
    ..*.
    .**.
    

如果乳草一开始在左下角（第 11 排，第 11 列），那么草地的地图将会以如下态势发展：

          ....  ....  MMM.  MMMM  MMMM  
          ..*.  MM*.  MM*.  MM*M  MM*M  
          M**.  M**.  M**.  M**.  M**M  
    星期数  0     1     2     3     4
    

乳草会在 44 星期后占领整片土地。

#### 输入格式

第 11 行: 四个由空格隔开的整数: XX, YY, MxMx, MyMy

第 22 到第 Y+1Y+1 行: 每行包含一个由 XX 个字符（`.` 表示草地，`*` 表示大石）构成的字符串，共同描绘了草地的完整地图。

#### 输出格式

输出一个整数，表示乳草完全占领草地所需要的星期数。

#### 数据范围

1≤X,Y≤1001≤X,Y≤100

#### 输入样例：

    4 3 1 1
    ....
    ..*.
    .**.
    

#### 输出样例：

    4
    

难度：简单

时/空限制：1s / 64MB

总通过数：1728

总尝试数：5625

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3740&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3740&show_algorithm_tags=1)[广度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%BF%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3740&show_algorithm_tags=1)[BFS](https://www.acwing.com/problem/search/1/?search_content=BFS&source_file_id=3740&show_algorithm_tags=1)